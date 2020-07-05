use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};
use uuid::Uuid;

use warp::{http::Method, Filter, hyper::{StatusCode, Response}};
use futures::executor::block_on;

mod redis_wrapper;
mod topics;
mod common;
mod api_handler;

use common::types::{
    BroadcastMessage,
    Register,
    UnRegister,
    Users
};

use api_handler::handler::{
    broadcast_message,
    broadcast_message_handler,
    register_message_handler,
    unregister_message_handler
};

use api_handler::filters::{
    json_body,
    json_body_register,
    json_body_unregister,
    with_db_multiplexd_aio
};

use redis_wrapper::init::check_key_exists_multiplexed;

pub fn close_running_sockets(users: Users, suuid: String) {
    async fn close_sockets(users: Users, uuid: String) {

        let tx;

        {
            match users.clone().read().await.get(&uuid.clone()) {
                Some(txval) => {
                    tx = txval.clone();
                    let _r = tx.send(Ok(Message::close()));
                },
                None => {

                }
            }
        }
        users.write().await.remove(&uuid);
    }

    block_on(close_sockets(users, suuid));
}

async fn user_connected(uuid_of_user: String, ws: WebSocket, users: Users, rconn: redis::aio::MultiplexedConnection) {
    match check_key_exists_multiplexed(rconn.clone(), &"active".to_string(), &uuid_of_user).await {
        Ok(true) => {},
        _ => {
            ws.close();
            return;
        }
    }

    let (conn_tx, mut conn_rx) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    {
        users.write().await.insert(uuid_of_user.clone(), tx.clone());
    }

    tokio::task::spawn(rx.forward(conn_tx).map(move |result| {
        eprintln!("[Conn task] websocket send error: {:?}", result.unwrap());
    }));

    while let Some(result) = conn_rx.next().await {
        let msg = match result {
            Ok(msg) => {
                if msg == Message::close() {
                    break
                } else {
                    msg
                }
            },
            Err(_e) => {
                eprintln!("websocket receive error ..");
                break;
            }
        };

        broadcast_message(msg, users.clone()).await;
    }

    users.write().await.remove(&uuid_of_user.to_string());
}

#[tokio::main]
async fn main() {
    let client = redis_wrapper::init::initialize_redis("redis://127.0.0.1:6379/".to_string()).unwrap();
    let connection = client.clone().get_connection();
    let mut c = client.get_multiplexed_async_connection().await.unwrap();

    let users = Users::default();
    let users_filterized = warp::any().map(move || users.clone());

    topics::t::get_topics_list(&mut connection.unwrap());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Content-Type",
            "Access-Control-Request-Headers"
        ])
        .allow_methods(vec!["POST"]);

    let broadcast = warp::path("broadcast")
        .and(warp::post())
        .and(users_filterized.clone())
        .and(json_body())
        .and_then(|users: Users, message: BroadcastMessage| {
            broadcast_message_handler(users.clone(), message.text)
    });

    let register = warp::path("register")
        .and(warp::post())
        .and(with_db_multiplexd_aio(c.clone()))
        .and(json_body_register())
        .and_then(|rconn: redis::aio::MultiplexedConnection, message: Register| {
            register_message_handler(message, rconn)
    }).with(cors.clone());

    let unregister = warp::path("unregister")
        .and(warp::post())
        .and(with_db_multiplexd_aio(c.clone()))
        .and(json_body_unregister())
        .and(users_filterized.clone())
        .and_then(|rconn: redis::aio::MultiplexedConnection, u: UnRegister, users: Users| {
          close_running_sockets(users, u.uuid.clone());
          unregister_message_handler(rconn, u)
    }).with(cors.clone());

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_db_multiplexd_aio(c.clone()))
        .and(users_filterized.clone())
        .map(|ws: warp::ws::Ws, id: String, rconn: redis::aio::MultiplexedConnection, users: Users| {
          println!("ws: user connected {:?}", id);
          ws.on_upgrade(move |incoming_websocket| {
              user_connected(id, incoming_websocket, users.clone(), rconn)
          })
    });

    let options_only = warp::options().map(warp::reply).with(cors.clone());

    warp::serve(
        ws_route.or(
            broadcast.or(
                register.or(
                    unregister.or(options_only)))))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
