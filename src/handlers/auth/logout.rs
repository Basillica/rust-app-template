use actix_web::{get, web, HttpResponse, Responder};


#[get("/logout/{name}")]
async fn logout(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}, you have been logged out!", &name))
}