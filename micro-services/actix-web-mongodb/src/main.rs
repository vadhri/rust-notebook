extern crate r2d2;
extern crate r2d2_mongodb;

use bson::from_bson;
use crate::model::users::User;
use actix_web::{get, web, App, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::options::*;
use mongodb::*;
use bson::Bson;

use bson::doc;
use std::env;
mod model;

struct AppState {
    mongo_conn_pool: mongodb::Database,
}

#[get("/admin/users/")]
async fn get_all_users(_info: web::Path<()>, data: web::Data<AppState>) -> impl Responder {
    let mut output: Vec<User> = Vec::new();
    let cursor = data.mongo_conn_pool.collection("users").find(None, None).unwrap();

    for result in cursor {
        match result {
            Ok(document) => {
                let res = from_bson(Bson::Document(document)).unwrap();
                output.push(res);
            }
            Err(_e) => {}
        }
    }

    serde_json::to_string(&output).unwrap()
}

#[get("/admin/users/{id}")]
async fn get_user_by_id(info: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut output: Vec<User> = Vec::new();

    let filter = doc! {
        "_id":  bson::oid::ObjectId::with_string(&info).unwrap()
    };

    let c = data.mongo_conn_pool.collection("users").find(Some(filter), None).unwrap();

    for row in c {
        match row {
            Ok(document) => {
                let res = from_bson(Bson::Document(document)).unwrap();
                output.push(res);
            }
            Err(_e) => {}
        }
    }
    serde_json::to_string(&output).unwrap()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mongodb_url = env::var("DB_HOST").expect("DB host must be set in the .env file.");
    let mongodb_port = env::var("DB_PORT").expect("DB port must be set in the .env file.");
    let mongodb_db = env::var("MONGO_DB").expect("MONGO_DB must be set in the .env file.");
    let _mongodb_uname = env::var("MONGO_USER").expect("MONGO_USER must be set in the .env file.");
    let _mongodb_password =
        env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD must be set in the .env file.");

    let options = ClientOptions::builder()
        .hosts(vec![StreamAddress {
            hostname: mongodb_url,
            port: Some(mongodb_port.parse::<u16>().unwrap()),
        }])
        .build();

    let client = Client::with_options(options).unwrap();

    let db = client.database(&mongodb_db);

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                mongo_conn_pool: db.clone(),
            })
            .service(
                web::scope("/admin/")
                    .service(get_all_users)
                    .service(get_user_by_id),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
