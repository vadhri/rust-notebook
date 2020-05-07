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
