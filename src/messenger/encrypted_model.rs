use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::messenger::model::MessageIO;
use crate::messenger::traits::GetMessageId;

#[derive(Deserialize, Serialize, Clone)]
pub struct Message {

    #[serde(alias="sentAt")]
    #[serde(rename(serialize = "sentAt", deserialize = "sentAt"))]
    pub sent_at: u64,

    #[serde(alias="toUserId")]
    #[serde(rename(serialize = "toUserId", deserialize = "toUserId"))]
    pub to_user_id: String,

    #[serde(alias="fromUserId")]
    #[serde(rename(serialize = "fromUserId", deserialize = "fromUserId"))]
    pub from_user_id: String,

    #[serde(alias="messageId")]
    #[serde(rename(serialize = "messageId", deserialize = "messageId"))]
    pub message_id: String,
    #[serde(alias="io")]
    #[serde(rename(serialize = "io", deserialize = "io"))]
    pub io: MessageIO,
    #[serde(alias="iv")]
    #[serde(rename(serialize = "iv", deserialize = "iv"))]
    pub iv: Vec<u8>,
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
    #[serde(rename(serialize = "fromUserId", deserialize = "fromUserId"))]
    pub from_user_id: String,

    #[serde(alias="toUserId")]
    #[serde(rename(serialize = "toUserId", deserialize = "toUserId"))]
    pub to_user_id: String,
    pub message: Vec<u8>,
    pub iv: Vec<u8>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Status {
    Sent,
    Pending,
    Failed
}
