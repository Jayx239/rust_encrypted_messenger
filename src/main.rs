use std::string::ToString;
use std::sync::Mutex;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
#[macro_use]
extern crate lazy_static;

mod messenger;

use crate::messenger::messenger_controller::{get_messages, send_message};
use crate::messenger::index::get_index;
use crate::messenger::registration_controller::register_user;
use crate::messenger::message_store::MessageStore;
use crate::messenger::model::Message;
use crate::messenger::user_store::UserStore;

const IP_ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    encrypted().await

}

async fn unencrypted() -> std::io::Result<()> {
    let mut message_app_data: Data<MessageStore<Message>> = Data::new(MessageStore::new());
    let mut user_app_data: Data<UserStore> = Data::new(UserStore::new());

    println!("Starting unencrypted messenger server on IP {:?} and port {:?}", IP_ADDRESS, PORT);

    HttpServer::new(move || App::new()
    .service(get_index)
    .service(get_messages)
    .service(send_message)
    .service(register_user)
    .app_data(message_app_data.to_owned())
    .app_data(user_app_data.to_owned()))
    .bind((IP_ADDRESS, PORT))?
    .run()
    .await
}

async fn encrypted() -> std::io::Result<()> {
    let mut message_app_data: Data<MessageStore<messenger::encrypted_model::Message>> = Data::new(MessageStore::new());
    let mut user_app_data: Data<UserStore> = Data::new(UserStore::new());

    println!("Starting encrypted messenger server on IP {:?} and port {:?}", IP_ADDRESS, PORT);

    HttpServer::new(move || App::new()
        .service(get_index)
        .service(messenger::encrypted_messenger_controller::get_messages)
        .service(messenger::encrypted_messenger_controller::send_message)
        .service(messenger::encrypted_registration_controller::register_user)
        .app_data(message_app_data.to_owned())
        .app_data(user_app_data.to_owned()))
        .bind((IP_ADDRESS, PORT))?
        .run()
        .await
}
