use actix_web::web;
use crate::handlers::{auth::logout, users::{add_user, delete_user, get_user, get_users}};

pub fn get_user_services() -> actix_web::Scope {
    return web::scope("/user")
    .service(add_user::add_user)
    .service(get_user::get_user)
    .service(get_users::get_users)
    .service(delete_user::delete_user)
    .service(logout::logout)
}