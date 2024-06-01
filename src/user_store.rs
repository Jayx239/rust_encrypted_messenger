use std::collections::HashMap;
use std::sync::Mutex;
use crate::model::UserInfo;

pub struct UserStore {
    pub users: Mutex<HashMap<String, UserInfo>>
}

impl UserStore {
    pub fn new() -> UserStore {
        return UserStore {
            users: Mutex::from(HashMap::new())
        }
    }
}