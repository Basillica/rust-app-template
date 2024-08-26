use actix_web::{web, post, HttpResponse, Responder};
use serde::Deserialize;

use crate::models::{errors::HttpError, state::AppState};


#[derive(Debug, Deserialize)]
struct NatsReq {
    message: String
}


#[post("/publish")]
async fn publish_message(app_state: web::Data<AppState>, req: web::Json<NatsReq>) -> Result<impl Responder, HttpError> {
    app_state.sender.lock().unwrap().send(req.0.message.to_string()).await.unwrap();
    // let conn = app_state.nats_client.lock().unwrap();
    // match conn.publish("easydev2.publish", req.0.message.into()).await {
    //     Ok(()) => Ok(HttpResponse::Ok().body("message published successfully!")),
    //     Err(e) => {
    //         println!("error: {:?}", e);
    //         Err(HttpError::NatsError)
    //     }
    // }
    Ok(HttpResponse::Ok().body("message published successfully!"))
}