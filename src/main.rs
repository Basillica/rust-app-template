use actix_web::{ web, App, HttpResponse, HttpServer, Responder};
use middleware::auth::TokenAuth;
use sqlx::{ postgres::PgPoolOptions, Executor};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use dotenv::dotenv;
use std::env;


mod api;
mod handlers;
mod models;
mod utils;
pub mod middleware;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_conn_string = env::var("DATABASE_CONNECTION_STRING").expect("the database connection string was not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_conn_string)
        .await.expect("could not exstablish a connection to the database");

    let _ = pool.execute(include_str!("../schema.sql"))
        .await
        .expect("there was some error executing schema");

    let data = web::Data::new(models::state::AppState{ pool });
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("error setting global subscriber for tracing");

    // redis connection, will error out if there it is not able to make a connection
    // let key = Key::generate();
    // let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379").await.unwrap();

    info!("starting server at port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(api::public::get_public_services())
            .wrap(TokenAuth)
            // .wrap(
            //     SessionMiddleware::new(redis_store.clone(), key.clone())
            // )
            .service(api::auth::get_auth_services())
            .service(api::user::get_user_services())
            .route("/health", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}