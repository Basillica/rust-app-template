use actix_web::web;
use crate::{handlers::{auth::logout, users::{get_user, get_users}}, utils::guard::AuthorizationHeader};

pub fn get_auth_services() -> actix_web::Scope {
    return web::scope("/auth")
        .guard(AuthorizationHeader)
        .service(get_user::get_user)
        .service(get_users::get_users)
        .service(logout::logout)
}