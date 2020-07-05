use crate::common::types::{
    BroadcastMessage,
    Register,
    UnRegister
};
use warp::Filter;

pub fn json_body() -> impl Filter<Extract = (BroadcastMessage,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn json_body_register() -> impl Filter<Extract = (Register,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn json_body_unregister() -> impl Filter<Extract = (UnRegister,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn _with_db(client: redis::Client) -> impl Filter<Extract = (redis::Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

pub fn with_db_multiplexd_aio(conn: redis::aio::MultiplexedConnection) -> impl Filter<Extract = (redis::aio::MultiplexedConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || conn.clone())
}
