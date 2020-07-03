use warp::ws::Message;
use std::convert::Infallible;
use redis::Connection;

use crate::common::types::{
    Users,
    BroadcastMessageResponse,
    Register,
    RegisterMessageResponse
};
use uuid::Uuid;

use crate::redis_wrapper::init::{
    write_hashmap_key,
    read_hashmap_key
};

pub async fn broadcast_message(msg: Message, users: Users) {
    for user in users.read().await.iter() {
        let _res = user.1.send(Ok(msg.clone()));
    }
}

pub async fn broadcast_message_handler(users: Users, msg: String) -> Result<impl warp::Reply, Infallible> {
    broadcast_message(Message::text(msg), users.clone()).await;

    Ok(warp::reply::json(&BroadcastMessageResponse {
        code: 0,
        reason: format!("Message broadcast to {:?} users.", users.read().await.len())
    }))
}

pub async fn register_message_handler(register: Register, rconn: redis::Client) -> Result<impl warp::Reply, Infallible> {
    let mut connection = rconn.get_connection().unwrap();
    let uuid_of_user = Uuid::new_v4();

    let _res = write_hashmap_key(&mut connection, &"users".to_string(), &register.name, &uuid_of_user.to_string());

    let resp = warp::reply::json(&RegisterMessageResponse {
        code: 0,
        uuid: uuid_of_user.to_string(),
        reason: format!("Registered user successfully!")
    });

    Ok(resp)
}
