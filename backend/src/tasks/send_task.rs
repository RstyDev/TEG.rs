use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use futures_util::{stream::SplitSink};
use structs::Player;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::run::Room;

pub async fn send_task(params: SendParams) {}

pub struct SendParams {
    pub this_player: Arc<Mutex<Option<Player>>>,
    pub arc_rooms: Arc<Mutex<HashMap<Uuid, Room>>>,
    pub send: SplitSink<WebSocket, Message>,
}