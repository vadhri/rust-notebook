table! {
    actor (actor_id) {
        actor_id -> Integer,
        first_name -> Text,
        last_name -> Text,
        last_update -> Timestamptz,
    }
}

table! {
    address_colname (address_id) {
        address_id -> Integer,
        address -> Text,
        address2 -> Nullable<Text>,
        district -> Text,
        city_id -> Integer,
        postal_code -> Nullable<Text>,
        phone -> Text,
        last_update -> Timestamptz,
    }
}

table! {
    category (category_id) {
        category_id -> Integer,
        name -> Text,
        last_update -> Timestamptz,
    }
}

table! {
    city_colname (city_id) {
        city_id -> Integer,
        city -> Text,
        country_id -> Integer,
        last_update -> Timestamptz,
    }
}

table! {
    country_colname (country_id) {
        country_id -> Integer,
        country -> Text,
        last_update -> Timestamptz,
    }
}

table! {
    customer (customer_id) {
        customer_id -> Integer,
        store_id -> Integer,
        first_name -> Text,
        last_name -> Text,
        email -> Nullable<Text>,
        address_id -> Integer,
        activebool -> Bool,
        create_date -> Date,
        last_update -> Nullable<Timestamptz>,
        active -> Nullable<Integer>,
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
        film_id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        release_year -> Nullable<Integer>,
        language_id -> Integer,
        original_language_id -> Nullable<Integer>,
        rental_duration -> Integer,
        rental_rate -> Numeric,
        length -> Nullable<Integer>,
        replacement_cost -> Numeric,
        rating -> crate::schema::MpaaRating,
        last_update -> Timestamptz,
        special_features -> Nullable<Array<Text>>,
        fulltext -> Text,
    }
}

table! {
    film_actor (actor_id, film_id) {
        actor_id -> Integer,
        film_id -> Integer,
        last_update -> Timestamptz,
    }
}

table! {
    film_category (film_id, category_id) {
        film_id -> Integer,
        category_id -> Integer,
        last_update -> Timestamptz,
    }
}

table! {
    inventory (inventory_id) {
        inventory_id -> Integer,
        film_id -> Integer,
        store_id -> Integer,
        last_update -> Timestamptz,
    }
}

table! {
    language (language_id) {
        language_id -> Integer,
        name -> Bpchar,
        last_update -> Timestamptz,
    }
}

table! {
    rental (rental_id) {
        rental_id -> Integer,
        rental_date -> Timestamptz,
        inventory_id -> Integer,
        customer_id -> Integer,
        return_date -> Nullable<Timestamptz>,
        staff_id -> Integer,
        last_update -> Timestamptz,
    }
}

table! {
    staff (staff_id) {
        staff_id -> Integer,
        first_name -> Text,
        last_name -> Text,
        address_id -> Integer,
        email -> Nullable<Text>,
        store_id -> Integer,
        active -> Bool,
        username -> Text,
        password -> Nullable<Text>,
        last_update -> Timestamptz,
        picture -> Nullable<Bytea>,
    }
}

table! {
    store (store_id) {
        store_id -> Integer,
        manager_staff_id -> Integer,
        address_id -> Integer,
        last_update -> Timestamptz,
    }
}

joinable!(address_colname -> city_colname (city_id));
joinable!(city_colname -> country_colname (country_id));
joinable!(customer -> address_colname (address_id));
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
joinable!(staff -> address_colname (address_id));
joinable!(staff -> store (store_id));
joinable!(store -> address_colname (address_id));

allow_tables_to_appear_in_same_query!(
    actor,
    address_colname,
    category,
    city_colname,
    country_colname,
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
