use warp::ws::Message;
use std::{ptr::null, convert::Infallible};
use redis::Connection;

use crate::common::types::{
    Users,
    BroadcastMessageResponse,
    Register,
    RegisterMessageResponse,
    RegisterErrorResponse
};
use uuid::Uuid;

use crate::redis_wrapper::init::{
    write_hashmap_key,
    read_hashmap_key,
    check_key_exists,
    check_key_exists_multiplexed,
    write_hashmap_key_multiplexed
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

pub async fn register_message_handler(register: Register, rconn: redis::aio::MultiplexedConnection) -> Result<impl warp::Reply, Infallible> {
    let uuid_of_user = Uuid::new_v4();

    let check_user = check_key_exists_multiplexed(rconn.clone(), &"users".to_string(), &register.name).await;

    match check_user {
        Ok(false) => {
            let _res = write_hashmap_key_multiplexed(rconn.clone(), "users".to_string(), register.name, uuid_of_user.to_string()).await;
            let _res = write_hashmap_key_multiplexed(rconn.clone(), "active".to_string(), uuid_of_user.to_string(), "enabled".to_string()).await;

            let resp = warp::reply::json(&RegisterMessageResponse {
                code: 0,
                uuid: uuid_of_user.to_string(),
                reason: format!("Registered user successfully!")
            });

            Ok(resp)
        },
        _ => {
            let resp = warp::reply::json(&RegisterErrorResponse {
                code: 1,
                reason: format!("User already exists!")
            });

            Ok(resp)
        }
    }

}
