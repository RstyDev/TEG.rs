use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use futures_util::{StreamExt, stream::SplitStream};
use macros::{arc_mutex, hashmap};
use structs::{Map, MessageDTO, Player};
use tokio::sync::{Mutex, broadcast};
use uuid::Uuid;

use crate::run::{AppState, Room};

pub async fn receive_task(params: ReceiveParams) {
    let ReceiveParams { mut recv, state, this_user } = params;
    while let Some(Ok(Message::Text(msg))) = recv.next().await {
        match serde_json::from_str::<MessageDTO>(msg.as_str()) {
            Ok(msg_dto) => match msg_dto {
                MessageDTO::AddPlayer { player } => match player.role() {
                    structs::PlayerRole::Master => {
                        let new_room = Room { id: Uuid::new_v4(), master: Some(player.clone()), players: Arc::new(Mutex::new(HashMap::new())), countries: Map::get(), status: arc_mutex!((hashmap!{})), tx: broadcast::channel::<String>(20).0 };
                        {
                            state.rooms.lock().await.insert(new_room.id, new_room);
                        }
                        *this_user.lock().await = Some(player);
                    },
                    structs::PlayerRole::Player{ room} => {
                        let room_found = state.rooms.lock().await.get(&room).cloned();
                        if let Some(room) = room_found {
                            room.players.lock().await.insert(player.id(), player.clone());
                            *this_user.lock().await = Some(player);
                        } else {
                            println!("Room with id {} not found for player {}", room, player.name());
                        }
                    },
                },
                MessageDTO::MakeMove { room_id ,player_id, from, to, troops } => {
                    
                },
                MessageDTO::UpdateState { status, room_id } => (),
                MessageDTO::StartGame { room_id } => (),
                MessageDTO::MissionCompleted { room_id } => (),
            },
            Err(e) => println!("Error deserializando mensaje: {e}"),
        }
    }
}

pub struct ReceiveParams {
    pub recv: SplitStream<WebSocket>,
    pub state: AppState,
    pub this_user: Arc<Mutex<Option<Player>>>,
}