use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use warp::ws::{Message};
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BroadcastMessage {
    pub text: String
}

#[derive(Deserialize, Serialize)]
pub struct BroadcastMessageResponse {
    pub code: u32,
    pub reason: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterMessageResponse {
    pub code: u32,
    pub uuid: String,
    pub reason: String,
}


#[derive(Deserialize, Serialize)]
pub struct UnRegister {
    pub name: String,
    pub uuid: String,
}

#[derive(Deserialize, Serialize)]
pub struct UnRegisterMessageResponse {
    pub code: u32,
    pub reason: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterErrorResponse {
    pub code: u32,
    pub reason: String,
}

#[derive(Deserialize, Serialize)]
pub struct Register {
    pub name: String

}

#[derive(Deserialize, Serialize)]
pub struct ErrorMessage {
    pub code: u16,
    pub reason: String
}

pub type HashMapType = HashMap<String, String>;
pub(crate) type Users = Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;
