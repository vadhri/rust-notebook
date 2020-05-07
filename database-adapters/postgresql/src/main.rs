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
use schema::address::dsl::*;
use schema::category::dsl::*;
use schema::city::dsl::*;
use schema::country::dsl::*;
use schema::customer::dsl::*;

pub fn establish_connection() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let results = actor.limit(5)
        .load::<models::Actor>(&conn)
        .expect("Error loading posts");

    println!("Displaying {} actors", results.len());
    for post in results {
        println!("{:?}", post);
    }
    println!("----------\n");

    let results = address.limit(5)
        .load::<models::Address>(&conn)
        .expect("Error loading posts");

    println!("Displaying {} address", results.len());
    for post in results {
        println!("{:?}", post);
    }
    println!("----------\n");

    let results = category.limit(5)
        .load::<models::Category>(&conn)
        .expect("Error loading posts");

    println!("Displaying {} categories .. ", results.len());
    for post in results {
        println!("{:?}", post);
    }
    println!("----------\n");

    let results = city.limit(5)
        .load::<models::City>(&conn)
        .expect("Error loading posts");

    println!("Displaying {} cities .. ", results.len());
    for post in results {
        println!("{:?}", post);
    }

    println!("----------\n");

    let results = country.limit(5)
        .load::<models::Country>(&conn)
        .expect("Error loading posts");

    println!("Displaying {} country .. ", results.len());
    for post in results {
        println!("{:?}", post);
    }

    println!("----------\n");

    let results = customer.limit(5)
        .load::<models::Customer>(&conn)
        .expect("Error loading posts");

    println!("Displaying {} customer .. ", results.len());
    for post in results {
        println!("{:?}", post);
    }

    println!("----------\n");

}

pub fn main() {
    let conn = establish_connection();
}
