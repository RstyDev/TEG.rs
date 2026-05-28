use std::{collections::HashMap, sync::Arc};
use futures::channel::mpsc::UnboundedSender;
use gloo_net::websocket::Message;
use rand::prelude::*;
use structs::{CStatus, Map, Player, PlayerRole, RoomMaster, Tokens};
use sycamore::prelude::*;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;

use crate::{app::get_point, libs::copy_to_clipboard, structs::{AppStatus, Notification}};

#[component(inline_props)]
pub fn Lobby(map: Arc<Map>, users: Signal<HashMap<Uuid, Player>>,status: Signal<HashMap<Uuid,CStatus>>, send: Signal<Option<UnboundedSender<Message>>>, notification: Signal<Notification>, room: Signal<Option<RoomMaster>>, app_status: Signal<AppStatus>, this_player: Signal<Option<Player>>) -> View {
    let users_sel = create_selector(move||{
        let mut users = users.get_clone().values().cloned().collect::<Vec<_>>();
        if let Some(RoomMaster{ master, .. }) = room.get_clone(){
            users.push(master);
        }
        users
    });

    
    view!{
        p(){"Jugadores actuales:"}
        Keyed(
            list=users_sel,
            view=|player| view!{
                p() { (format!("{}: {}", player.id(), player.name())) }
            },
            key = |player|player.id()
        )
        (match this_player.get_clone().map(|player|player.role()).unwrap_or(PlayerRole::Player{ room: Uuid::nil() }) {
            PlayerRole::Master => {
                let map = map.clone();
                view!{
                    (match room.get_clone() {
                        Some(room_master) => {
                            let c2 = room_master.room_id.clone();
                            view!{
                                p(){(format!("Codigo de sala: {}", room_master.room_id))}
                                button(on:click=move |MouseEvent| {
                                    let c2 = c2;
                                    spawn_local(async move {
                                        if let Err(e) = copy_to_clipboard(c2.to_string().as_str()).await {
                                            notification.set(Notification::Error(format!("Error copiando al portapapeles: {:#?}", e)));
                                        } else {
                                            notification.set(Notification::Info("Codigo copiado al portapapeles".to_string()));
                                        }
                                    });
                                }){"Copiar codigo"}
                            }
                        },
                        None => view!{},
                    })
                    button(on:click=move|ev:MouseEvent|{
                        ev.prevent_default();
                        let indexes = users.get_clone().keys().into_iter().enumerate().map(|(i,_)|i).collect::<Vec<_>>();
                        // console_log!("indexes: {:?}", indexes);
                        if !indexes.is_empty(){
                            let mut indexes_mut = indexes.clone();
                            let mut state_vec = HashMap::new();
                            let mut rng = rand::rng();
                            let users_copy = users_sel.get_clone();
                            for (country_id,_) in map.0.clone() {
                                let i = indexes_mut.remove(rng.random_range(0..indexes_mut.len()));
                                state_vec.insert(country_id,CStatus{ country_id, location: get_point(map.0.get(&country_id).unwrap().name()), tokens: Some(Tokens { owner: users_copy[i].id(), amount: 1 }) });
                                if indexes_mut.is_empty() {
                                    indexes_mut = indexes.clone();
                                }
                            }
                            // console_dbg!(&state_vec);
                            status.set(state_vec);
                            app_status.set_fn(|st|st.next());
                        }
                    }){"Empezar"}
                }
            },
            PlayerRole::Player { .. } => view!{
                p(){"Esperando al master para empezar el juego..."}
            },
        })
    }
}

#[derive(Copy,Clone,PartialEq, Eq, Debug)]
enum Action {
    CreateRoom,
    JoinRoom,
}
