use std::collections::HashMap;
use std::sync::Mutex;
use crate::model::{Message, UserInfo};


pub struct MessageStore {
    pub messages: Mutex<HashMap<UserInfo, HashMap<String, Message>>>,
}

impl MessageStore {
    pub fn new() -> MessageStore {
        return MessageStore {
            messages: Mutex::from(HashMap::new())
        }
    }
}