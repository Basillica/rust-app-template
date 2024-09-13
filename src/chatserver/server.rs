
//! A multi-room chat server.

use std::{
    collections::HashMap,
    io,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tracing::info;
use tokio::sync::{mpsc, oneshot};
use crate::models::room::PersonalRoom;

use super::types::{ChatServer, ConnId, Msg, RoomId, Command};


impl ChatServer {
    pub fn new(rooms: Vec<PersonalRoom>) -> (Self, ChatServerHandle) {
        // create empty server
        // create default room
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        (
            Self {
                sessions: HashMap::new(),
                rooms,
                visitor_count: Arc::new(AtomicUsize::new(0)),
                cmd_rx,
            },
            ChatServerHandle { cmd_tx },
        )
    }

    /// Send message to users in a room.
    ///
    /// `skip` is used to prevent messages triggered by a connection also being received by it.
    async fn send_system_message(&self, room_id: &str, user_id: &str, msg: impl Into<Msg>) {
        let room = self.rooms.iter().find(|r| r.id == room_id);
        if let Some(room) = room {
            let msg = msg.into();
            for member_id in room.members.as_ref().unwrap() {
                if *member_id != user_id {
                    if let Some(tx) = self.sessions.get(member_id) {
                        // errors if client disconnected abruptly and hasn't been timed-out yet
                        let _ = tx.send(msg.clone());
                    }
                }
            }
        }
    }

    /// Send message to all other users in current room.
    ///
    /// `conn` is used to find current room and prevent messages sent by a connection also being
    /// received by it.
    async fn send_message(&self, room_id: ConnId, user_id: ConnId,  msg: impl Into<Msg>) {
        match self
            .rooms
            .iter()
            .find_map(|room| Some(room.id == room_id))
        {
            Some(_) => self.send_system_message(&room_id, &user_id, msg).await,
            None => println!("there are no matching rooms found"),
        };
    }

    /// Register new session and assign unique ID to this session
    async fn connect(&mut self, tx: mpsc::UnboundedSender<Msg>, user_id: &str, room_id: &str) {
        info!("Someone joined");

        // notify all users in same room
        self.send_system_message(room_id, "", format!("{} joined!", &user_id)).await;
        // register session with random connection ID
        self.sessions.insert(user_id.to_string(), tx);
        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_system_message(room_id, "", format!("Total visitors {count}"))
            .await;
    }

    /// Unregister connection from room map and broadcast disconnection message.
    async fn disconnect(&mut self, room_id: ConnId, user_id: ConnId) {
        println!("Someone disconnected");
        // send message to other users
        self.send_system_message(&room_id, &user_id, format!("{} disconnected!", &user_id))
            .await;
    }

    /// Returns list of created room names.
    fn list_rooms(&mut self) -> Vec<RoomId> {
        self.rooms.iter().map(|r| r.name.clone()).collect()
    }

    /// Join room, send disconnect message to old room send join message to new room.
    async fn join_room(&mut self, room_id: ConnId, user_id: ConnId) {
        let rrs = self
                    .rooms
                    .iter()
                    .filter(|room| room.members.is_some() && room.members.as_ref().unwrap().contains(&user_id));
        // send message to other users
        for room in rrs {
            self.send_system_message(&room.id, &user_id, "Someone disconnected")
                .await;
        }
        self.send_system_message(&room_id, &user_id, "Someone connected")
            .await;
    }

    pub async fn run(mut self) -> io::Result<()> {
        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                Command::Connect { conn_tx, res_tx, user_id, room_id } => {
                    self.connect(conn_tx, &user_id, &room_id).await;
                    let _ = res_tx.send(user_id);
                }

                Command::Disconnect { user_id, room_id } => {
                    self.disconnect(room_id, user_id).await;
                }

                Command::List { res_tx } => {
                    let _ = res_tx.send(self.list_rooms());
                }

                Command::Join { room_id, user_id, res_tx } => {
                    self.join_room(room_id, user_id).await;
                    let _ = res_tx.send(());
                }

                Command::Message { room_id, user_id, msg, res_tx } => {
                    self.send_message(room_id, user_id, msg).await;
                    let _ = res_tx.send(());
                }
            }
        }

        Ok(())
    }
}

/// Handle and command sender for chat server.
///
/// Reduces boilerplate of setting up response channels in WebSocket handlers.
#[derive(Debug, Clone)]
pub struct ChatServerHandle {
    cmd_tx: mpsc::UnboundedSender<Command>,
}

impl ChatServerHandle {
    /// Register client message sender and obtain connection ID.
    pub async fn connect(&self, conn_tx: mpsc::UnboundedSender<Msg>, user_id: &String, room_id: &String) {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.cmd_tx
            .send(Command::Connect { conn_tx, res_tx, user_id: user_id.to_string(), room_id: room_id.to_string() })
            .unwrap();

        // unwrap: chat server does not drop out response channel
        res_rx.await.unwrap();
    }

    /// List all created rooms.
    pub async fn list_rooms(&self) -> Vec<RoomId> {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.cmd_tx.send(Command::List { res_tx }).unwrap();

        // unwrap: chat server does not drop our response channel
        res_rx.await.unwrap()
    }

    /// Join `room`, creating it if it does not exist.
    pub async fn join_room(&self, room_id: ConnId, user_id: ConnId) {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.cmd_tx
            .send(Command::Join {
                room_id,
                user_id,
                res_tx,
            })
            .unwrap();

        // unwrap: chat server does not drop our response channel
        res_rx.await.unwrap();
    }

    /// Broadcast message to current room.
    pub async fn send_message(&self, room_id: &ConnId, user_id: &ConnId, msg: impl Into<Msg>) {
        let (res_tx, res_rx) = oneshot::channel();

        // unwrap: chat server should not have been dropped
        self.cmd_tx
            .send(Command::Message {
                msg: msg.into(),
                room_id: room_id.to_string(),
                user_id: user_id.to_string(),
                res_tx,
            })
            .unwrap();

        // unwrap: chat server does not drop our response channel
        res_rx.await.unwrap();
    }

    /// Unregister message sender and broadcast disconnection message to current room.
    pub fn disconnect(&self, room_id: ConnId, user_id: ConnId) {
        // unwrap: chat server should not have been dropped
        self.cmd_tx.send(Command::Disconnect { room_id, user_id }).unwrap();
    }
}
