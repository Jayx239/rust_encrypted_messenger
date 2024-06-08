use actix_web::{get, HttpResponse, post, Responder, web};
use crate::messenger::message_store::MessageStore;
use crate::messenger::model::GetMessagesRequest;
use crate::messenger::user_store::UserStore;

#[get("/")]
pub async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html")
        .body("<html><head></head><body></body></html>")
}