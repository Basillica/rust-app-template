use std::sync::Mutex;
use actix_web::{ web, App, HttpResponse, HttpServer, Responder};


mod api;
mod handlers;
mod models;
mod utils;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(models::state::AppState{
        state: Mutex::new("init-state".to_string())
    });
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(api::public::get_public_services())
            .service(api::auth::get_auth_services())
            .route("/health", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}