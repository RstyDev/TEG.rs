use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, stream::SplitSink};
use rand::seq::IteratorRandom;
use structs::{ResponseDTO, RoomMaster};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::run::{Room, RoomPlayer};

pub async fn send_task(params: SendParams) {
    let SendParams {
        this_player,
        arc_rooms,
        mut send,
    } = params;
    loop {
        let mut player_opt;
        'inner: loop {
            player_opt = this_player.lock().await.clone();
            if player_opt.is_some() {
                // println!("Player is {:#?}",player_opt);
                break 'inner;
            }
        }
        //  println!("Player found");

        if let Some(player) = player_opt {
            let mut rooms_lock;
            'inner: loop {
                rooms_lock = arc_rooms.lock().await.get(&player.room_id).cloned();
                if rooms_lock.is_some() {
                    //    println!("Room is {:#?}", rooms_lock);
                    break 'inner;
                }
            }
            // println!("Room found");
            if let Some(room) = rooms_lock {
                let mut rx = room.tx.subscribe();
                let users;
                {
                    users = room.players.lock().await.clone();
                }
                // println!("Suscribed");

                //TODO ver si se necesita enviar el estado completo o solo el cambio
                if let Err(e) = send
                    .send(Message::Text(
                        serde_json::to_string(&ResponseDTO::CompleteUpdate {
                            room: RoomMaster {
                                room_id: room.id,
                                master: room.master.clone().unwrap(),
                            },
                            players: users.to_owned(),
                            status: room.status.lock().await.to_owned(),
                            this_player: player.player.id(),
                        })
                        .unwrap()
                        .into(),
                    ))
                    .await
                {
                    println!("Error sending message: {e}");
                }

                // println!("Listening");
                while let Ok(msg) = rx.recv().await {
                    // println!("Message received");
                    match msg {
                        crate::run::SenderMessage::Move => {
                            let player_lock;
                            {
                                player_lock = this_player.lock().await.clone();
                            }
                            if let Some(player) = player_lock {
                                let room_lock;
                                {
                                    room_lock =
                                        arc_rooms.lock().await.get(&player.room_id).cloned();
                                }
                                if let Some(room) = room_lock {
                                    let updated_status = room
                                        .status
                                        .lock()
                                        .await
                                        .clone()
                                        .values()
                                        .cloned()
                                        .collect::<Vec<_>>();
                                    if let Err(e) = send
                                        .send(Message::Text(
                                            serde_json::to_string(&ResponseDTO::UpdateState {
                                                statuses: updated_status,
                                            })
                                            .unwrap_or_default()
                                            .into(),
                                        ))
                                        .await
                                    {
                                        println!("Error sending message: {e}");
                                        continue;
                                    }
                                } else {
                                    println!("Room not found for player in move message");
                                }
                            } else {
                                println!("Player not found for move message");
                            }
                        }
                        crate::run::SenderMessage::UpdateState => {
                            let player_lock;
                            {
                                player_lock = this_player.lock().await.clone();
                            }
                            if let Some(player) = player_lock {
                                let room_lock;
                                {
                                    room_lock =
                                        arc_rooms.lock().await.get(&player.room_id).cloned();
                                }
                                if let Some(room) = room_lock {
                                    let players = room.players.lock().await.to_owned();
                                    let starter = *players.keys().choose(&mut rand::rng()).unwrap();
                                    if let Err(e) = send
                                        .send(Message::Text(
                                            serde_json::to_string(&ResponseDTO::GameStarted {
                                                room: RoomMaster {
                                                    room_id: player.room_id,
                                                    master: player.player.id(),
                                                },
                                                players,
                                                status: room.status.lock().await.to_owned(),
                                                starter,
                                            })
                                            .unwrap_or_default()
                                            .into(),
                                        ))
                                        .await
                                    {
                                        println!("Error sending message: {e}");
                                        continue;
                                    }
                                } else {
                                    println!("Room not found for player in move message");
                                }
                            } else {
                                println!("Player not found for move message");
                            }
                        }
                        crate::run::SenderMessage::StartGame => {
                            let players = room.players.lock().await.to_owned();
                            let status = room.status.lock().await.to_owned();
                            let starter = *players.keys().choose(&mut rand::rng()).unwrap();
                            let master = players.get(room.master.as_ref().unwrap()).unwrap().id();
                            if let Err(e) = send
                                .send(
                                    serde_json::to_string(&ResponseDTO::GameStarted {
                                        room: RoomMaster {
                                            room_id: room.id,
                                            master,
                                        },
                                        players,
                                        status,
                                        starter,
                                    })
                                    .expect("Formatting err")
                                    .into(),
                                )
                                .await
                            {
                                println!("Error starting game: {}", e)
                            } else {
                                println!("Game started message sent")
                            }
                        }
                        crate::run::SenderMessage::LoggedIn => {
                            let this_player =
                                this_player.lock().await.as_ref().unwrap().player.clone();
                            println!(
                                "Player logged in as {:#?}, sending initial state",
                                this_player
                            );
                            let users = room.players.lock().await;
                            let master = users.get(room.master.as_ref().unwrap()).cloned().unwrap();
                            if let Err(e) = send
                                .send(
                                    serde_json::to_string(&ResponseDTO::LoggedIn {
                                        users: users.to_owned(),
                                        this_player,
                                        room: RoomMaster {
                                            room_id: room.id,
                                            master: master.id(),
                                        },
                                    })
                                    .expect("Error formatting")
                                    .into(),
                                )
                                .await
                            {
                                println!("Error sending login message: {e}");
                                continue;
                            } else {
                                println!("Message sent - 107")
                            }
                        }
                    }
                }
            }
        }
        //tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

pub struct SendParams {
    pub this_player: Arc<Mutex<Option<RoomPlayer>>>,
    pub arc_rooms: Arc<Mutex<HashMap<Uuid, Room>>>,
    pub send: SplitSink<WebSocket, Message>,
}
