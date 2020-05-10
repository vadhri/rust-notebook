extern crate r2d2;
extern crate r2d2_mongodb;

use actix::prelude::*;

use crate::model::users::UserRecord;
use crate::model::users::UserReturned;

use actix_web::Error;
use bson::from_bson;
use crate::model::users::User;
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer};

use actix_web::http::{StatusCode};
use dotenv::dotenv;
use mongodb::options::*;
use mongodb::*;
use bson::Bson;
use actix_cors::Cors;
use qstring::QString;

use bson::doc;
use std::env;
mod model;
use actix_files as fs;

#[derive(Debug)]
struct AppState {
    mongo_conn_pool: mongodb::Database,
}

struct State {
    db: Addr<AppState>,
    s: AppState
}

impl Actor for AppState {
    type Context = SyncContext<Self>;
}

impl Message for UserReturned {
    type Result = Option<UserReturned>;
}

impl Handler<UserReturned> for AppState {
    type Result = Option<UserReturned>;

    fn handle(&mut self, msg: UserReturned, _: &mut Self::Context) -> Self::Result {
        println!("Create user -> {:?}", msg);
        let new_user = doc! {
            "given_name": &msg.given_name,
            "last_name": &msg.last_name,
            "city": &msg.city,
            "email": &msg.email,
            "pincode": &msg.pincode
        };

        let res = self.mongo_conn_pool.collection("users").insert_one(new_user, None);

        match res.unwrap().inserted_id {
            Bson::ObjectId(objectid) => {
                Some(UserReturned {
                    id: objectid.to_string(),
                    given_name: msg.given_name,
                    last_name: msg.last_name,
                    city: msg.city,
                    email: msg.email,
                    pincode: msg.pincode
                })
            },
            _rest => {
                None
            }
        }
    }
}

async fn delete_record(info: web::Path<String>, data: web::Data<State>) -> HttpResponse  {
    let doc_to_delete = doc! {
        "_id":  bson::oid::ObjectId::with_string(&info.to_string()).unwrap()
    };

    match data.s.mongo_conn_pool.collection("users").delete_one(doc_to_delete, None) {
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


async fn update_record(info: web::Path<String>, item: web::Json<UserRecord>, data: web::Data<State>) -> HttpResponse  {
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

    match data.s.mongo_conn_pool.collection("users").update_one(doc_to_update, record, None) {
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

async fn create_record(item: web::Json<UserRecord>, data: web::Data<State>) -> Result<HttpResponse, Error> {
    match data.db.send(UserReturned {
            id: "".to_string(),
            given_name: item.0.given_name,
            last_name: item.0.last_name,
            city: item.0.city,
            email: item.0.email,
            pincode: item.0.pincode,
        }).await {
            Ok(result) => {
                Ok(HttpResponse::build(StatusCode::CREATED)
                    .content_type("application/json; charset=utf-8")
                    .body(serde_json::to_string(&result).unwrap()))
            }, Err(reason) => {
                Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .content_type("application/json; charset=utf-8")
                    .body(reason.to_string()))
            }
        }
}

async fn get_all_records(req: web::HttpRequest, _info: web::Path<()>, data: web::Data<State>) -> HttpResponse {
    let mut output: Vec<UserReturned> = Vec::new();
    let length = data.s.mongo_conn_pool.collection("users").count_documents(None, None).unwrap();

    println!("get_all_records -> Total documents = {:?}, _info.query = {:?}", length, req.query_string());
    let mut filter = doc! {

    };

    let mut options = FindOptions::builder().build();
    let mut start:i64 = 0;
    // _end=10&_order=ASC&_sort=id&_start=0

    let qs = QString::from(req.query_string());
    match qs.get("id") {
        Some(id) => {
            filter.insert("_id", bson::oid::ObjectId::with_string(id).unwrap());
        }, None => {

        }
    };

    match qs.get("_start") {
        Some(_start) => {
            start = _start.parse::<i64>().unwrap();
            options.skip = Some(start);
        }, None => {

        }
    };

    match qs.get("_end") {
        Some(_end) => {
            let limit = _end.parse::<i64>().unwrap() - start;
            options.limit = Some(limit);
        }, None => {

        }
    };

    let cursor = data.s.mongo_conn_pool.collection("users").find(Some(filter), Some(options)).unwrap();

    for result in cursor {
        match result {
            Ok(document) => {
                let res: User = from_bson(Bson::Document(document)).unwrap();

                output.push(UserReturned {
                    id: res.id.to_string(),
                    given_name: res.given_name,
                    last_name: res.last_name,
                    email: res.email,
                    city: res.city,
                    pincode: res.pincode
                });

            }
            Err(_e) => {}
        }
    }

    HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .set_header("Access-Control-Expose-Headers", "X-Total-Count")
        .set_header("X-Total-Count", length.to_string())
        .body(serde_json::to_string(&output).unwrap())
}

async fn get_record_by_id(info: web::Path<String>, data: web::Data<State>) -> HttpResponse  {
    let mut output: Vec<User> = Vec::new();

    println!("get_record_by_id -> {:?}", info);

    let filter = doc! {
        "_id":  bson::oid::ObjectId::with_string(&info.to_string()).unwrap()
    };

    let c = data.s.mongo_conn_pool.collection("users").find(Some(filter), None).unwrap();

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

fn main() {
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

    let sys = actix::System::new("diesel-example");

    // Start 3 parallel db executors
    let addr = SyncArbiter::start(3, move || {
        AppState {
            mongo_conn_pool: client.database(&mongodb_db)
        }
    });

    HttpServer::new(move || {
        App::new()
        .wrap(
           Cors::new()
             .allowed_header(http::header::CONTENT_TYPE)
             .allowed_header(http::header::ACCESS_CONTROL_EXPOSE_HEADERS)
             .max_age(3600)
             .finish())
            .data(State { db: addr.clone(), s: AppState {
                mongo_conn_pool: db.clone()
            }})
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
    .bind("127.0.0.1:8081")
    .unwrap()
    .run();

    println!("Started http server: 127.0.0.1:8081");
    let _ = sys.run();
}
