extern crate r2d2;
extern crate r2d2_mongodb;

use crate::model::users::UserRecord;
use actix_web::Error;
use bson::from_bson;
use crate::model::users::User;
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web::http::{StatusCode};
use dotenv::dotenv;
use mongodb::options::*;
use mongodb::*;
use bson::Bson;

use bson::doc;
use std::env;
mod model;
use actix_files as fs;

#[derive(Debug)]
struct AppState {
    mongo_conn_pool: mongodb::Database,
}

async fn delete_record(info: web::Path<String>, data: web::Data<AppState>) -> HttpResponse  {
    let doc_to_delete = doc! {
        "_id":  bson::oid::ObjectId::with_string(&info.to_string()).unwrap()
    };

    match data.mongo_conn_pool.collection("users").delete_one(doc_to_delete, None) {
        Ok(_value) => {
            if _value.deleted_count > 0 {
            HttpResponse::build(StatusCode::OK)
                .content_type("application/json")
                .body("")
            } else {
                HttpResponse::build(StatusCode::NOT_FOUND)
                    .content_type("application/json")
                    .body("Record not found.")
            }
        }, Err(reason) => {
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json")
                .body(reason.to_string())
        }
    }
}


async fn update_record(info: web::Path<String>, item: web::Json<UserRecord>, data: web::Data<AppState>) -> HttpResponse  {
    let doc_to_update = doc! {
        "_id":  bson::oid::ObjectId::with_string(&info.to_string()).unwrap()
    };

    let record = doc! {
        "given_name": item.0.given_name,
        "last_name": item.0.last_name,
        "city": item.0.city,
        "email": item.0.email,
        "pincode": item.0.pincode,
    };

    match data.mongo_conn_pool.collection("users").update_one(doc_to_update, record, None) {
        Ok(_value) => {
            if _value.modified_count > 0 {
            HttpResponse::build(StatusCode::OK)
                .content_type("application/json")
                .body("")
            } else {
                HttpResponse::build(StatusCode::NOT_FOUND)
                    .content_type("application/json")
                    .body("Record not found.")
            }
        }, Err(reason) => {
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json")
                .body(reason.to_string())
        }
    }
}

async fn create_record(item: web::Json<UserRecord>, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let record = doc! {
        "given_name": item.0.given_name,
        "last_name": item.0.last_name,
        "city": item.0.city,
        "email": item.0.email,
        "pincode": item.0.pincode,
    };

    match data.mongo_conn_pool.collection("users").insert_many(vec![record], None) {
        Ok(_result) => {
            Ok(HttpResponse::build(StatusCode::CREATED)
                .content_type("application/json; charset=utf-8")
                .body("".to_string()))
        }, Err(reason) => {
            Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .content_type("application/json; charset=utf-8")
                .body(reason.to_string()))
        }
    }
}

async fn get_all_records(_info: web::Path<()>, data: web::Data<AppState>) -> HttpResponse {
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

    HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .body(serde_json::to_string(&output).unwrap())
}

async fn get_record_by_id(info: web::Path<String>, data: web::Data<AppState>) -> HttpResponse  {
    let mut output: Vec<User> = Vec::new();

    let filter = doc! {
        "_id":  bson::oid::ObjectId::with_string(&info.to_string()).unwrap()
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

    HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .body(serde_json::to_string(&output).unwrap())
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
                fs::Files::new("/static/", "./static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(
                web::resource("/admin/users")
                    .route(web::get().to(get_all_records))
                    .route(web::post().to(create_record))
            )
            .service(
                web::resource("/admin/users/{id}")
                    .route(web::get().to(get_record_by_id))
                    .route(web::delete().to(delete_record))
                    .route(web::put().to(update_record)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
