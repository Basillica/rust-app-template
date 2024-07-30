use actix_web::{ post, web,HttpResponse, Responder};
use crate::models::{auth, state};


#[post("/login")]
async fn login(app_data: web::Data<state::AppState>, req: web::Json<auth::LoginReq>) -> impl Responder {
    let mut state = app_data.state.lock().unwrap();
    *state = "login".to_string();
    println!("the current app state is {}", state);
    println!("your credentials are {:?}", req);
    HttpResponse::Ok().body("Hello world!")
}