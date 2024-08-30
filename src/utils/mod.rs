use cron::Schedule;
use std::str::FromStr;
use chrono::Utc;


pub mod db;
pub mod jwt;
pub mod guard;
pub mod queue;

fn Scheduler() {
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