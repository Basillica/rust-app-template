use actix_web::{get, web, Responder};
use crate::{models::{errors, state}, utils::db::users};

#[get("/users")]
async fn get_users(app_data: web::Data<state::AppState>) -> Result<impl Responder,  errors::HttpError> {
    match users::getall(&app_data.pool).await {
        Ok(res) => Ok(web::Json(res)),
        Err(e) => {
            println!("{:?}", e);
            Err(errors::HttpError::InternalError)
        },
    }
}