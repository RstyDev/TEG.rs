use std::{collections::HashMap, sync::Arc};
use rand::prelude::*;
use macros::string;
use structs::{CStatus, Map, Player, PlayerRole, Tokens};
use sycamore::prelude::*;
use uuid::Uuid;
use web_sys::{MouseEvent, SubmitEvent};

use crate::app::get_point;

#[component(inline_props)]
pub fn AddUsers(map: Arc<Map>, users: Signal<HashMap<Uuid, Player>>,status: Signal<HashMap<Uuid,CStatus>>) -> View {
    let users_sel = create_selector(move||{
        users.get_clone().values().cloned().collect::<Vec<_>>()
    });
    let user_name = create_signal(string!());
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
        aside(id="side_forms"){
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
                let new_user = Player::new(user_name.get_clone(), PlayerRole::Player { room: Uuid::new_v4() });
                users.update(|users| {
                    users.insert(new_user.id(), new_user);
                });
                user_name.set(string!());
            }){
                input(placeholder="Ingrese el nombre del jugador", bind:value=user_name){}
                input(r#type="submit", value="Agregar jugador"){}
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
}