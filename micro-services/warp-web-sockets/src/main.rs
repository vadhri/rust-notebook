use std::collections::HashMap;
use std::sync::{
    Arc,
};

use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, RwLock};
use warp::ws::{Message, WebSocket};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BroadcastMessage {
    pub text: String
}

#[derive(Deserialize, Serialize)]
struct Response {
    code: u32,
    reason: String,
}
use std::convert::Infallible;

async fn broadcast_message_handler(users: Users, msg: String) -> Result<impl warp::Reply, Infallible> {

    broadcast_message(Message::text(msg), users.clone()).await;

    let r: Response = Response {
        code: 0,
        reason: format!("Message broadcast to {:?} users.", users.read().await.len())
    };

    Ok(warp::reply::json(&r))
}

fn json_body() -> impl Filter<Extract = (BroadcastMessage,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let _res = redis_wrapper::init::initialize_redis("redis://127.0.0.1:6379/".to_string());
    let users = Users::default();
    let users_filterized = warp::any().map(move || users.clone());

    topics::t::get_topics_list(&mut _res.unwrap());

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

      warp::serve(ws_route.or(broadcast))
          .run(([127, 0, 0, 1], 3030))
          .await;
}
