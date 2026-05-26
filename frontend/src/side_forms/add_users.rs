use std::{collections::HashMap, str::FromStr, sync::Arc};
use futures::channel::mpsc::UnboundedSender;
use gloo_net::websocket::Message;
use rand::prelude::*;
use macros::string;
use structs::{CStatus, Map, MessageDTO, Player, PlayerRole, Tokens};
use sycamore::prelude::*;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_sys::{MouseEvent, SubmitEvent};

use crate::{app::get_point, libs::send_message};

#[component(inline_props)]
pub fn AddUsers(map: Arc<Map>, users: Signal<HashMap<Uuid, Player>>,status: Signal<HashMap<Uuid,CStatus>>, send: Signal<Option<UnboundedSender<Message>>>, error: Signal<Option<String>>) -> View {
    let action = create_signal(Action::CreateRoom);
    let users_sel = create_selector(move||{
        users.get_clone().values().cloned().collect::<Vec<_>>()
    });
    let user_name = create_signal(string!());
    let room_code = create_signal(string!());
    // create_effect(move || {
    //     console_dbg!(users_sel.get_clone());
    // });
    users.set_fn(|u|{
        let mut u = u.clone();
        for i in 0..5 {
            u.insert(Uuid::new_v4(), Player::new(format!("Jugador {}", u.len() + i), PlayerRole::Player { room: Uuid::new_v4() }));
        }
        u
    });
    // create_memo(move || {
    //     console_log!("=-=-= Status actual: =-=-= {:?}", status.get_clone());
    // });
    
    view!{
        
            button(class=match action.get(){
                Action::CreateRoom => "selected",
                _ => "",
            },on:click=move|ev:MouseEvent|{
                ev.prevent_default();
                if action.get() == Action::JoinRoom {
                    action.set(Action::CreateRoom);
                }
            }){"Create Room"}
            button(class=match action.get(){
                Action::JoinRoom => "selected",
                _ => "",
            },on:click=move|ev:MouseEvent|{
                ev.prevent_default();
                if action.get() == Action::CreateRoom {
                    action.set(Action::JoinRoom);
                }
            }){"Join Room"}
            p(){"Jugadores actuales:"}
            Keyed(
                list=users_sel,
                view=|player| view!{
                    p() { (format!("{}: {}", player.id(), player.name())) }
                },
                key = |player|player.id()
            )
            form(on:submit=move|ev: SubmitEvent|{
                ev.prevent_default();
                spawn_local(async move {
                    if let Err(e) = match action.get(){
                        Action::CreateRoom => {
                            let size = user_name.with(|n| n.len());
                            if size == 0 {
                                console_log!("User name is empty - Add Users");
                                Err(string!("Debe ingresar el nombre"))
                            } else {
                                let player = Player::new(user_name.get_clone(), PlayerRole::Master );
                                send_message(*send, MessageDTO::AddPlayer { player }).await
                            }
                        },
                        Action::JoinRoom => {
                            match Uuid::from_str(room_code.get_clone().as_str()) {
                                Ok(room_id) => {
                                    let size = user_name.with(|n| n.len());
                                    if size == 0 {
                                        Err(string!("Debe ingresar el nombre"))
                                    } else {
                                        let player = Player::new(user_name.get_clone(), PlayerRole::Player { room: room_id });
                                        send_message(*send, MessageDTO::AddPlayer { player }).await
                                    }
                                },
                                Err(e) => Err(format!("Codigo de sala inválido: {e}")),
                            }
                            
                        },
                    } {
                        error.set(Some(format!("Error: {e}")));
                    }
                });
                // users.update(|users| {
                //     users.insert(new_user.id(), new_user);
                // });
                user_name.set(string!());
            }){
                (match action.get(){
                    Action::CreateRoom => view!{
                        input(placeholder="Nombre", bind:value=user_name){}
                        input(r#type="submit", value="Crear sala"){}
                    },
                    Action::JoinRoom => view!{
                        input(placeholder="Nombre", bind:value=user_name){}
                        input(placeholder="Código de sala", bind:value=room_code){}
                        input(r#type="submit", value="Unirse"){}
                    },
                })
            }
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
                        state_vec.insert(country_id,CStatus{ country_id, location: get_point(map.0.get(&country_id).unwrap().name()), tokens: Some(Tokens { owner: users_copy[i].id(), amount: 2 }) });
                        if indexes_mut.is_empty() {
                            indexes_mut = indexes.clone();
                        }
                    }
                    // console_dbg!(&state_vec);
                    status.set(state_vec);
                }
            }){"Empezar"}
    }
}

#[derive(Copy,Clone,PartialEq, Eq, Debug)]
enum Action {
    CreateRoom,
    JoinRoom,
}
