use actix_web::{ web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use chrono::Local;
use cron::Schedule;
use tokio::sync::mpsc;
use std::sync::Mutex;
use middleware::auth::TokenAuth;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use dotenv::dotenv;
use std::{env, str::FromStr};


mod api;
mod handlers;
mod models;
mod utils;
mod middleware;
// mod chatserver;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("server is alive")
}

struct CronState {
    schedule: Mutex<Schedule>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let (tx, rx) = mpsc::channel::<String>(100);
    let sender = Mutex::new(tx);

    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let client = async_nats::connect(nats_url).await.unwrap();
    let client2 = client.clone();
    let nats_client = Mutex::new(client);
    let pool = utils::get_db_pool(include_str!("../schema.sql")).await;
    let data = web::Data::new(models::state::AppState{ pool, nats_client, sender });
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("error setting global subscriber for tracing");

    info!("starting server at port 8080");


    // setup a cron on a new thread
    let cron_state = CronState{
        schedule: Mutex::new(Schedule::from_str("0/10 * * * * *").unwrap()),
    };
    std::thread::spawn(move || {
        let mut last_run = chrono::Local::now();
        loop {
            let schedule = cron_state.schedule.lock().unwrap();
            let next_run = schedule.upcoming(Local).next().unwrap();
            drop(schedule); // unlock the mutex

            if next_run > last_run{
                let wait_time = next_run - chrono::Local::now();
                std::thread::sleep(std::time::Duration::from_secs(wait_time.num_seconds() as u64));
                println!("perioding task executed at {}", chrono::Local::now());
                last_run = chrono::Local::now();
            }
        }
    });
    
    tokio::spawn(utils::queue::send_to_nats(client2.clone()));
    tokio::spawn(utils::queue::handle_nats_messages(client2.clone()));
    tokio::spawn(utils::queue::handler_sender(client2.clone(), rx));
    tokio::spawn(utils::queue::receive_from_nats(client2));


    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .wrap(TokenAuth::default())
            .service(api::public::get_public_services())
            .service(api::auth::get_auth_services())
            .service(api::user::get_user_services())
            .service(handlers::file::upload_video)
            .service(handlers::file::download_file)
            .service(handlers::file::uploadv1)
            .service(handlers::file::uploadv2)
            .service(api::nats::get_nasts_services())
            .route("/health", web::get().to(index))
            .route("/ws", web::get().to(handlers::chat::ws))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};
    use utils::jwt::jwt::decode;

    use super::*;

    #[actix_web::test]
    async fn test_index_post() {
        let app = test::init_service(
            App::new()
                .route("/health", web::get().to(index))
        ).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        println!("the frigging resp{:?} and status {}", resp, resp.status());
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_login() {
        dotenv().ok();
        let pool = utils::get_db_pool(include_str!("../schema.sql")).await;
        let (tx, _rx) = mpsc::channel::<String>(100);
        let sender = Mutex::new(tx);
        let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
        let client = async_nats::connect(nats_url).await.unwrap();
        let nats_client = Mutex::new(client);
        let data = web::Data::new(models::state::AppState{ pool, nats_client, sender });

        let app = test::init_service(
            App::new()
                .app_data(data.clone())
                .wrap(Logger::default())
                .wrap(TokenAuth::default())
                .service(api::public::get_public_services())
                .service(api::user::get_user_services())
        )
        .await;


        let payload = r#"{"password":"12345", "email":"basillica@example.com"}"#.as_bytes();

        // test login
        let req = test::TestRequest::post()
            .uri("/public/login")
            .insert_header(ContentType::json())
            .set_payload(payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(true, decode(std::str::from_utf8(&body).unwrap()));

        // test fetch users with token
        let req = test::TestRequest::get()
            .uri("/user/users")
            .insert_header(("Authorization", format!("Bearer {}", std::str::from_utf8(&body).unwrap())))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
