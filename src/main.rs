use actix_web::{App, HttpServer};
use actix_web::web::Data;
use crate::contact_book::{get_messages, send_message};
use crate::registration::register_user;
use crate::store::MessageStore;
use crate::user_store::UserStore;

mod contact_book;
mod model;
mod store;
mod user_store;
mod registration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Hello, world!");
    HttpServer::new(|| App::new()
        .service(get_messages)
        .service(send_message)
        .service(register_user)
        .app_data(Data::new(MessageStore::new()))
        .app_data(Data::new(UserStore::new())))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
