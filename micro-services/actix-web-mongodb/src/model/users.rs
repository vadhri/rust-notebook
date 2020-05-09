use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing
    pub id: bson::oid::ObjectId,
    given_name: String,
    last_name: String,
    email: String,
    city: String,
    pincode: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRecord {
    pub given_name: String,
    pub last_name: String,
    pub email: String,
    pub city: String,
    pub pincode: String
}
