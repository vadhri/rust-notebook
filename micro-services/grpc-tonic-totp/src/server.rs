use google_authenticator::{
    GoogleAuthenticator,
    ErrorCorrectionLevel
};

use tonic::{transport::Server, Request, Response, Status};

use register_and_identify::validate_totp_server::{ValidateTotp, ValidateTotpServer};
use register_and_identify::{Totp, AuthenticationResult, User, RegistrationResult};
use mongodb::{Client, options::ClientOptions, Collection};
use mongodb::bson::doc;

pub mod register_and_identify {
    tonic::include_proto!("register_and_identify");
}

#[derive(Debug)]
pub struct RegisterAndIdentify {
    db_conn: Collection
}

#[tonic::async_trait]
impl ValidateTotp for RegisterAndIdentify {
    async fn validate(
        &self,
        request: Request<Totp>,
    ) -> Result<Response<AuthenticationResult>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = register_and_identify::AuthenticationResult {
            res: 2
        };

        Ok(Response::new(reply))
    }

    async fn register(
        &self,
        request: Request<User>,
    ) -> Result<Response<RegistrationResult>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let req = request.into_inner();
        let auth = GoogleAuthenticator::new();
        let a = auth.create_secret(32);
        let qr_code_url = auth.qr_code_url(&a, "QR", "TestService", 200, 200, ErrorCorrectionLevel::High);

        let result = self.db_conn.insert_one(doc! { 
            "firstname": req.firstname, 
            "lastname": req.lastname,
            "_id": req.email,
            "secret": a.clone(),
            "qr_code": qr_code_url.clone()
        }, None).await;

        match result {
            Ok(_res) => {
                let reply = register_and_identify::RegistrationResult {
                    res: 0,
                    qr_code: qr_code_url
                };

                Ok(Response::new(reply))
            }, 
            Err(_code) => {
                let reply = register_and_identify::RegistrationResult {
                    res: 1,
                    qr_code: "".to_string()
                };

                Ok(Response::new(reply))                
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:8089".parse().unwrap();
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    let client = Client::with_options(client_options)?;
    let db = client.database("service");

    let collection = db.collection("users");

    let vtotp = RegisterAndIdentify {
        db_conn: collection
    };

    Server::builder()
        .add_service(ValidateTotpServer::new(vtotp))
        .serve(addr)
        .await?;

    Ok(())
}