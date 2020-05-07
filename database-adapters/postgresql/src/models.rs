#[derive(Queryable, Debug)]
pub struct Actor {
    pub actor_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub last_update: diesel::pg::data_types::PgTimestamp
}

#[derive(Queryable, Debug)]
pub struct Address {
    pub address_id: i32,
    pub address: String,
    pub address2: Option<String>,
    pub district: String,
    pub city_id: i32,
    pub postal_code: Option<String>,
    pub phone: String,
    pub last_update: diesel::pg::data_types::PgTimestamp
}

#[derive(Queryable, Debug)]
pub struct Category {
    pub category_id: i32,
    pub name: String,
    pub last_update: diesel::pg::data_types::PgTimestamp
}

#[derive(Queryable, Debug)]
pub struct City {
    pub city_id: i32,
    pub city_name: String,
    pub country_id: i32,
    pub last_update: diesel::pg::data_types::PgTimestamp
}

#[derive(Queryable, Debug)]
pub struct Country {
    pub country_id: i32,
    pub country_name: String,
    pub last_update: diesel::pg::data_types::PgTimestamp
}

#[derive(Queryable, Debug)]
pub struct Customer {
    pub customer_id: i32,
    pub store_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub address_id: i32,
    pub activebool: bool,
    pub create_date: diesel::pg::data_types::PgDate,
    pub last_update: Option<diesel::pg::data_types::PgTimestamp>,
    pub active: Option<i32>
}

use diesel_derive_enum::DbEnum;

#[derive(Debug, DbEnum)]
pub enum MyEnum {
    #[db_rename = "G"]
    G,
    #[db_rename = "PG"]
    PG,
    #[db_rename = "NC-17"]
    NC17
}

#[derive(Queryable, Debug)]
pub struct Film {
    pub film_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub release_year: Option<i32>,
    pub language_id: i32,
    pub original_language_id: Option<i32>,
    pub rental_duration: i32,
    pub rental_rate: diesel::pg::data_types::PgNumeric,
    pub length: Option<i32>,
    pub replacement_cost: diesel::pg::data_types::PgNumeric,
    pub rating: MyEnum,
    pub last_update: diesel::pg::data_types::PgTimestamp,
    pub special_features: Option<Vec<String>>,
    pub fulltext: String
}

#[derive(Queryable, Debug)]
pub struct FilmActor {
    pub actor_id: i32,
    pub film_id: i32,
    pub last_update: diesel::pg::data_types::PgTimestamp
}

#[derive(Queryable, Debug)]
pub struct FilmCategory {
    pub film_id: i32,
    pub category_id: i32,
    pub last_update: diesel::pg::data_types::PgTimestamp
}

#[derive(Queryable, Debug)]
pub struct Inventory {
    pub inventory_id: i32,
    pub film_id: i32,
    pub store_id: i32,
    pub last_update: diesel::pg::data_types::PgTimestamp
}

#[derive(Queryable, Debug)]
pub struct Language {
    pub language_id: i32,
    pub name: String,
    pub last_update: diesel::pg::data_types::PgTimestamp
}

#[derive(Queryable, Debug)]
pub struct Rental {
    pub rental_id: i32,
    pub rental_date: diesel::pg::data_types::PgTimestamp,
    pub inventory_id: i32,
    pub customer_id: i32,
    pub return_date: Option<diesel::pg::data_types::PgTimestamp>,
    pub staff_id: i32,
    pub last_update: diesel::pg::data_types::PgTimestamp
}

#[derive(Queryable, Debug)]
pub struct Staff {
    pub staff_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub address_id: i32,
    pub email: Option<String>, 
    pub store_id: i32,
    pub active: bool,
    pub username: String,
    pub password: Option<String>,
    pub last_update: diesel::pg::data_types::PgTimestamp,
    pub picture: Option<Vec<u8>>,
}

#[derive(Queryable, Debug)]
pub struct Store {
    pub store_id: i32,
    pub manager_staff_id: i32,
    pub address_id: i32,
    pub last_update: diesel::pg::data_types::PgTimestamp
}
