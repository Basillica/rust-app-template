use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PersonalRoom {
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub members: Option<Vec<String>>,
    pub media_uris: Option<Vec<String>>,
    pub id: String,
}

impl Default for PersonalRoom {
    fn default() -> Self {
        PersonalRoom {
            name: String::new(),
            description: String::new(),
            created_at: String::new(),
            id: String::new(),
            members: None,
            media_uris: None,
        }
    }
}