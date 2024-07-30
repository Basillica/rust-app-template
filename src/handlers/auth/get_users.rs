use actix_web::{get, web, HttpResponse, Responder};
use crate::models::{auth, state};

#[get("/user/{name}/{id}/{email}")]
async fn get_users(app_data: web::Data<state::AppState>, path: web::Path<auth::PathParams>) -> impl Responder {
    let state = app_data.state.lock().unwrap();
    println!("the current app state is {}", state);
    HttpResponse::Ok().body(format!("Hello {}, your id is {} and your email is {}!", &path.name, &path.id, &path.email))
}