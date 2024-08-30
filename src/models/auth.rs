use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginReq {
    pub password: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct UserModel {
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub email: String,
    pub id: String,
}

impl Default for UserModel {
    fn default() -> Self {
        UserModel {
            firstname: String::new(),
            lastname: String::new(),
            email: String::new(),
            password: String::new(),
            id: String::new(),
        }
    }
}

#[derive(Deserialize)]
pub struct PathParams {
    pub name: String,
    pub id: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub custom_claim: UserModel,
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: u64,
}