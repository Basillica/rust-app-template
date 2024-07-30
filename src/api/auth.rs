 
use actix_web::web;
use crate::handlers::auth::{get_user, get_users, logout};

pub fn get_auth_services() -> actix_web::Scope {
    return web::scope("/auth")
    .service(get_user::get_user)
    .service(get_users::get_users)
    .service(logout::logout)
}