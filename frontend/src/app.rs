use futures::channel::mpsc::UnboundedSender;
use gloo_net::websocket::Message;
use gloo_timers::future::sleep;
use macros::string;
use wasm_bindgen::{prelude::Closure, JsCast};
use std::{collections::HashMap, sync::OnceLock, time::Duration};
use structs::{MAP, Player, Point, RoomMaster, Tokens, initialize_map};
use sycamore::{futures::spawn_local, prelude::*};
use uuid::Uuid;
use web_sys::window;
use crate::{
    libs::{ConnectParams, connect, get_point},
    side_forms::{InGame, Lobby, SelectRoom},
    structs::{AppStatus, GamePhase, Notification},
};
const CSS: &str = "border-radius: 10px; border: 2px solid black; background-color: white; padding: 5px; font-size: 14px; font-weight: bold;";
static LOCATIONS: OnceLock<HashMap<Uuid, Point>> = OnceLock::new();
#[component]
pub fn App() -> View {
    initialize_map();
    let width = create_signal(get_width());
    let closure = Closure::<dyn Fn()>::new(move || {
        width.set(get_width());
    });
    window()
        .unwrap()
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .unwrap();

    closure.forget();
    let users = create_signal(HashMap::new());
    let game_phase = create_signal(GamePhase {
        player: Uuid::nil(),
        movement: crate::structs::Movement::AssignTroops,
    });
    let missions = create_signal(HashMap::new());

    LOCATIONS.set({
        MAP.get_or_init(Default::default)
            .iter()
            .map(|(id, c)| (*id, get_point(c.name(), width.get()*0.75)))
            .collect::<HashMap<_, _>>()
    }).unwrap();
    console_dbg!(&LOCATIONS);
    let status = create_signal(
        MAP.get_or_init(Default::default).iter()
            .map(|(&country_id, _)| {
                (
                    country_id,
                    Tokens {
                        owner: Uuid::nil(),
                        amount: 0,
                    },
                )
            })
            .collect::<HashMap<_, _>>(),
    );

    let notification = create_signal(Notification::None);
    let ws_sender: Signal<Option<UnboundedSender<Message>>> =
        create_signal(None::<UnboundedSender<Message>>);
    let app_status = create_signal(AppStatus::Login);
    let this_player = create_signal(None::<Player>);
    let room_master = create_signal(None::<RoomMaster>);
    // create_memo(move || {
    //     // console_dbg!(&status);
    //     // console_dbg!(&users);
    // });
    create_memo(move || {
        let err = notification.with(|e| e != &Notification::None);
        spawn_local(async move {
            if err {
                sleep(Duration::from_millis(2000)).await;
                notification.set(Notification::None);
                // console_dbg!("Copied to false now");
            }
        });
    });
    create_memo(move || {
        console_log!("{:#?}", this_player.get_clone());
        console_log!("{:#?}", room_master.get_clone());
    });
    let connect_params = ConnectParams {
        users,
        status,
        ws_sender: ws_sender.clone(),
        notification,
        app_status,
        this_player: this_player.clone(),
        room_master: room_master.clone(),
        missions,
        game_phase,
    };
    spawn_local(connect(connect_params));
    // console_dbg!(&map);
    view! {
        article(){
            img(src="./public/map.webp", alt="Mapa del juego",width=format!("{}px",width.get()*0.75), height=format!("{}px",width.get()*0.5)){}
        }
        aside(id="side_forms"){
            (match app_status.get() {
                AppStatus::Login => view!{SelectRoom(send = ws_sender, notification = notification)},
                AppStatus::Lobby => view!{Lobby(users=users, status=status, send = ws_sender, notification = notification, room=room_master, app_status=app_status, this_player = this_player)},
                AppStatus::InGame => view!{InGame(missions = missions, users=users, status=status, send = ws_sender, notification = notification, room=room_master, this_player = this_player, game_phase = game_phase)},
            })
            (LOCATIONS.get_or_init(Default::default).iter().map(|(id,c)|{
                let status = status.with(|st| st.get(&id).cloned());
                view!{
                    article(class="tokens"){
                        p(style=format!("{}position:absolute; left:{}px; top:{}px;", CSS, c.x, c.y)){(match &status{
                            None => string!("0"),
                            Some(tokens) => format!("{}",tokens.amount),
                        })}
                    }
                }
            }).collect::<Vec<_>>())
            // (status.get_clone().into_iter().map(|(id,c_status)|view!{
            //     article(class="tokens"){
            //             p(style=format!("{}position:absolute; left:{}px; top:{}px;", CSS, c_status.location.x, c_status.location.y)){(match &c_status.tokens{
            //                 None => string!("0"),
            //                 Some(tokens) => format!("{}",tokens.amount),
            //             })}
            //         }
            // }).collect::<Vec<View>>())
            // Keyed(
            //     list=status,
            //     view=|c_status| view!{
            //         article(class="tokens"){
            //             p(style=format!("{}position:absolute; left:{}px; top:{}px;", CSS, c_status.location.x, c_status.location.y)){(match &c_status.tokens{
            //                 None => string!("0"),
            //                 Some(tokens) => format!("{}",tokens.amount),
            //             })}
            //         }
            //     },
            //     key=|c_status| c_status.id
            // )
            (match notification.get_clone(){
                Notification::Error(er) => view!{p(class="notification error"){(er)}},
                Notification::Warning(warn) => view!{p(class="notification warning"){(warn)}},
                Notification::Info(info) => view!{p(class="notification info"){(info)}},
                Notification::None => view!{},
            })
        }
    }
}


fn get_width() -> f32 {
    window().unwrap().inner_width().unwrap().as_f64().unwrap() as f32
}
/*
OVERFLOW
Over 1: 15 34 56
Over 2: 100 176
Prom: 78
New: 15

UNDERFLOW
Root:
Leaf:
Fusionar:

NodosLibres: []

                                            2: [0](315)[1](485)[4](547)[5](639)[3]
                        0: (148)(223) 1: (333)(390)(442)(454) 4: (508)(511) 5: (614)(633) 3: (789)(915)

LE 0: Overflow: divido en 15 34 56 | 78 100 176, promuevo 78 al nuevo nodo padre
E 1: 100 176
E 2: 78


*/
