use actix_ws::Message;
use futures_util::StreamExt;
use actix_web::{ get, web, Error, HttpRequest, HttpResponse, Responder};
use tokio::task;
use crate::{chatserver::handler, models::state::AppState};


#[get("/v1/{room_id}/{user_id}")]
pub async fn chat_ws(app_data: web::Data<AppState>, req: HttpRequest, stream: web::Payload, path: web::Path<(String, String)>) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;
    // spawn websocket handler (and don't await it) so that the response is returned immediately
    let (room_id, user_id) = path.into_inner();
    task::spawn_local(handler::chat_ws(app_data.clone(), session, msg_stream, room_id, user_id));
    Ok(res)
}


#[get("/v2")]
pub async fn ws(req: HttpRequest, body: web::Payload) -> actix_web::Result<impl Responder> {
    let (resp, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;
    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return
                    }
                },
                Message::Text(msg) => println!("we have recieved the message {msg}"),
                _ => break
            }
        }

        let _ = session.close(None).await;
    });

    Ok(resp)
}