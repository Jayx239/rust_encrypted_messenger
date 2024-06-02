use std::collections::HashMap;
use actix_web::{HttpResponse, post, put, web};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use crate::model::{SendMessageRequest, SendMessageResponse, Status, UserInfo};
use crate::model::Status::Failed;
use crate::store::MessageStore;
use crate::user_store::UserStore;

#[derive(Debug, Deserialize, Serialize)]
struct RegisterUserRequest {
    #[serde(alias="userId")]
    pub user_id: String
}

#[derive(Debug, Deserialize, Serialize)]
struct RegisterUserResponse {
    status: String,
    message: String,
}

#[post("/register")]
pub async fn register_user(req_body: web::Json<RegisterUserRequest>, message_store: web::Data<MessageStore>, user_store: web::Data<UserStore>) -> HttpResponse {
    let user_id = req_body.user_id.clone();
    if user_store.users.lock().unwrap().contains_key(&user_id.clone()) {
        let response = RegisterUserResponse {
            status: String::from("Failed"),
            message: String::from("Username already taken"),
        };

        let body = serde_json::to_string(&response).unwrap();

        return HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(body)
    }

    let user_info = UserInfo {
        name: user_id.clone()
    };

    user_store.users.lock().unwrap().insert(user_id.clone(), user_info.clone());

    message_store.messages.lock().unwrap().insert(user_info.clone(), HashMap::new());

    // println!("{:?}",message_store.messages.lock().unwrap());

    let response = RegisterUserResponse {
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
    use crate::registration::{register_user, RegisterUserRequest};
    use crate::store::MessageStore;
    use crate::user_store::UserStore;
    use actix_web::test;
    use actix_web::http::header::ContentType;
    use actix_web::App;
    use actix_web::web::Data;
    
    #[test]
    async fn it_registers_a_user() {
        let user_store = UserStore::new();
        let message_store = MessageStore::new();
        let request = RegisterUserRequest {
            user_id: String::from("jason"),
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
