use actix_web::{ get, HttpResponse, http::header::ContentType };
use awc::Client;


pub mod login;
pub mod register;

#[get("/file")]
async fn fetch_image() -> HttpResponse {
    let client = Client::default();
        let res = client
            .get("http://www.rust-lang.org")    // <- Create request builder
            .insert_header(("User-Agent", "Actix-web"))
            .send()                             // <- Send http request
            .await;

    println!("Response: {:?}", res); 
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(format!("Response: {:?}", res))
}