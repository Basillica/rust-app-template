use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;


#[derive(Deserialize)]
struct PathParams {
    name: String,
    id: String,
    email: String,
}

#[derive(Deserialize, Debug)]
struct LoginReq {
    password: String,
    email: String,
}

#[derive(Deserialize, Debug)]
struct UserModel {
    firstname: String,
    lastname: String,
    password: String,
    email: String,
}

struct AppState {
    state: Mutex<String>
}


#[post("/login")]
async fn login(app_data: web::Data<AppState>, req: web::Json<LoginReq>) -> impl Responder {
    let mut state = app_data.state.lock().unwrap();
    *state = "login".to_string();
    println!("the current app state is {}", state);
    println!("your credentials are {:?}", req);
    HttpResponse::Ok().body("Hello world!")
}

#[get("/logout/{name}")]
async fn logout(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}, you have been logged out!", &name))
}


#[get("/user/{name}/{id}/{email}")]
async fn fetch_user(app_data: web::Data<AppState>, path: web::Path<PathParams>) -> impl Responder {
    let state = app_data.state.lock().unwrap();
    println!("the current app state is {}", state);
    HttpResponse::Ok().body(format!("Hello {}, your id is {} and your email is {}!", &path.name, &path.id, &path.email))
}

#[get("/user")]
async fn get_user(path: web::Query<PathParams>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}, your id is {} and your email is {}!", &path.name, &path.id, &path.email))
}

#[get("/user/register")]
async fn register(user: web::Either<web::Json<UserModel>, web::Form<UserModel>>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {:?}, you have been logged out!", &user))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState{
        state: Mutex::new("init-state".to_string())
    });
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::scope("/auth")
                .service(login)
                .service(logout)
                .service(fetch_user)
                .service(get_user)
                .service(register)
            )
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}