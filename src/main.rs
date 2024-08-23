use actix_web::{ web, App, HttpResponse, HttpServer, Responder};
mod chatserver;

use chrono::Local;
use cron::Schedule;
use std::sync::Mutex;
use middleware::auth::TokenAuth;
use sqlx::{ postgres::PgPoolOptions, Executor};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use dotenv::dotenv;
use std::{env, str::FromStr};


mod api;
mod handlers;
mod models;
mod utils;
pub mod middleware;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

struct CronState {
    schedule: Mutex<Schedule>
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


    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(api::public::get_public_services())
            .wrap(TokenAuth)
            .service(api::auth::get_auth_services())
            .service(api::user::get_user_services())
            .service(handlers::file::upload_video)
            .service(handlers::file::download_file)
            .service(handlers::file::uploadv1)
            .service(handlers::file::uploadv2)
            .route("/health", web::get().to(manual_hello))
            .route("/ws", web::get().to(handlers::chat::ws))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// use std::str::FromStr;
// use chrono::Utc;
// use cron::Schedule;


// fn main() {
//     let schedule = Schedule::from_str("0/10 * * * * *").unwrap();
//     for datetime in schedule.upcoming(Utc).take(10) {
//         println!("->>>> {}", datetime);
//     }
    
//     // .next().unwrap()
//     let mut next_run = schedule.upcoming(Utc).next().unwrap();
//     loop {
//         if next_run <= chrono::Local::now() {
//             // execute my job
//             println!("executing cron job ......");
//             // reschedule the next job
//             schedule.upcoming(Utc);

//             // reset
//             next_run = schedule.upcoming(Utc).next().unwrap();
//         }

//         std::thread::sleep(std::time::Duration::from_secs(1))
//     }
// }
