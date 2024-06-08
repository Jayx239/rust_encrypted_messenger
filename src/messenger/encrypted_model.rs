use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::messenger::model::{GetMessageId, MessageIO};

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
    pub body: Vec<u8>
}

impl GetMessageId for Message {
    fn get_message_id(self) -> String {
        return self.message_id.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendMessageRequest {

    #[serde(alias="fromUserId")]
    pub from_user_id: String,
    #[serde(alias="toUserId")]
    pub to_user_id: String,
    #[serde(alias="message")]
    pub message: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Status {
    Sent,
    Pending,
    Failed
}
