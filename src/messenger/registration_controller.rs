use std::collections::HashMap;
use actix_web::{HttpResponse, post, put, web};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use super::model::{Message, UserInfo};
use super::message_store::MessageStore;
use super::user_store::UserStore;

#[derive(Debug, Deserialize, Serialize)]
struct RegisterUserRequest {
    #[serde(alias="userName")]
    pub user_name: String
}

#[derive(Debug, Deserialize, Serialize)]
struct RegisterUserResponse {
    pub user_id: String,
    pub user_name: String,
    pub status: String,
    pub message: String,
}

#[post("/register")]
pub async fn register_user(req_body: web::Json<RegisterUserRequest>, message_store: web::Data<MessageStore<Message>>, user_store: web::Data<UserStore>) -> HttpResponse {
    println!("Registering user with req_body");
    println!("Registering user with req_body {:?}", req_body.user_name.clone());
    let user_name = req_body.user_name.clone();
    // if user_store.users.lock().unwrap().contains_key(&user_id.clone()) {
    if user_store.has_user_name(user_name.clone()) {
        let response = RegisterUserResponse {
            user_id: "".to_string(),
            user_name: "".to_string(),
            status: String::from("Failed"),
            message: String::from("Username already taken"),
        };

        let body = serde_json::to_string(&response).unwrap();

        return HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(body)
    }

    let user_info = UserInfo {
        user_id: None,
        user_name: user_name.clone()
    };

    let registered_user_info= user_store.create_user(user_info.clone());

    // message_store.messages.lock().unwrap().insert(user_info.clone(), HashMap::new());
    message_store.initialize_message_store_for_user(registered_user_info.clone()).unwrap();
    // println!("{:?}",message_store.messages.lock().unwrap());

    let response = RegisterUserResponse {
        user_name: registered_user_info.clone().user_name.clone(),
        user_id: registered_user_info.user_id.unwrap().clone(),
        status: String::from("Success"),
        message: String::from("Username registered"),
    };

    let body = serde_json::to_string(&response).unwrap();

    return HttpResponse::Accepted()
        .content_type(ContentType::json())
        .body(body)

}

#[cfg(test)]
mod tests {
    use actix_web::test;
    use actix_web::http::header::ContentType;
    use actix_web::App;
    use actix_web::web::Data;
    use crate::messenger::message_store::MessageStore;
    use crate::messenger::registration_controller::RegisterUserRequest;
    use crate::messenger::user_store::UserStore;

    #[test]
    async fn it_registers_a_user() {
        let user_store = UserStore::new();
        let message_store = MessageStore::new();
        let request = RegisterUserRequest {
            user_name: String::from("jason"),
        };
        // let response = register_user(request,
        //               user_store,
        //               message_store);
    }

    #[actix_web::test]
    async fn test_register_user() {
        let user_store = Data::new(UserStore::new());
        let message_store = Data::new(MessageStore::new());
        let app = test::init_service(App::new().service(register_user)
        .app_data(user_store.clone())
        .app_data(message_store.clone())).await;
        let request = RegisterUserRequest {             user_id: String::from("jason"),};
        let req = test::TestRequest::post()
            .insert_header(ContentType::json())
            .uri("/register")
            .set_json(request)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert!(user_store.clone().users.lock().unwrap().get(&String::from("jason")).is_some());
        println!("{:?}", user_store.clone().users.lock().unwrap().get(&String::from("jason")))
    }
}
