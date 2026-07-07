use std::collections::HashMap;

use futures::channel::mpsc::UnboundedSender;
use gloo_net::websocket::Message;
use structs::{MAP, Player, RoomMaster, Tokens};
use sycamore::prelude::*;
use uuid::Uuid;

use crate::structs::{GamePhase, Movement, Notification};

#[component(inline_props)]
pub fn InGame(
    missions: Signal<HashMap<Uuid, String>>,
    users: Signal<HashMap<Uuid, Player>>,
    status: Signal<HashMap<Uuid, Tokens>>,
    send: Signal<Option<UnboundedSender<Message>>>,
    notification: Signal<Notification>,
    room: Signal<Option<RoomMaster>>,
    this_player: Signal<Option<Player>>,
    game_phase: Signal<GamePhase>,
) -> View {
    console_dbg!(&missions);
    let my_mission = create_selector(move || {
        missions.get_clone().into_iter().find_map(|m| {
            this_player
                .get_clone()
                .into_iter()
                .find_map(|p| (p.id() == m.0).then_some(m.1.to_owned()))
        })
    });
    let show_mission = create_signal(false);
    let my_turn = this_player
        .get_clone()
        .as_ref()
        .map_or(false, |p| p.id() == game_phase.get_clone().player);
    // console_dbg!(&MAP);
    let your_countries = create_selector(move || {
        status
            .get_clone()
            .into_iter()
            .filter_map(|(id, st)| {
                this_player
                    .get_clone()
                    .map_or(false, |us| us.id() == id)
                    .then_some((
                        id,
                        MAP.get_or_init(Default::default).get(&id)
                            .map(|m| m.name().to_string())
                            .unwrap_or_default(),
                    ))
            })
            .collect::<Vec<_>>()
    });
    view! {
        p(){"In Game"}
        button(on:click=move |_| show_mission.set(!show_mission.get())){(format!("{} Mission",match show_mission.get(){
            true => "Hide",
            false => "Show",
        }))}
        (match show_mission.get(){
            true => view!{
                p(){(match my_mission.get_clone(){
                    Some(m) => m,
                    None => String::new(),
                })}
            },
            false => view!{},
        })
        p(){"Fase: " (match game_phase.get_clone().movement {
            Movement::AssignTroops => "Ordenamiento de tropas",
            Movement::Advance => "Decisión de ataque",
            Movement::Attack => "Ataque",
            Movement::Defend => "Defensa",
        })}
        (match my_turn {
            true => view!{
                p(){"Tu turno"}
                form(){
                    label(r#for="your_countries"){"Tus países"}
                    select(id="your_countries"){
                        (your_countries.get_clone().into_iter().map(|(id,c)|{view!{
                            option(value=id.to_string()){(c)}
                        }}).collect::<Vec<_>>())
                    }
                }

            },
            false => view!{
                p(){"Turno de "(users.get_clone()[&game_phase.with(|g|g.player)].name().to_owned())}
            },
        })
    }
}
