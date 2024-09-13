use libsql::Database;
use sqlx::PgPool;
use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};

use crate::chatserver::server::ChatServerHandle;

pub struct AppState {
    pub pool: Mutex<PgPool>,
    pub db_client: Arc<Database>,
    pub nats_client: Option<Mutex<async_nats::Client>>,
    pub sender: Mutex<mpsc::Sender<String>>,
    pub server_tx: ChatServerHandle,
}