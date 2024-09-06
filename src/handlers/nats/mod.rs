use actix_web::{web, post, HttpResponse, Responder};
use serde::Deserialize;

use crate::models::{errors::HttpError, state::AppState};


#[derive(Debug, Deserialize)]
struct NatsReq {
    message: String
}


#[post("/publish")]
async fn publish_message(app_state: web::Data<AppState>, req: web::Json<NatsReq>) -> Result<impl Responder, HttpError> {
    match &app_state.nats_client {
        // if no client is set, use the channel to send message
        None => {
            app_state.sender.lock().unwrap().send(req.0.message.to_string()).await.unwrap();
            Ok(HttpResponse::Ok().body("message published successfully!"))
        },
        // use the client to transmit message when present
        Some(client) => {
            let conn = client.lock().unwrap();
            match conn.publish("easydev2.publish", req.0.message.into()).await {
                Ok(()) => Ok(HttpResponse::Ok().body("message published successfully!")),
                Err(e) => {
                    println!("error: {:?}", e);
                    Err(HttpError::NatsError)
                }
            }
        }
    }
}