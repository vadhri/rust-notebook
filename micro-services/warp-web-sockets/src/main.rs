use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};
use uuid::Uuid;

use warp::{http::Method, Filter, hyper::{StatusCode, Response}};

mod redis_wrapper;
mod topics;
mod common;
mod api_handler;

use common::types::{
    BroadcastMessage,
    Register,
    Users
};

use api_handler::handler::{
    broadcast_message,
    broadcast_message_handler,
    register_message_handler
};

use api_handler::filters::{
    json_body,
    json_body_register,
    with_db
};

async fn user_connected(ws: WebSocket, users: Users) {
    // Split the websocket into rx and tx streams.
    let (conn_tx, mut conn_rx) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel();
    let uuid_of_user = Uuid::new_v4();

    users.write().await.insert(uuid_of_user, tx.clone());

    tokio::task::spawn(rx.forward(conn_tx).map(move |result| {
        let user = uuid_of_user.clone();
        if let Err(e) = result {
            eprintln!("[Conn task] websocket send error: {} {}", e, user);
        } else {
            eprintln!("[Conn task] Websocket forward successful. {}", user);
        }
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

    users.write().await.remove(&uuid_of_user);
}

#[tokio::main]
async fn main() {
    let client = redis_wrapper::init::initialize_redis("redis://127.0.0.1:6379/".to_string()).unwrap();
    let connection = client.clone().get_connection();
    let users = Users::default();
    let users_filterized = warp::any().map(move || users.clone());

    topics::t::get_topics_list(&mut connection.unwrap());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent",
        "Sec-Fetch-Mode",
        "Referer",
        "Origin",
        "Access-Control-Request-Method",
        "Content-Type",
        "Access-Control-Request-Headers"])
        .allow_methods(vec!["POST"]);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(users_filterized.clone())
        .map(|ws: warp::ws::Ws, users: Users| {
          ws.on_upgrade(move |incoming_websocket| {
              user_connected(incoming_websocket, users.clone())
          })
    });

    let broadcast = warp::path("broadcast")
        .and(warp::post())
        .and(users_filterized)
        .and(json_body())
        .and_then(|users: Users, message: BroadcastMessage| {
            broadcast_message_handler(users.clone(), message.text)
    });

    let register = warp::path("register")
        .and(warp::post())
        .and(with_db(client.clone()))
        .and(json_body_register())
        .and_then(|rconn: redis::Client, message: Register| {
            register_message_handler(message, rconn)
    }).with(cors.clone());

    let options_only = warp::options().map(warp::reply).with(cors.clone());

    warp::serve(ws_route.or(broadcast.or(register.or(options_only))))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
