use actix_web::{get, web, Responder};
use crate::{models::{errors, state::AppState}, utils::db::users};

#[get("/user/{id}")]
async fn get_user(app_data: web::Data<AppState>, id: web::Path<String>) -> Result<impl Responder,  errors::HttpError> {
    match users::get(id.to_string(), &app_data.pool.lock().unwrap()).await {
        Ok(u) => Ok(web::Json(u)),
        Err(e) => {
            println!("{:?}", e);
            Err(errors::HttpError::InternalError)
        },
    }
}