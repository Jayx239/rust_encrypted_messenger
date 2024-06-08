use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Debug, Hash, Eq, PartialEq, Clone)]
pub struct UserInfo {
    #[serde(alias="userId")]
    pub user_id: Option<String>,
    #[serde(alias="userName")]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum MessageIO {
    Inbound,
    Outbound
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    #[serde(alias="sentAt")]
    pub sent_at: u64,
    #[serde(alias="toUserId")]
    pub to_user_id: String,
    #[serde(alias="fromUserId")]
    pub from_user_id: String,
    #[serde(alias="messageId")]
    pub message_id: String,
    #[serde(alias="messageIO")]
    pub io: MessageIO,
    #[serde(alias="body")]
    pub body: String
}

impl GetMessageId for Message {
    fn get_message_id(self) -> String {
        return self.message_id.clone()
    }
}

pub trait GetMessageId {
    fn get_message_id(self) -> String;
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
    #[serde(alias="messageId")]
    pub message_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMessagesRequest {
    #[serde(alias="userId")]
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMessageRequest {
    #[serde(alias="userId")]
    pub user_id: String,
    #[serde(alias="messageId")]
    pub message_id: String,
}
