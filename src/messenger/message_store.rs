use std::collections::HashMap;
use std::sync::Mutex;
use super::model::{Message, UserInfo};


pub struct MessageStore {
    /**
        Key: user_id,
        Value: {
            message_id,
            Message
        }
    **/
    messages: Mutex<HashMap<String, HashMap<String, Message>>>,
}

impl MessageStore {
    pub fn new() -> MessageStore {
        return MessageStore {
            messages: Mutex::from(HashMap::new())
        }
    }

    pub fn get_messages(&self, user_info: UserInfo) -> Result<HashMap<String, Message>, String> {
        let id = user_info.user_id.clone().unwrap();
        let message_store = self.messages.lock().unwrap();
        let messages = message_store.get(&id);

        // let messages = self.messages.lock().unwrap();
        // if messages.contains_key(&user_info) {
        // if !self.messages.lock().unwrap().contains_key(&user_info) {
        if messages.is_none() {
            println!("Failed to get messages as there is no messages map for {:?}", user_info);
            return Err("Failed to get messages due to missing messages map".to_string())
        }

        return Ok(messages.unwrap().clone());
    }

    pub fn put_message(&self, user_info: UserInfo, message: Message) -> Result<(), String> {
        let id = user_info.user_id.clone().unwrap();
        let mut all_messages = self.messages.lock().unwrap();

        if !all_messages.contains_key(&id) {
        // if !self.messages.lock().unwrap().contains_key(&user_info) {
            println!("No message store for user: {:?}", user_info);
            return Err("User message store not found".to_string())
        }

        let mut messages = all_messages.get(&id).unwrap().clone();
        messages.insert(message.clone().message_id, message);

        // self.messages.lock().unwrap().insert(user_info, messages);
        all_messages.insert(id, messages);
        return Ok(())
    }

    pub fn initialize_message_store_for_user(&self, user_info: UserInfo) -> Result<(), String> {
        let id = user_info.user_id.clone().unwrap();
        let mut messages = self.messages.lock().unwrap();
        if messages.contains_key(&id) {
        // if messages.lock().unwrap().contains_key(&user_info) {
            return Err("Message store already exists".to_string())
        }

        // self.messages.lock().unwrap().insert(user_info.clone(), HashMap::new());
        messages.insert(id, HashMap::new());
        return Ok(())
    }

    pub fn remove_messages(&self, user_info: UserInfo, messages_to_remove: HashMap<String, Message>) {
        let id = user_info.user_id.unwrap();
        let mut messages = self.messages.lock().unwrap();
        let mut existing_messages = messages.get(&id).unwrap().to_owned();//self.messages.lock().unwrap().get(&user_info.clone()).unwrap().to_owned();
        // let mut existing_messages = messages.;

        for message in messages_to_remove {
            existing_messages.remove(&message.0);
        }

        // self.messages.lock().unwrap().insert(user_info, existing_messages.to_owned());
            messages.insert(id, existing_messages.to_owned());
    }
}