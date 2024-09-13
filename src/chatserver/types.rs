use std::{collections::{HashMap, HashSet}, sync::{atomic::AtomicUsize, Arc}};
use tokio::sync::{oneshot, mpsc};

use crate::models::room::PersonalRoom;

/// Connection ID.
pub type ConnId = String;

/// Room ID.
pub type RoomId = String;

/// Message sent to a room/client.
pub type Msg = String;

/// A multi-room chat server.
///
/// Contains the logic of how connections chat with each other plus room management.
///
/// Call and spawn [`run`](Self::run) to start processing commands.
#[derive(Debug)]
pub struct ChatServer {
    /// Map of connection IDs to their message receivers.
    pub sessions: HashMap<ConnId, mpsc::UnboundedSender<Msg>>,
    /// Map of room name to participant IDs in that room.
    pub rooms: Vec<PersonalRoom>,
    /// Tracks total number of historical connections established.
    pub visitor_count: Arc<AtomicUsize>,
    /// Command receiver.
    pub cmd_rx: mpsc::UnboundedReceiver<Command>,
}

/// A command received by the [`ChatServer`].
#[derive(Debug)]
pub enum Command {
    Connect {
        conn_tx: mpsc::UnboundedSender<Msg>,
        res_tx: oneshot::Sender<ConnId>,
        user_id: ConnId,
        room_id: ConnId,
    },
    Disconnect {
        user_id: ConnId,
        room_id: ConnId,
    },
    List {
        res_tx: oneshot::Sender<Vec<RoomId>>,
    },
    Join {
        user_id: ConnId,
        room_id: ConnId,
        res_tx: oneshot::Sender<()>,
    },
    Message {
        msg: Msg,
        res_tx: oneshot::Sender<()>,
        user_id: ConnId,
        room_id: ConnId,
    },
}