extern crate r2d2;
extern crate r2d2_mongodb;

use crate::model::users::User;
use actix_web::{get, web, App, HttpServer, Responder};
use dotenv::dotenv;
use r2d2::Pool;
use r2d2_mongodb::mongodb::db::ThreadedDatabase;
use r2d2_mongodb::mongodb::*;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};
use std::env;
use std::ops::Deref;
mod model;

struct AppState {
    mongo_conn_pool: r2d2::Pool<r2d2_mongodb::MongodbConnectionManager>,
}

#[get("/admin/users/")]
async fn get_all_users(_info: web::Path<()>, data: web::Data<AppState>) -> impl Responder {
    let connection = &data.mongo_conn_pool.get().unwrap();
    let conn_handle = connection.deref();
    let mut output:Vec<User> = Vec::new();

    let cursor = conn_handle
        .collection("users")
        .find(None, None)
        .unwrap();

        for result in cursor {
            match result {
                Ok(document) => {
                    let res = from_bson(Bson::Document(document)).unwrap();
                    output.push(res);
                }
                Err(_e) => {

                }
            }
        }

    serde_json::to_string(&output).unwrap()
}

#[get("/admin/users/{id}")]
async fn get_user_by_id(info: web::Path<String>, _data: web::Data<AppState>) -> impl Responder {
    format!("User -> {:?}", info)
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

    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host(&mongodb_url, mongodb_port.parse::<u16>().unwrap())
            // .with_ssl(
            //     Some("path/to/ca.crt"),
            //     "path/to/client.crt",
            //     "path/to/client.key",
            //     VerifyPeer::Yes
            // )
            // .with_unauthenticated_ssl(
            //     Some("path/to/ca.crt"),
            //     VerifyPeer::No
            // )
            .with_db(&mongodb_db)
            // .with_auth(&mongodb_uname, &mongodb_password)
            .build(),
    );

    let pool = Pool::builder().max_size(16).build(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                mongo_conn_pool: pool.clone()
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
