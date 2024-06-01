use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct UserInfo {
    pub name: String,
}

pub enum MessageIO {
    Inbound,
    Outbound
}

pub struct Message {
    pub io: MessageIO,
    pub body: String
}

pub struct MessageIOPair {
    pub inbound: HashMap<String, Message>,
    pub outbound: HashMap<String, Message>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendMessageRequest {

    #[serde(alias="fromUserId")]
    pub from_user_id: String,
    #[serde(alias="toUserId")]
    pub to_user_id: String,
    #[serde(alias="message")]
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Status {
    Sent,
    Pending,
    Failed
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendMessageResponse {
    pub status: Status,
    pub message: String,
    pub message_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMessagesRequest {
    pub user_id: String,
}