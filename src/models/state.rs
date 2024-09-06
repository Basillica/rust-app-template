use sqlx::PgPool;
use tokio::sync::mpsc;
use std::sync::Mutex;

pub struct AppState {
    pub pool: Mutex<PgPool>,
    pub nats_client: Option<Mutex<async_nats::Client>>,
    pub sender: Mutex<mpsc::Sender<String>>,
}