use cron::Schedule;
use chrono::Utc;
use sqlx::{ postgres::PgPoolOptions, Executor, Pool, Postgres};
use std::sync::Mutex;
use std::{env, str::FromStr};


pub mod db;
pub mod jwt;
pub mod guard;
pub mod queue;
pub mod sqlite;


#[allow(dead_code)]
pub fn scheduler() {
    let schedule = Schedule::from_str("0/10 * * * * *").unwrap();
    for datetime in schedule.upcoming(Utc).take(10) {
        println!("->>>> {}", datetime);
    }
    
    // .next().unwrap()
    let mut next_run = schedule.upcoming(Utc).next().unwrap();
    loop {
        if next_run <= chrono::Local::now() {
            // execute my job
            println!("executing cron job ......");
            // reschedule the next job
            schedule.upcoming(Utc);

            // reset
            next_run = schedule.upcoming(Utc).next().unwrap();
        }

        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}

pub async fn get_db_pool(path: &str) -> Mutex<Pool<Postgres>> {
    let db_conn_string = env::var("DATABASE_CONNECTION_STRING").expect("the database connection string was not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_conn_string)
        .await.expect("could not exstablish a connection to the database");

    let _ = pool.execute(path)
        .await
        .expect("there was some error executing schema");

    Mutex::new(pool)
}