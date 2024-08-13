use actix_web::{post, web, HttpResponse, Responder};
use crate::{models::{auth::UserModel, errors, state}, utils::db::users};

#[post("/user/add")]
async fn add_user(app_data: web::Data<state::AppState>, user: web::Json<UserModel>) -> Result<impl Responder,  errors::HttpError> {
    let id = uuid::Uuid::new_v4().to_string();
    match users::insert(user.0, &app_data.pool, &id).await {
        Ok(_) => Ok(HttpResponse::Ok().body(format!("the user with the id {} has been successfully created!", id))),
        Err(e) => {
            println!("{:?}", e);
            Err(errors::HttpError::InternalError)
        },
    }
}