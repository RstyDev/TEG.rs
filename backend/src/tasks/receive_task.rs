use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use futures_util::{StreamExt, stream::SplitStream};
use macros::{arc_mutex, hashmap};
use structs::{Map, MessageDTO, Player};
use tokio::sync::{Mutex, broadcast};
use uuid::Uuid;

use crate::run::{AppState, Room, RoomPlayer, SenderMessage};

pub async fn receive_task(params: ReceiveParams) {
    let ReceiveParams { mut recv, state, this_user } = params;
    while let Some(Ok(Message::Text(msg))) = recv.next().await {
        match serde_json::from_str::<MessageDTO>(msg.as_str()) {
            Ok(msg_dto) => match msg_dto {
                MessageDTO::AddPlayer { player } => {
                    let room_send;
                    match player.role() {
                        structs::PlayerRole::Master => {
                            {
                                let new_room = Room { id: Uuid::new_v4(), master: Some(player.clone()), players: Arc::new(Mutex::new(HashMap::new())), countries: Map::get(), status: arc_mutex!((hashmap!{})), tx: broadcast::channel::<SenderMessage>(10).0 };
                                room_send = new_room.id;
                                state.rooms.lock().await.insert(room_send, new_room);
                            }
                            // println!("Room Created: {:#?}", state.rooms.lock().await);
                            *this_user.lock().await = Some(RoomPlayer { room_id: room_send, player });
                        },
                        structs::PlayerRole::Player{ room} => {
                            let mut rooms_lock = state.rooms.lock().await;
                            let room_found = rooms_lock.get(&room).cloned();
                            if let Some(room) = room_found {
                                room.players.lock().await.insert(player.id(), player.clone());
                                *this_user.lock().await = Some(RoomPlayer { room_id: room.id, player });
                                room_send = room.id;
                                rooms_lock.insert(room.id, room);
                            } else {
                                println!("Room with id {} not found for player {}", room, player.name());
                                continue;
                            }
                        },
                    }
                    loop {
                        if state.rooms.lock().await.get(&room_send).unwrap().tx.send(SenderMessage::LoggedIn).is_ok() {break;}
                    }
                    
                },
                MessageDTO::MakeMove { room_id ,player_id, from, to, troops } => {
                     if let Some(mut room) = state.rooms.lock().await.get(&room_id) {
                        let status = room.status.lock().await;
                        let attacker_status = status.get(&from);
                        let defender_status = status.get(&to);
                        if let (Some(attacker_status), Some(defender_status)) = (attacker_status, defender_status) {
                            if let Some(attacker_tokens) = &attacker_status.tokens {
                                if attacker_tokens.owner == player_id {
                                    println!("Player {} is attacking from {} to {} with {} troops, having {} tokens", player_id, from, to, troops, attacker_tokens.amount);
                                }else {
                                    println!("Player {} is not the owner of the attacking country {}", player_id, from);
                                }
                                if let Err(e) = room.tx.send(SenderMessage::Move) {
                                    println!("Error broadcasting move: {e}");
                                    break;
                                }
                                continue;
                            }
                        } else {
                            println!("Invalid move: attacker or defender status not found.");
                        }
                     }
                },
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
    pub this_user: Arc<Mutex<Option<RoomPlayer>>>,
}