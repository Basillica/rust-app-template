use actix_web::{get, web,HttpResponse, Responder};
use crate::models::auth;

#[get("/user/register")]
async fn register(user: web::Either<web::Json<auth::UserModel>, web::Form<auth::UserModel>>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {:?}, you have been logged out!", &user))
}