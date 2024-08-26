use actix_web::{ web, App, HttpResponse, HttpServer, Responder};
use async_nats::message;
use chrono::Local;
use cron::Schedule;
use futures_util::StreamExt;
use tokio::sync::mpsc;
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
mod middleware;
mod chatserver;

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

    let pool = Mutex::new(pool);

    let (tx, rx) = mpsc::channel::<String>(100);
    let sender = Mutex::new(tx);

    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let client = async_nats::connect(nats_url).await.unwrap();
    let client2 = client.clone();
    let nats_client = Mutex::new(client);

    let data = web::Data::new(models::state::AppState{ pool, nats_client, sender });
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("error setting global subscriber for tracing");

    info!("starting server at port 8080");


    // // setup a cron on a new thread
    // let cron_state = CronState{
    //     schedule: Mutex::new(Schedule::from_str("0/10 * * * * *").unwrap()),
    // };
    // std::thread::spawn(move || {
    //     let mut last_run = chrono::Local::now();
    //     loop {
    //         let schedule = cron_state.schedule.lock().unwrap();
    //         let next_run = schedule.upcoming(Local).next().unwrap();
    //         drop(schedule); // unlock the mutex

    //         if next_run > last_run{
    //             let wait_time = next_run - chrono::Local::now();
    //             std::thread::sleep(std::time::Duration::from_secs(wait_time.num_seconds() as u64));
    //             println!("perioding task executed at {}", chrono::Local::now());
    //             last_run = chrono::Local::now();
    //         }
    //     }
    // });
    
    tokio::spawn(send_to_nats(client2.clone()));
    tokio::spawn(handle_nats_messages(client2.clone()));
    tokio::spawn(handler_sender(client2.clone(), rx));
    tokio::spawn(receive_from_nats(client2));


    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(api::public::get_public_services())
            // .wrap(TokenAuth)
            .service(api::auth::get_auth_services())
            .service(api::user::get_user_services())
            .service(handlers::file::upload_video)
            .service(handlers::file::download_file)
            .service(handlers::file::uploadv1)
            .service(handlers::file::uploadv2)
            .service(api::nats::get_nasts_services())
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

async fn handler_sender(client: async_nats::Client, mut recv: mpsc::Receiver<String>) -> Result<(), async_nats::Error> {
    loop {
        while let Some(msg) = recv.recv().await {
            println!("message received: {}", msg);
            match client.publish("easydev2.publish", msg.into()).await {
                Ok(()) => println!("successfully published message"),
                Err(e) => println!("error: {:?}", e)
            }
        }
    }
}

async fn send_to_nats(client: async_nats::Client) -> Result<(), async_nats::Error> {
    loop {
        for subject in ["easydev.topic1", "easydev.topic2", "easydev.topic3"] {
            client.publish(subject, "hello from easydev".into()).await?;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        info!("completed sending cycle")
    }
}

async fn receive_from_nats(client: async_nats::Client) -> Result<(), async_nats::Error> {
    let mut subscription = client.subscribe("easydev.*").await?;
    loop {
        while let Some(msg) = subscription.next().await {
            println!("{:?} received message on {:?}", std::str::from_utf8(&msg.payload), &msg.subject)
        }
    }
}


async fn handle_nats_messages(client: async_nats::Client) -> Result<(), async_nats::Error> {
    let mut subscription = client.subscribe("easydev2.*").await?;
    loop {
        while let Some(msg) = subscription.next().await {
            println!("{:?} received message on {:?}", std::str::from_utf8(&msg.payload), &msg.subject)
        }
    }
}