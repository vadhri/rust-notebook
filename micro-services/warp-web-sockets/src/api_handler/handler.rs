use warp::ws::Message;
use std::convert::Infallible;

use crate::common::types::{
    Users,
    BroadcastMessageResponse,
    Register,
    UnRegister,
    RegisterMessageResponse,
    RegisterErrorResponse,
    UnRegisterMessageResponse
};
use uuid::Uuid;

use crate::redis_wrapper::init::{
    check_key_exists_multiplexed,
    write_hashmap_key_multiplexed,
    del_hashmap_key_multiplexed
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

            Ok(warp::reply::with_status(resp, warp::http::StatusCode::CREATED))
        },
        _ => {
            let resp = warp::reply::json(&RegisterErrorResponse {
                code: 1,
                reason: format!("User already exists!")
            });

            Ok(warp::reply::with_status(resp, warp::http::StatusCode::CONFLICT))
        }
    }
}

pub async fn unregister_message_handler(rconn: redis::aio::MultiplexedConnection, u: UnRegister) -> Result<impl warp::Reply, Infallible> {
    let check_user = check_key_exists_multiplexed(rconn.clone(), &"users".to_string(), &u.name).await;

    match check_user {
        Ok(false) => {
            let response_message = warp::reply::json(&UnRegisterMessageResponse {
                code: 0,
                reason: format!("User not found!")
            });

            Ok(warp::reply::with_status(response_message, warp::http::StatusCode::CONFLICT))
        },
        _ => {
            del_hashmap_key_multiplexed(rconn.clone(), "users".to_string(), u.name).await;
            del_hashmap_key_multiplexed(rconn.clone(), "active".to_string(), u.uuid.to_string()).await;

            //println!("Delete uuid : {:?}", u.uuid.to_string());

            let response_message = warp::reply::json(&RegisterErrorResponse {
                code: 0,
                reason: format!("User deleted exists!")
            });

            Ok(warp::reply::with_status(response_message, warp::http::StatusCode::OK))
        }
    }
}
