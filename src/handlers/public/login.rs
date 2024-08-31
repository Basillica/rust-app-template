use actix_web::{ post, web,HttpResponse, Responder};
use crate::{models::{auth, errors::HttpError, state}, utils::{db::users::get_by_email, jwt::jwt::encode}};


#[post("/login")]
async fn login(app_data: web::Data<state::AppState>, req: web::Json<auth::LoginReq>) -> Result<impl Responder,  HttpError>  {
    match get_by_email(req.email.clone(), &app_data.pool.lock().unwrap()).await {
        Ok(user) => {
            let token = encode(user);
            Ok(HttpResponse::Ok().body(token))
        },
        Err(e) => {
            println!("{:?}", e);
            Err(HttpError::Unauthorized)
        }
    }
}