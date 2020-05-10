use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    // #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing
    #[serde(rename(serialize = "id", deserialize = "_id"))]
    pub id: bson::oid::ObjectId,
    pub given_name: String,
    pub last_name: String,
    pub email: String,
    pub city: String,
    pub pincode: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserReturned {
    // #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing
    #[serde(rename(serialize = "id", deserialize = "_id"))]
    pub id: String,
    pub given_name: String,
    pub last_name: String,
    pub email: String,
    pub city: String,
    pub pincode: String
}

/* When information is sent from external clients, id field will not be present.
The structure will mitigate that need. */

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRecord {
    pub given_name: String,
    pub last_name: String,
    pub email: String,
    pub city: String,
    pub pincode: String
}
