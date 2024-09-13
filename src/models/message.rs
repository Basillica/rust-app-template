use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PersonalMessage {
    pub room_id: String,
    pub user_id: String,
    pub message: String,
    pub created_at: String,
    pub media_uris: Option<Vec<String>>,
    pub id: String,
}


impl Default for PersonalMessage {
    fn default() -> Self {
        PersonalMessage {
            room_id: String::new(),
            user_id: String::new(),
            message: String::new(),
            created_at: String::new(),
            id: String::new(),
            media_uris: None,
        }
    }
}