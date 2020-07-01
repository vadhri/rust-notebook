use std::collections::HashMap;
use std::sync::{
    Arc,
};

use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, RwLock};
use warp::ws::{Message, WebSocket};
use uuid::Uuid;

use warp::Filter;

mod redis_wrapper;
mod topics;
mod common;

type Users = Arc<RwLock<HashMap<Uuid, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

async fn broadcast_message(msg: Message, users: Users) {
    // println!("[broadcast_message] Message to be broadcasted = {:?}", msg.clone());
    for user in users.read().await.iter() {
        // println!("[broadcast_message] send meessage to user -> {:?}", user.0);
        let _res = user.1.send(Ok(msg.clone()));
    }
}

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
    let _res = redis_wrapper::init::initialize_redis("redis://127.0.0.1:6379/".to_string());
    let users = Users::default();
    let users_filterized = warp::any().map(move || users.clone());

    topics::t::get_topics_list(&mut _res.unwrap());

    let ws_route = warp::path("ws")
      .and(warp::ws())
      .and(users_filterized)
      .map(|ws: warp::ws::Ws, users: Users| {
          ws.on_upgrade(move |incoming_websocket| {
              user_connected(incoming_websocket, users.clone())
          })
      });

      warp::serve(ws_route)
          .run(([127, 0, 0, 1], 3030))
          .await;
}
