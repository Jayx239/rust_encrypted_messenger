use std::collections::{HashMap};
use std::sync::Mutex;
use super::model::UserInfo;

pub struct UserStore {
    users: Mutex<HashMap<String, UserInfo>>,
    /**
    Key - user_name
    Value: user_id
    **/
    user_names: Mutex<HashMap<String, String>>
}

impl UserStore {
    pub fn new() -> UserStore {
        return UserStore {
            users: Mutex::from(HashMap::new()),
            user_names: Mutex::from(HashMap::new()),
        }
    }

    pub fn create_user(&self, user_info: UserInfo) -> UserInfo {
        let id = uuid::Uuid::new_v4().to_string();
        let mut registered_user_info = UserInfo {
            user_id: Some(id.clone()),
            user_name: user_info.user_name,
        };
        self.users.lock().unwrap().insert(id.clone(), registered_user_info.clone());
        self.user_names.lock().unwrap().insert(registered_user_info.user_name.clone(), id.clone());
        return registered_user_info.clone();
    }

    pub fn get_user_info(&self, user_id: String) -> Result<UserInfo, String> {
        let users = self.users.lock().unwrap();
        if !users.contains_key(&user_id) {
            println!("No user found in user store with id ${:?}", user_id);
            return Err("No user with specified user_id".to_string())
        }

        return Ok(users.get(&user_id).unwrap().clone())
    }

    pub fn has_user_info(&self, user_id: String) -> bool {
        return self.users.lock().unwrap().contains_key(&user_id)
    }

    pub fn has_user_name(&self, user_id: String) -> bool {
        return self.user_names.lock().unwrap().contains_key(&user_id)
    }
}
