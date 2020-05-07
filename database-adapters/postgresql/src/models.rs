#[derive(Queryable, Debug)]
pub struct Actor {
    pub actor_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub last_update: diesel::pg::data_types::PgTimestamp
}
