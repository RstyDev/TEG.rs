use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, stream::SplitSink};
use structs::{MessageDTO, Player};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::run::{Room, RoomPlayer};

pub async fn send_task(params: SendParams) {
    let SendParams { this_player, arc_rooms, mut send } = params;
    'outer: loop {
        let mut player_opt;
        loop {
            player_opt = this_player.lock().await.clone();
            if player_opt.is_some() {println!("Player is {:#?}",player_opt); break; }
        }
         

        if let Some(player) = player_opt {
            let mut rooms_lock;
            loop {
                rooms_lock = arc_rooms.lock().await.get(&player.room_id).cloned();
                if rooms_lock.is_some() { println!("Room is {:#?}", rooms_lock); break; }
            }
            if let Some(room) = rooms_lock {
                let mut rx = room.tx.subscribe();
                let users = room.players.lock().await.values().cloned().collect::<Vec<_>>();

                //TODO ver si se necesita enviar el estado completo o solo el cambio

                while let Ok(msg) = rx.recv().await {
                    match msg {
                        crate::run::SenderMessage::Move { room_id, player_id, from, to, troops } => {
                            let player_lock;
                            {
                                player_lock = this_player.lock().await.clone();
                            }
                            if let Some(player) = player_lock {
                                let room_lock;
                                {
                                    room_lock = arc_rooms.lock().await.get(&player.room_id).cloned();
                                }
                                if let Some(room) = room_lock {
                                    let updated_status = room.status.lock().await.clone().values().cloned().collect::<Vec<_>>();
                                     if let Err(e) = send.send(Message::Text(serde_json::to_string(&MessageDTO::UpdateState { statuses: updated_status }).unwrap_or_default().into())).await {
                                        println!("Error sending message: {e}");
                                        break 'outer;
                                    }
                                } else {
                                    println!("Room not found for player in move message");
                                }
                            } else {
                                println!("Player not found for move message");
                            }
                        },
                        crate::run::SenderMessage::UpdateState { room_id } => (),
                        crate::run::SenderMessage::StartGame { room_id } => (),
                    }
                }

            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

pub struct SendParams {
    pub this_player: Arc<Mutex<Option<RoomPlayer>>>,
    pub arc_rooms: Arc<Mutex<HashMap<Uuid, Room>>>,
    pub send: SplitSink<WebSocket, Message>,
}