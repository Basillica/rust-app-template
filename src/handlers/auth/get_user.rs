use actix_web::{get, web, Responder};
use crate::models::{auth, errors};

#[get("/user")]
async fn get_user(path: web::Query<auth::PathParams>) -> Result<impl Responder,  errors::HttpError> {
    // make a query to the db
    // if error, return Err(erros::HttpError{message: ""})
    let user = auth::UserModel{
        firstname: "tonie".to_string(),
        lastname: "etienne".to_string(),
        password: "*****************".to_string(),
        email: path.email.clone()
    };
    Ok(web::Json(user))
}