use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use futures_util::{StreamExt, stream::SplitStream};
use macros::{arc_mutex, hashmap};
use rand::{
    RngExt, SeedableRng,
    rngs::{StdRng, SysRng},
};
use structs::{CStatus, MAP, MessageDTO, Tokens};
use tokio::sync::{Mutex, broadcast};
use uuid::Uuid;

use crate::{
    run::{AppState, Room, RoomPlayer, SenderMessage},
    structs::Mission,
};

pub async fn receive_task(params: ReceiveParams) {
    let ReceiveParams {
        mut recv,
        state,
        this_user,
    } = params;
    while let Some(Ok(Message::Text(msg))) = recv.next().await {
        match serde_json::from_str::<MessageDTO>(msg.as_str()) {
            Ok(msg_dto) => match msg_dto {
                MessageDTO::AddPlayer { player } => {
                    println!("Logging in as {:#?}", player.role());
                    let room_send;
                    match player.role() {
                        structs::PlayerRole::Master => {
                            {
                                let pl_id = player.id();
                                let pl = player.clone();
                                let value = hashmap!({pl_id:pl});
                                let new_room = Room {
                                    id: Uuid::new_v4(),
                                    master: Some(pl_id),
                                    players: Arc::new(Mutex::new(value)),
                                    status: arc_mutex!((hashmap! {})),
                                    tx: broadcast::channel::<SenderMessage>(10).0,
                                    missions: arc_mutex!((HashMap::new())),
                                };
                                room_send = new_room.id;
                                state.rooms.lock().await.insert(room_send, new_room);
                            }
                            // println!("Room Created: {:#?}", state.rooms.lock().await);

                            *this_user.lock().await = Some(RoomPlayer {
                                room_id: room_send,
                                player,
                            });
                        }
                        structs::PlayerRole::Player { room } => {
                            let mut rooms_lock = state.rooms.lock().await;
                            let room_found = rooms_lock.get(&room).cloned();
                            println!("Logging in to room {} with player {:#?}", room, player);
                            if let Some(room) = room_found {
                                room.players
                                    .lock()
                                    .await
                                    .insert(player.id(), player.clone());
                                *this_user.lock().await = Some(RoomPlayer {
                                    room_id: room.id,
                                    player,
                                });
                                room_send = room.id;
                                rooms_lock.insert(room.id, room);
                            } else {
                                println!(
                                    "Room with id {} not found for player {}",
                                    room,
                                    player.name()
                                );
                                continue;
                            }
                        }
                    }
                    loop {
                        // println!("-.-49");
                        if state
                            .rooms
                            .lock()
                            .await
                            .get(&room_send)
                            .unwrap()
                            .tx
                            .send(SenderMessage::LoggedIn)
                            .is_ok()
                        {
                            println!("Message sent with player {:#?}", this_user.lock().await);
                            break;
                        }
                        // println!("-.-54");
                    }
                }
                MessageDTO::MakeMove {
                    room_id,
                    player_id,
                    from,
                    to,
                    troops,
                } => {
                    if let Some(room) = state.rooms.lock().await.get(&room_id) {
                        let status = room.status.lock().await;
                        let attacker_status = status.get(&from);
                        let defender_status = status.get(&to);
                        if let (Some(attacker_status), Some(defender_status)) =
                            (attacker_status, defender_status)
                        {
                            if let Some(attacker_tokens) = &attacker_status.tokens {
                                if attacker_tokens.owner == player_id {
                                    println!(
                                        "Player {} is attacking from {} to {} with {} troops, having {} tokens",
                                        player_id, from, to, troops, attacker_tokens.amount
                                    );
                                } else {
                                    println!(
                                        "Player {} is not the owner of the attacking country {}",
                                        player_id, from
                                    );
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
                }
                MessageDTO::StartGame { room_id } => {
                    if let Some(room) = state.rooms.lock().await.get(&room_id) {
                        let mut state_vec = HashMap::new();
                        let mut rng = StdRng::try_from_rng(&mut SysRng).unwrap();
                        let mut users_copy = room.players.lock().await;
                        let mut missions = room.missions.lock().await;
                        for (id, user) in &mut *users_copy {
                            user.grant_troops(5);

                            //TODO! Add missions for players
                        }
                        let mut indexes_mut =
                            users_copy.iter().map(|u| u.0).cloned().collect::<Vec<_>>();

                        let indexes = indexes_mut.clone();
                        dbg!(&indexes_mut);
                        for (id, _) in MAP.iter() {
                            let i = indexes_mut.remove(match indexes_mut.len() {
                                0 => break,
                                1 => 0,
                                len => rng.random_range(0..len),
                            });

                            // user.grant_troops(5);
                            state_vec.insert(
                                *id,
                                CStatus {
                                    country_id: *id,
                                    location: MAP.get(id).unwrap().name().get_point(),
                                    tokens: Some(Tokens {
                                        owner: users_copy[&i].id(),
                                        amount: 1,
                                    }),
                                },
                            );
                            if indexes_mut.is_empty() {
                                indexes_mut = indexes.clone();
                            }
                        }
                        dbg!(&state_vec);
                        *room.status.lock().await = state_vec.to_owned();
                        // for (country_id,_) in map.0.clone() {
                        //     let i = indexes_mut.remove(rng.random_range(0..indexes_mut.len()));
                        //     state_vec.insert(country_id,CStatus{ country_id, location: get_point(map.0.get(&country_id).unwrap().name()), tokens: Some(Tokens { owner: users_copy[i].id(), amount: 1 }) });
                        //     if indexes_mut.is_empty() {
                        //         indexes_mut = indexes.clone();
                        //     }
                        // }
                        // console_dbg!(&state_vec);

                        // status.set(state_vec);
                        loop {
                            if room.tx.send(SenderMessage::StartGame).is_ok() {
                                break;
                            }
                        }
                    }
                }
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
