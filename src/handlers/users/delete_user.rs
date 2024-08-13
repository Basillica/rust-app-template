use actix_web::{delete, web, HttpResponse, Responder};
use crate::{models::{errors, state}, utils::db::users};

#[delete("/user/{id}")]
async fn delete_user(app_data: web::Data<state::AppState>,  id: web::Path<String>) -> Result<impl Responder,  errors::HttpError> {
    match users::delete(id.to_string(),  &app_data.pool).await {
        Ok(_) => Ok(HttpResponse::Ok().body(format!("the user with the id {} has been successfully deleted!", id))),
        Err(e) => {
            println!("{:?}", e);
            Err(errors::HttpError::InternalError)
        },
    }
}