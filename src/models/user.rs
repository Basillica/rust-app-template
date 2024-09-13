use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserModel {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub profile_uri: String,
    pub status: String,
    pub password: String,
    pub email: String,
    pub created_at: String,
    pub id: String,
}

impl Default for UserModel {
    fn default() -> Self {
        UserModel {
            firstname: String::new(),
            lastname: String::new(),
            email: String::new(),
            password: String::new(),
            username: String::new(),
            profile_uri: String::new(),
            created_at: String::new(),
            status: String::new(),
            id: String::new(),
        }
    }
}