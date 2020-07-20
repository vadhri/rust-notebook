use google_authenticator::{
    GoogleAuthenticator,
    ErrorCorrectionLevel
};
use jsonwebtoken::errors::ErrorKind;
use tonic::{transport::Server, Request, Response, Status};
use std::time::SystemTime;

use register_and_identify::validate_totp_server::{ValidateTotp, ValidateTotpServer};
use register_and_identify::{Totp, AuthenticationResult, User, RegistrationResult};
use mongodb::{Client, options::ClientOptions, Collection};
use mongodb::{
    bson::{doc, Bson},
    options::FindOptions,
};

pub mod register_and_identify {
    tonic::include_proto!("register_and_identify");
}

use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    firstname: String,
    lastname: String,
    email: String,
    sub: String,
    company: String,
    exp: usize
}

#[derive(Debug)]
pub struct RegisterAndIdentify {
    db_conn: Collection,
    token_secret: String
}

#[tonic::async_trait]
impl ValidateTotp for RegisterAndIdentify {
    async fn validate(
        &self,
        request: Request<Totp>,
    ) -> Result<Response<AuthenticationResult>, Status> {
        let req_clone = request.into_inner().clone();
            println!("Got a request from {:?}", req_clone.token.clone());

        println!("{:?}", self.token_secret.clone());

        let token = decode::<Claims>(&req_clone.token.clone(), 
            &DecodingKey::from_secret(self.token_secret.as_ref()), &Validation::default());

        match token {
            Ok(result) => {
                println!("user details = {:?}", result.claims.email);

                // Query the documents in the collection with a filter and an option.
                let filter = doc! { "_id": result.claims.email };
                let mut cursor = self.db_conn.find_one(filter, None).await.unwrap();

                if let Some(secret) = cursor.unwrap().get("secret").and_then(Bson::as_str) {
                    let auth = GoogleAuthenticator::new();

                    if auth.verify_code(secret, req_clone.input.as_str(), 1, 0) {
                        let reply = register_and_identify::AuthenticationResult {
                            res: 1
                        };

                        Ok(Response::new(reply))  
                    } else {
                        let reply = register_and_identify::AuthenticationResult {
                            res: 2
                        };

                        Ok(Response::new(reply))  
                    }
                }  else {
                    let reply = register_and_identify::AuthenticationResult {
                        res: 3
                    };

                    Ok(Response::new(reply))  

                }
            }, _ => {

                let reply = register_and_identify::AuthenticationResult {
                    res: 3
                };

                Ok(Response::new(reply))  
            }
        }
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
            "firstname": req.firstname.clone(), 
            "lastname": req.lastname.clone(),
            "_id": req.email.clone(),
            "secret": a.clone(),
            "qr_code": qr_code_url.clone()
        }, None).await;

        let claim = Claims {
            firstname: req.firstname,
            lastname: req.lastname,
            email: req.email,
            exp: 10000000000,
            company: "ACME".to_owned(),
            sub: "Subject".to_owned()
        };

        let token = encode(&Header::default(), &claim, &EncodingKey::from_secret(self.token_secret.as_ref())).unwrap();

        match result {
            Ok(_res) => {
                let reply = register_and_identify::RegistrationResult {
                    res: 0,
                    qr_code: qr_code_url,
                    token: token
                };

                Ok(Response::new(reply))
            }, 
            Err(_code) => {
                let reply = register_and_identify::RegistrationResult {
                    res: 1,
                    qr_code: "".to_string(),
                    token: "".to_string()
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
    let token_secret = "secret";

    let collection = db.collection("users");

    let vtotp = RegisterAndIdentify {
        db_conn: collection,
        token_secret: token_secret.to_string()
    };

    Server::builder()
        .add_service(ValidateTotpServer::new(vtotp))
        .serve(addr)
        .await?;

    Ok(())
}