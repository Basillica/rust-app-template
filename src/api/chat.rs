use actix_web::web;
use crate::handlers::chat;

pub fn get_chat_services() -> actix_web::Scope {
    return web::scope("/ws")
        .service(chat::ws)
        .service(chat::chat_ws)
}