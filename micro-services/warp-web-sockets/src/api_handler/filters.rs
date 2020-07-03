use crate::common::types::{
    BroadcastMessage,
    Register
};
use warp::Filter;

pub fn json_body() -> impl Filter<Extract = (BroadcastMessage,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn json_body_register() -> impl Filter<Extract = (Register,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn with_db(client: redis::Client) -> impl Filter<Extract = (redis::Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}
