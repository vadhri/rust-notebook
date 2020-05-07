#[macro_use]
extern crate diesel;
extern crate diesel_derive_enum;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use schema::actor::dsl::*;

pub fn establish_connection() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let results = actor.limit(5)
        .load::<models::Actor>(&conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{:?}", post);
        println!("----------\n");
    }
}

pub fn main() {
    let conn = establish_connection();
}
