use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};
use std::time::UNIX_EPOCH;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, put, HttpRequest};
use actix_web::body::{BodySize, BoxBody, MessageBody};
use actix_web::http::header::{ContentType, Date};
use actix_web::web::{Bytes, get};
use serde::{Deserialize, Serialize};
use log::{error, info};
use uuid::uuid;
use super::model::{GetMessageRequest, GetMessagesRequest, Message, MessageIO, SendMessageRequest, SendMessageResponse};
use super::model::Status::{Failed, Pending};
use super::message_store::MessageStore;
use super::user_store::UserStore;

#[post("/messages")]
pub async fn get_messages(req_body: web::Json<GetMessagesRequest>, message_store: web::Data<MessageStore<Message>>, user_store: web::Data<UserStore>) -> impl Responder {
    let user_id = req_body.user_id.clone();
    let user_data = user_store.get_user_info(user_id.clone()).unwrap().clone();
    // let messages = message_store.messages.lock().unwrap();
    let messages = message_store.get_messages(user_data.clone()).unwrap();
    let message = serde_json::to_string(&messages).unwrap();
    message_store.remove_messages(user_data, messages);
    HttpResponse::Ok().body(message)
}

#[post("/message")]
pub async fn get_message(req_body: web::Json<GetMessageRequest>, message_store: web::Data<MessageStore<Message>>, user_store: web::Data<UserStore>) -> impl Responder {
    let user_id = req_body.user_id.clone();
    let message_id = req_body.message_id.clone();

    let user_data = user_store.get_user_info(user_id.clone()).unwrap().clone();
    let messages = message_store.get_messages(user_data.clone()).unwrap();
    // let messages = messages.get(&user_data.clone()).unwrap();
    let message = serde_json::to_string(&messages.clone()).unwrap();
    HttpResponse::Ok().body(message)
}


impl Responder for SendMessageResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}


#[put("/message")]
pub async fn send_message(req_body: web::Json<SendMessageRequest>, mut message_store: web::Data<MessageStore<Message>>, user_store: web::Data<UserStore>) -> HttpResponse {
    println!("Send message request received: ${:?}", req_body.clone());
    // let users = user_store.users.lock().unwrap();
    let from_user = user_store.get_user_info(req_body.clone().from_user_id);

    if from_user.is_err() {
        println!("from_user not found for send message request: ${:?}", req_body);
        let response = SendMessageResponse {
            status: Failed,
            message: String::from("Invalid sender address"),
            message_id: String::new()
        };

        let body = serde_json::to_string(&response).unwrap();

        return HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(body)
    }

    let mut to_user = user_store.get_user_info(req_body.clone().to_user_id);
    if to_user.is_err() {
        println!("to_user not found for send message request: ${:?}", req_body);
        let response = SendMessageResponse {
            status: Failed,
            message: String::from("Invalid recipient specified"),
            message_id: String::new()
        };

        let body = serde_json::to_string(&response).unwrap();

        return HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(body)
    }

    let message_id = uuid::Uuid::new_v4().to_string();
    let message = &req_body.message;
    message_store.put_message(to_user.unwrap().clone(), Message {
        sent_at: UNIX_EPOCH.elapsed().unwrap().as_secs(),
        to_user_id: req_body.to_user_id.clone(),
        from_user_id: req_body.from_user_id.clone(),
        message_id: message_id.clone(),
        io: MessageIO::Inbound,
        body: message.clone(),
    }).unwrap();
    // let mut messages = message_store.messages.lock().unwrap();
    // let message = &req_body.message;
    // messages.get_mut(&to_user.unwrap()).unwrap().insert(from_user.unwrap().clone().name, Message {
    //     message_id: message_id.clone(),
    //     io: MessageIO::Inbound,
    //     body: message.clone(),
    // });

    // messages.get(&from_user.unwrap()).unwrap().insert(message_id.clone(), Message {
    //     io: MessageIO::Outbound,
    //     body: message.clone(),
    // });
    let response = SendMessageResponse {
        status: Pending,
        message: String::from("Message pending"),
        message_id,
    };
    let body = serde_json::to_string(&response).unwrap();

    println!("Successfully stored message, sending ok response");
    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
