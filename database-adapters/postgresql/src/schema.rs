table! {
    actor (actor_id) {
        actor_id -> Int4,
        first_name -> Text,
        last_name -> Text,
        last_update -> Timestamptz,
    }
}

table! {
    address (address_id) {
        address_id -> Int4,
        #[sql_name = "address"]
        address_name -> Text,
        address2 -> Nullable<Text>,
        district -> Text,
        city_id -> Int4,
        postal_code -> Nullable<Text>,
        phone -> Text,
        last_update -> Timestamptz,
    }
}

table! {
    category (category_id) {
        category_id -> Int4,
        name -> Text,
        last_update -> Timestamptz,
    }
}

table! {
    city (city_id) {
        city_id -> Int4,
        #[sql_name = "city"]
        city_name -> Text,
        country_id -> Int4,
        last_update -> Timestamptz,
    }
}

table! {
    country (country_id) {
        country_id -> Int4,
        #[sql_name = "country"]
        country_name -> Text,
        last_update -> Timestamptz,
    }
}

table! {
    customer (customer_id) {
        customer_id -> Int4,
        store_id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Nullable<Text>,
        address_id -> Int4,
        activebool -> Bool,
        create_date -> Date,
        last_update -> Nullable<Timestamptz>,
        active -> Nullable<Int4>,
    }
}

pub enum MpaaRating {
    G,
    PG,
    PG13,
    R,
    NC17
}

table! {
    film (film_id) {
        film_id -> Int4,
        title -> Text,
        description -> Nullable<Text>,
        release_year -> Nullable<Int4>,
        language_id -> Int4,
        original_language_id -> Nullable<Int4>,
        rental_duration -> Int4,
        rental_rate -> Numeric,
        length -> Nullable<Int4>,
        replacement_cost -> Numeric,
        rating -> crate::schema::MpaaRating,
        last_update -> Timestamptz,
        special_features -> Nullable<Array<Text>>,
        fulltext -> Text,
    }
}

table! {
    film_actor (actor_id, film_id) {
        actor_id -> Int4,
        film_id -> Int4,
        last_update -> Timestamptz,
    }
}

table! {
    film_category (film_id, category_id) {
        film_id -> Int4,
        category_id -> Int4,
        last_update -> Timestamptz,
    }
}

table! {
    inventory (inventory_id) {
        inventory_id -> Int4,
        film_id -> Int4,
        store_id -> Int4,
        last_update -> Timestamptz,
    }
}

table! {
    language (language_id) {
        language_id -> Int4,
        name -> Bpchar,
        last_update -> Timestamptz,
    }
}

table! {
    rental (rental_id) {
        rental_id -> Int4,
        rental_date -> Timestamptz,
        inventory_id -> Int4,
        customer_id -> Int4,
        return_date -> Nullable<Timestamptz>,
        staff_id -> Int4,
        last_update -> Timestamptz,
    }
}

table! {
    staff (staff_id) {
        staff_id -> Int4,
        first_name -> Text,
        last_name -> Text,
        address_id -> Int4,
        email -> Nullable<Text>,
        store_id -> Int4,
        active -> Bool,
        username -> Text,
        password -> Nullable<Text>,
        last_update -> Timestamptz,
        picture -> Nullable<Bytea>,
    }
}

table! {
    store (store_id) {
        store_id -> Int4,
        manager_staff_id -> Int4,
        address_id -> Int4,
        last_update -> Timestamptz,
    }
}

joinable!(address -> city (city_id));
joinable!(city -> country (country_id));
joinable!(customer -> address (address_id));
joinable!(customer -> store (store_id));
joinable!(film_actor -> actor (actor_id));
joinable!(film_actor -> film (film_id));
joinable!(film_category -> category (category_id));
joinable!(film_category -> film (film_id));
joinable!(inventory -> film (film_id));
joinable!(inventory -> store (store_id));
joinable!(rental -> customer (customer_id));
joinable!(rental -> inventory (inventory_id));
joinable!(rental -> staff (staff_id));
joinable!(staff -> address (address_id));
joinable!(staff -> store (store_id));
joinable!(store -> address (address_id));

allow_tables_to_appear_in_same_query!(
    actor,
    address,
    category,
    city,
    country,
    customer,
    film,
    film_actor,
    film_category,
    inventory,
    language,
    rental,
    staff,
    store,
);
