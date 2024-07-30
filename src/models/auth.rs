use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginReq {
    pub password: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserModel {
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct PathParams {
    pub name: String,
    pub id: String,
    pub email: String,
}