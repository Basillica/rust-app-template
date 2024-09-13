use std::{
    pin::pin,
    time::{Duration, Instant},
};
use chrono::Local;
use uuid::Uuid;
use tracing::{info, debug, warn, error};
use actix_ws::AggregatedMessage;
use futures_util::{
    future::{select, Either},
    StreamExt as _,
};
use tokio::{sync::mpsc, time::interval};
use actix_web::web::Data;
use crate::{models::{message::PersonalMessage, state::AppState}, utils::sqlite::message};

use super::{server::ChatServerHandle, types::ConnId};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Echo text & binary messages received from the client, respond to ping messages, and monitor
/// connection health to detect network issues and free up resources.
pub async fn chat_ws(
    app_data: Data<AppState>, mut session: actix_ws::Session, 
    msg_stream: actix_ws::MessageStream, room_id: String, user_id: String
) {
    info!("connected");
    let mut name = None;
    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);
    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();
    // unwrap: chat server is not dropped before the HTTP server
    app_data.server_tx.connect(conn_tx, &user_id, &room_id).await;

    let msg_stream = msg_stream
        .max_frame_size(128 * 1024)
        .aggregate_continuations()
        .max_continuation_size(2 * 1024 * 1024);

    let mut msg_stream = pin!(msg_stream);

    let close_reason = loop {
        // most of the futures we process need to be stack-pinned to work with select()
        let tick = pin!(interval.tick());
        let msg_rx = pin!(conn_rx.recv());

        // TODO: nested select is pretty gross for readability on the match
        let messages = pin!(select(msg_stream.next(), msg_rx));

        match select(messages, tick).await {
            // commands & messages received from client
            Either::Left((Either::Left((Some(Ok(msg)), _)), _)) => {
                debug!("msg: {msg:?}");
                match msg {
                    AggregatedMessage::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        // unwrap:
                        session.pong(&bytes).await.unwrap();
                    }
                    AggregatedMessage::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }
                    AggregatedMessage::Text(text) => {
                        let m = PersonalMessage{
                            room_id: room_id.clone(), user_id: user_id.clone(),
                            message: text.trim().to_string(), created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                            media_uris: Some(vec![]), id: Uuid::new_v4().to_string()
                        };
                        match message::db::insert(m, &app_data.db_client).await {
                            Ok(_) => process_text_msg(&app_data.server_tx, &mut session, &text, &room_id, &user_id, &mut name).await,
                            Err(e) => println!("error: {:?}", e),
                        };
                        
                    }
                    AggregatedMessage::Binary(_bin) => {
                        warn!("unexpected binary message");
                    }
                    AggregatedMessage::Close(reason) => break reason,
                }
            }

            // client WebSocket stream error
            Either::Left((Either::Left((Some(Err(err)), _)), _)) => {
                error!("{}", err);
                break None;
            }

            // client WebSocket stream ended
            Either::Left((Either::Left((None, _)), _)) => break None,
            // chat messages received from other room participants
            Either::Left((Either::Right((Some(chat_msg), _)), _)) => {
                session.text(chat_msg).await.unwrap();
            }
            // all connection's message senders were dropped
            Either::Left((Either::Right((None, _)), _)) => unreachable!(
                "all connection message senders were dropped; chat server may have panicked"
            ),
            // heartbeat internal tick
            Either::Right((_inst, _)) => {
                // if no heartbeat ping/pong received recently, close the connection
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    info!(
                        "client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting"
                    );
                    break None;
                }
                // send heartbeat ping
                let _ = session.ping(b"").await;
            }
        };
    };

    app_data.server_tx.disconnect(room_id, user_id);
    // attempt to close connection gracefully
    let _ = session.close(close_reason).await;
}


async fn process_text_msg(
    chat_server: &ChatServerHandle, session: &mut actix_ws::Session, text: &str,
    room_id: &ConnId, user_id: &ConnId, name: &mut Option<String>
) {
    // strip leading and trailing whitespace (spaces, newlines, etc.)
    let msg = text.trim();

    // we check for /<cmd> type of messages
    if msg.starts_with('/') {
        let mut cmd_args = msg.splitn(2, ' ');

        // unwrap: we have guaranteed non-zero string length already
        match cmd_args.next().unwrap() {
            "/list" => {
                info!("room_id {room_id}: listing rooms");
                let rooms = chat_server.list_rooms().await;
                for room in rooms {
                    session.text(room).await.unwrap();
                }
            }

            "/join" => match cmd_args.next() {
                Some(room) => {
                    info!("room_id {room_id}: joining room {room}");
                    chat_server.join_room(room_id.to_string(), user_id.to_string()).await;
                    session.text(format!("joined {room}")).await.unwrap();
                }
                None => {
                    session.text("!!! room name is required").await.unwrap();
                }
            },

            "/name" => match cmd_args.next() {
                Some(new_name) => {
                    info!("room_id {room_id}: setting name to: {new_name}");
                    name.replace(new_name.to_owned());
                }
                None => {
                    session.text("!!! name is required").await.unwrap();
                }
            },

            _ => {
                session
                    .text(format!("!!! unknown command: {msg}"))
                    .await
                    .unwrap();
            }
        }
    } else {
        // prefix message with our name, if assigned
        let msg = match name {
            Some(ref name) => format!("{name}: {msg}"),
            None => msg.to_owned(),
        };
        chat_server.send_message(room_id, user_id,  msg).await
    }
}