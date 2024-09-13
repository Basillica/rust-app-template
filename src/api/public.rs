use actix_web::web;
use crate::handlers::public::{fetch_image, login, register};

pub fn get_public_services() -> actix_web::Scope {
    return web::scope("/public")
        .service(login::login)
        .service(register::register)
        .service(fetch_image)
}