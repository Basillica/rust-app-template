use actix_web::{ web, App, HttpResponse, HttpServer, Responder, middleware as default_middleware};
use chrono::Local;
use cron::Schedule;
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{Executor, PgPool};
use shuttle_runtime::SecretStore;
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


#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     dotenv().ok();

    let (tx, rx) = mpsc::channel::<String>(100);
    let sender = Mutex::new(tx);

    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());

    let db_conn_str = secret_store.get("DATABASE_CONNECTION_STRING").expect("DATABASE_CONNECTION_STRING must be set");
    let jwt_secret = secret_store.get("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_subjet = secret_store.get("JWT_SUBJECT").expect("JWT_SUBJECT must be set");
    let jwt_aud = secret_store.get("JWT_AUDIENCE").expect("JWT_AUDIENCE must be set");
    let jwt_iss = secret_store.get("JWT_ISSUER").expect("JWT_ISSUER must be set");

    let nats_client = match async_nats::connect(nats_url).await {
        Ok(client) => {
            let client2 = client.clone();
            // let handles = vec![
            tokio::spawn(utils::queue::send_to_nats(client2.clone()));
            tokio::spawn(utils::queue::handle_nats_messages(client2.clone()));
            tokio::spawn(utils::queue::handler_sender(client2.clone(), rx));
            tokio::spawn(utils::queue::receive_from_nats(client2));
            // ];
            // let mut results = Vec::with_capacity(handles.len());
            // for handle in handles {
            //     results.push(handle.await.unwrap()); // this is a blocking piece of code
            // };
            Some(Mutex::new(client))
        },
        Err(e) => {
            eprintln!("Failed to create NATS client: {}", e);
            None
        },
    };

    let pool = utils::get_db_pool(include_str!("../schema.sql")).await;
    let data = web::Data::new(models::state::AppState{ pool, nats_client, sender, db_conn_str, jwt_secret, jwt_subjet, jwt_aud, jwt_iss});
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
    

    // uncomment this part out if you would like to run outside of shuttle
    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(data.clone())
    //         .wrap(default_middleware::Logger::default())
    //         .wrap(default_middleware::DefaultHeaders::new().add(("X-XSS-Protection", "1; mode=block")))
    //         .wrap(default_middleware::Compress::default())
    //         .wrap(TokenAuth::default())
    //         .service(api::public::get_public_services())
    //         .service(api::auth::get_auth_services())
    //         .service(api::user::get_user_services())
    //         .service(handlers::file::upload_video)
    //         .service(handlers::file::download_file)
    //         .service(handlers::file::uploadv1)
    //         .service(handlers::file::uploadv2)
    //         .service(api::nats::get_nasts_services())
    //         .route("/health", web::get().to(index))
    //         .route("/ws", web::get().to(handlers::chat::ws))
    // }).bind(("127.0.0.1", 8080))?
    // .run()
    // .await

    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            web::scope("/v1")
                .app_data(data.clone())
                .wrap(default_middleware::Logger::default())
                .wrap(default_middleware::DefaultHeaders::new().add(("X-XSS-Protection", "1; mode=block")))
                .wrap(default_middleware::Compress::default())
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
        );
    };

    Ok(config.into())
}


// #[cfg(test)]
// mod tests {
//     use actix_web::{http::header::ContentType, test, App};
//     use utils::jwt::jwt::decode;

//     use super::*;

//     #[actix_web::test]
//     async fn test_index_post() {
//         let app = test::init_service(
//             App::new()
//                 .route("/health", web::get().to(index))
//         ).await;
//         let req = test::TestRequest::get().uri("/health").to_request();
//         let resp = test::call_service(&app, req).await;
//         println!("the frigging resp{:?} and status {}", resp, resp.status());
//         assert!(resp.status().is_success());
//     }

//     #[actix_web::test]
//     async fn test_login() {
//         dotenv().ok();
//         let pool = utils::get_db_pool(include_str!("../schema.sql")).await;
//         let (tx, _rx) = mpsc::channel::<String>(100);
//         let sender = Mutex::new(tx);
//         // let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
//         // let client = async_nats::connect(nats_url).await.unwrap();
//         // let nats_client = Mutex::new(client);
//         let data = web::Data::new(models::state::AppState{ pool, nats_client: None, sender });

//         let app = test::init_service(
//             App::new()
//                 .app_data(data.clone())
//                 .wrap(default_middleware::Logger::default())
//                 .wrap(TokenAuth::default())
//                 .service(api::public::get_public_services())
//                 .service(api::user::get_user_services())
//         )
//         .await;


//         let payload = r#"{"password":"12345", "email":"basillica@example.com"}"#.as_bytes();

//         // test login
//         let req = test::TestRequest::post()
//             .uri("/public/login")
//             .insert_header(ContentType::json())
//             .set_payload(payload)
//             .to_request();

//         let resp = test::call_service(&app, req).await;
//         assert!(resp.status().is_success());
//         let body = test::read_body(resp).await;
//         assert_eq!(true, decode(std::str::from_utf8(&body).unwrap()));

//         // test fetch users with token
//         let req = test::TestRequest::get()
//             .uri("/user/users")
//             .insert_header(("Authorization", format!("Bearer {}", std::str::from_utf8(&body).unwrap())))
//             .to_request();

//         let resp = test::call_service(&app, req).await;
//         assert!(resp.status().is_success());
//     }
// }
