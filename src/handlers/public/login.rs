use actix_web::{ post, web,HttpResponse, Responder};
use crate::{models::{auth, errors::HttpError, state}, utils::{db::users::getByEmail, jwt::jwt::encode}};


#[post("/login")]
async fn login(app_data: web::Data<state::AppState>, req: web::Json<auth::LoginReq>) -> Result<impl Responder,  HttpError>  {
    match getByEmail(req.email.clone(), &app_data.pool.lock().unwrap()).await {
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