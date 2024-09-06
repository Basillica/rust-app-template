use sqlx::PgPool;
use tokio::sync::mpsc;
use std::sync::Mutex;

pub struct AppState {
    pub pool: Mutex<PgPool>,
    pub nats_client: Option<Mutex<async_nats::Client>>,
    pub sender: Mutex<mpsc::Sender<String>>,
    pub db_conn_str: String,
    pub jwt_secret: String,
    pub jwt_subjet: String,
    pub jwt_aud: String,
    pub jwt_iss: String,
}