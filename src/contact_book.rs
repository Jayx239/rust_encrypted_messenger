use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, put, HttpRequest};
use actix_web::body::{BodySize, BoxBody, MessageBody};
use actix_web::http::header::ContentType;
use actix_web::web::{Bytes, get};
use serde::{Deserialize, Serialize};
use log::{error, info};
use uuid::uuid;
use crate::contact_book::SendMessageStatus::{Failed, Pending};

#[derive(Serialize, Deserialize, Debug)]
struct GetMessagesRequest {
    user_id: String,
}

#[post("/messages")]
async fn get_messages(req_body: GetMessagesRequest) -> impl Responder {
    HttpResponse::Ok().body("")
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SendMessageRequest {
    from_user_id: String,
    to_user_id: String,
    message: String,
}

#[derive(Clone, Debug)]
enum SendMessageStatus {
    Sent,
    Pending,
    Failed
}

#[derive(Serialize, Deserialize, Debug, Clone, Siz)]
struct SendMessageResponse {
    status: SendMessageStatus,
    message: String,
    message_id: String,
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

struct UserInfo {
    name: String,

}

enum MessageIO {
    Inbound,
    Outbound
}

struct Message {
    io: MessageIO,
    body: String
}

struct MessageIOPair {
    inbound: HashMap<String, Message>,
    outbound: HashMap<String, Message>
}

struct UserStore {
    users: Mutex<HashMap<String, UserInfo>>
}

struct MessageStore {
    messages: Mutex<HashMap<UserInfo, HashMap<String, Message>>>,
}

#[put("/message")]
async fn echo(req_body: SendMessageRequest, message_store: web::Data<MessageStore>, user_store: web::Data<UserStore>) -> impl Responder {
    info("Send message request received: ${?:}", req_body.clone());
    let users = user_store.users.lock().unwrap();
    let from_user = users.get(&req_body.from_user_id);

    if from_user.is_none() {
        error!("from_user not found for send message request: ${:?}", req_body);
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

    let to_user = users.get(&req_body.to_user_id);
    if to_user.is_none() {
        error!("to_user not found for send message request: ${:?}", req_body);
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
    let messages = message_store.messages.lock().unwrap();
    
    let response = SendMessageResponse {
        status: Pending,
        message: String::from("Message pending"),
        message_id,
    };
    let body = serde_json::to_string(&response).unwrap();
    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}