use actix_web::web;
use crate::handlers:: nats::publish_message;

pub fn get_nasts_services() -> actix_web::Scope {
    return web::scope("/nats")
        .service(publish_message)
}