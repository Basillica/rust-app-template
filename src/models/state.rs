use std::sync::Mutex;

pub struct AppState {
   pub state: Mutex<String>
}