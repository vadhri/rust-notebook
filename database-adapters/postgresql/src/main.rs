#[macro_use]
extern crate diesel;
extern crate diesel_derive_enum;
extern crate dotenv;

use std::ops::Deref;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

extern crate r2d2;
extern crate r2d2_diesel;
use std::thread;

use r2d2_diesel::ConnectionManager;

pub mod models;
pub mod schema;

use std::panic;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::time::Duration;

use schema::actor::dsl::*;
use schema::address::dsl::*;
use schema::category::dsl::*;
use schema::city::dsl::*;
use schema::country::dsl::*;
use schema::customer::dsl::*;
use schema::film::dsl::*;
use schema::film_actor::dsl::*;
use schema::film_category::dsl::*;
use schema::inventory::dsl::*;
use schema::language::dsl::*;
use schema::rental::dsl::*;
use schema::staff::dsl::*;
use schema::store::dsl::*;

pub fn establish_connection(conn: &PgConnection) {

    let results = actor.limit(50).load::<models::Actor>(conn).expect("Error loading actors");

    println!("Displaying {} actors", results.len());
    for post in results { println!("{:?}", post); }
    println!("----------\n");

    let results = address.limit(50).load::<models::Address>(conn).expect("Error loading address");

    println!("Displaying {} address", results.len());
    for post in results { println!("{:?}", post); }
    println!("----------\n");

    let results = category.limit(50).load::<models::Category>(conn).expect("Error loading category");

    println!("Displaying {} categories .. ", results.len());
    for post in results { println!("{:?}", post);}
    println!("----------\n");

    let results = city.limit(50).load::<models::City>(conn).expect("Error loading posts");

    println!("Displaying {} cities .. ", results.len());
    for post in results { println!("{:?}", post);}
    println!("----------\n");

    let results = country.limit(50).load::<models::Country>(conn).expect("Error loading posts");

    println!("Displaying {} country .. ", results.len());
    for post in results { println!("{:?}", post); }
    println!("----------\n");

    let results = customer.limit(50).load::<models::Customer>(conn).expect("Error loading posts");

    println!("Displaying {} customer .. ", results.len());
    for post in results { println!("{:?}", post); }
    println!("----------\n");

    let results = film.limit(50).load::<models::Film>(conn).expect("Error loading posts");
    println!("Displaying {} films .. ", results.len());
    for post in results { println!("{:?}", post); }
    println!("----------\n");

    let results = film_actor.limit(50).load::<models::FilmActor>(conn).expect("Error loading posts");
    println!("Displaying {} film actors  .. ", results.len());
    for post in results { println!("{:?}", post); }
    println!("----------\n");

    let results = film_category.limit(50).load::<models::FilmCategory>(conn).expect("Error loading posts");
    println!("Displaying {} film categories .. ", results.len());
    for post in results { println!("{:?}", post); }
    println!("----------\n");

    let results = inventory.limit(50).load::<models::Inventory>(conn).expect("Error loading posts");
    println!("Displaying {} store inventories .. ", results.len());
    for post in results { println!("{:?}", post); }
    println!("----------\n");

    let results = language.limit(50).load::<models::Language>(conn).expect("Error loading posts");
    println!("Displaying {} languages .. ", results.len());
    for post in results { println!("{:?}", post); }
    println!("----------\n");

    let results = rental.limit(50).load::<models::Rental>(conn).expect("Error loading posts");
    println!("Displaying {} rental records .. ", results.len());
    for post in results { println!("{:?}", post); }
    println!("----------\n");

    let results = staff.limit(50).load::<models::Staff>(conn).expect("Error loading posts");
    println!("Displaying {} Staff details.. ", results.len());
    for post in results { println!("{:?}", post); }
    println!("----------\n");

    let results = store.limit(50).load::<models::Store>(conn).expect("Error loading posts");
    println!("Displaying {} Stores .. ", results.len());
    for post in results { println!("{:?}", post); }
    println!("----------\n");
}

static GLOBAL_THREAD_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in the .env file.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    let mut vector = Vec::new();

    for _ in 0..1 {
        let pool = pool.clone();

        let t1 = thread::spawn(move || {
            let connection = pool.get();
            GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);

            if connection.is_ok() {
                establish_connection(connection.unwrap().deref());
            }

            GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
        });

        vector.push(t1);
    }

    // Wait for other threads to finish.
    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(1));
    }

    // Give some time for writes to finish otherwise, it would close the app without writing to stdout.
    thread::sleep(Duration::from_millis(1000));
}
