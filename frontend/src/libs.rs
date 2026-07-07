use std::{collections::HashMap, sync::LazyLock};

use futures::{SinkExt, StreamExt, channel::mpsc::UnboundedSender};
use gloo_net::websocket::{Message, futures::WebSocket};
use macros::string;
use structs::{CName, MessageDTO, Player, Point, ResponseDTO, RoomMaster, Tokens};
use sycamore::{
    reactive::{ReadSignal, Signal},
    web::{console_dbg, console_error, console_log},
};
use uuid::Uuid;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Clipboard, js_sys::Date, window};

use crate::structs::{AppStatus, GamePhase, Movement, Notification};
pub static HOST: LazyLock<String> = LazyLock::new(|| std::env!("BACKEND").to_string());
pub async fn send_message(
    send: ReadSignal<Option<UnboundedSender<Message>>>,
    message: MessageDTO,
) -> Result<(), String> {
    match send.get_clone() {
        Some(mut sender) => match sender
            .send(Message::Text(
                serde_json::to_string(&message).unwrap_or_default(),
            ))
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        },
        None => Err(string!("Error: No Socket available")),
    }
}

pub async fn connect(
    ConnectParams {
        users,
        status,
        ws_sender,
        notification,
        app_status,
        this_player,
        room_master,
        missions,
        game_phase,
    }: ConnectParams,
) {
    console_log!("{}", HOST.as_str());

    let ws = match WebSocket::open(&HOST) {
        Ok(ws) => ws,
        Err(e) => {
            console_error!("Failed to connect to WebSocket: {}", e);
            return;
        }
    };

    console_log!("Connected to WebSocket, state: {:#?}", ws.state());
    let (mut write, mut read) = ws.split();
    let (tx, mut rx) = futures::channel::mpsc::unbounded();
    ws_sender.set(Some(tx));
    spawn_local(async move {
        while let Some(Ok(Message::Text(msg))) = read.next().await {
            match serde_json::from_str::<ResponseDTO>(&msg) {
                Ok(msg_dto) => match msg_dto {
                    ResponseDTO::UpdateState { statuses } => {
                        for (id,st) in statuses {
                            let country_st = match status.get_clone().get(&id) {
                                Some(s) => s,
                                None => {
                                    console_error!(
                                        "Received status for unknown country: {}",
                                        id
                                    );
                                    continue;
                                }
                            };
                        }
                    }
                    ResponseDTO::UpdateRoom {
                        room_id,
                        players,
                        statuses,
                    } => {
                        users.set(players);
                        status.set(statuses);
                    }
                    ResponseDTO::GameStarted {
                        status: status_,
                        players,
                        starter,
                        missions: _missions,
                    } => {
                        console_dbg!(&starter);
                        game_phase.set(GamePhase {
                            player: starter,
                            movement: Movement::AssignTroops,
                        });
                        users.set(players);
                        status.set(status_);
                        missions.set(_missions);
                        app_status.set(AppStatus::InGame);
                    }
                    ResponseDTO::MissionCompleted { player } => (),
                    ResponseDTO::Error { message } => {
                        notification.set(Notification::Error(message))
                    }
                    ResponseDTO::LoggedIn {
                        users: users_,
                        this_player: this_player_,
                        room,
                    } => {
                        console_log!(
                            "Logged in: users: {:#?}, this_player: {:#?}, room: {:#?}",
                            users_,
                            this_player_,
                            room
                        );
                        // console_dbg!(&msg);
                        users.set(users_);
                        // status.set(status_.iter().map(|(id, st)|{
                        //     let mut status = st.clone();
                        //     status.location = get_point(map.0.get(id).unwrap().name());
                        //     status.tokens = None;
                        //     (*id, status)
                        // }).collect::<HashMap<_,_>>());
                        this_player.set(Some(this_player_));
                        room_master.set(Some(room));
                        // app_status.set_fn(|st|st.next());
                    }
                    ResponseDTO::CompleteUpdate {
                        room,
                        players,
                        status: status_,
                        this_player: this_player_,
                    } => {
                        console_log!(
                            "Complete Update received: master: {:#?}, players: {:#?}, status: {:#?}",
                            room,
                            players,
                            status_
                        );
                        // console_dbg!(&msg);
                        this_player.set_silent(players.get(&this_player_).cloned());
                        room_master.set(Some(room));
                        users.set(players);

                        // status.set(status_);
                        app_status.set_fn(|st| st.next());
                    }
                },
                Err(e) => console_error!("Failed to parse message: {}", e),
            }
        }
    });
    spawn_local(async move {
        while let Some(msg) = rx.next().await {
            write.send(msg).await.unwrap();
        }
    });
}

pub fn get_token() -> Result<Option<Player>, JsValue> {
    if let Some(window) = window()
        && let Some(storage) = window.session_storage()?
        && let Some(data) = storage.get_item("teg_token")?
    {
        if let Some(value) = data.split("|").nth(0) {
            console_dbg!(value);
            Ok(Some(
                serde_json::from_str(value)
                    .map_err(|e| JsValue::from_str(e.to_string().as_str()))?,
            ))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub fn delete_token() -> Result<(), JsValue> {
    if let Some(window) = window()
        && let Some(storage) = window.session_storage()?
    {
        storage.remove_item("teg_token")?;
    }
    Ok(())
}

pub fn save_token(token: String) -> Result<(), JsValue> {
    if let Some(window) = window()
        && let Some(storage) = window.session_storage()?
    {
        let expiry = Date::now() + 10_200_000.0; // 3 hours in ms
        let data = format!("{}|{}", token, expiry);
        storage.set_item("teg_token", &data)?
    }
    Ok(())
}

pub async fn copy_to_clipboard(text: &str) -> Result<(), JsValue> {
    let nav = window().ok_or("no window")?.navigator();
    // La API Clipboard puede no estar disponible en contextos inseguros
    let clipboard: Clipboard = nav.clipboard();
    JsFuture::from(clipboard.write_text(text)).await?;
    Ok(())
}

pub struct ConnectParams {
    pub users: Signal<HashMap<Uuid, Player>>,
    pub status: Signal<HashMap<Uuid, Tokens>>,
    pub ws_sender: Signal<Option<UnboundedSender<Message>>>,
    pub notification: Signal<Notification>,
    pub app_status: Signal<AppStatus>,
    pub this_player: Signal<Option<Player>>,
    pub room_master: Signal<Option<RoomMaster>>,
    pub missions: Signal<HashMap<Uuid, String>>,
    pub game_phase: Signal<GamePhase>,
}

pub fn get_point(name: CName, width: f32) -> Point {
    match name {
        CName::Canadá => Point::new(width * 0.165, width * 0.12),
        CName::Yukón => Point::new(width * 0.09, width * 0.18),
        CName::Alaska => Point::new(width * 0.025, width * 0.22),
        CName::Groenlandia => Point::new(width * 0.35, width * 0.115),
        CName::Oregón => Point::new(width * 0.085, width * 0.255),
        CName::California => Point::new(width * 0.21, width * 0.28),
        CName::México => Point::new(width * 0.25, width * 0.335),
        CName::NuevaYork => Point::new(width * 0.17, width * 0.205),
        CName::Terranova => Point::new(width * 0.20, width * 0.195),
        CName::Labrador => Point::new(width * 0.255, width * 0.17),
        CName::Argentina => Point::new(width * 0.33, width * 0.45),
        CName::Brasil => Point::new(width * 0.36, width * 0.38),
        CName::Perú => Point::new(width * 0.295, width * 0.4),
        CName::Colombia => Point::new(width * 0.33, width * 0.345),
        CName::Chile => Point::new(width * 0.3, width * 0.495),
        CName::Uruguay => Point::new(width * 0.37, width * 0.445),
        CName::GranBretaña => Point::new(width * 0.545, width * 0.235),
        CName::Islandia => Point::new(width * 0.43, width * 0.225),
        CName::España => Point::new(width * 0.505, width * 0.345),
        CName::Francia => Point::new(width * 0.58, width * 0.295),
        CName::Alemania => Point::new(width * 0.63, width * 0.275),
        CName::Italia => Point::new(width * 0.63, width * 0.34),
        CName::Polonia => Point::new(width * 0.68, width * 0.265),
        CName::Rusia => Point::new(width * 0.68, width * 0.18),
        CName::Suecia => Point::new(width * 0.595, width * 0.15),
        CName::Sahara => Point::new(width * 0.59, width * 0.42),
        CName::Etiopía => Point::new(width * 0.665, width * 0.445),
        CName::Egipto => Point::new(width * 0.685, width * 0.42),
        CName::Madagascar => Point::new(width * 0.77, width * 0.48),
        CName::Zaire => Point::new(width * 0.62, width * 0.48),
        CName::Sudáfrica => Point::new(width * 0.71, width * 0.52),
        CName::Arabia => Point::new(width * 0.795, width * 0.38),
        CName::Aral => Point::new(width * 0.72, width * 0.13),
        CName::China => Point::new(width * 0.87, width * 0.2),
        CName::India => Point::new(width * 0.865, width * 0.315),
        CName::Irán => Point::new(width * 0.76, width * 0.225),
        CName::Tartaria => Point::new(width * 0.745, width * 0.11),
        CName::Taymyr => Point::new(width * 0.79, width * 0.11),
        CName::Japón => Point::new(width * 0.915, width * 0.14),
        CName::Kamchatka => Point::new(width * 0.86, width * 0.1),
        CName::Siberia => Point::new(width * 0.815, width * 0.14),
        CName::Mongolia => Point::new(width * 0.8, width * 0.19),
        CName::Gobi => Point::new(width * 0.835, width * 0.22),
        CName::Malasia => Point::new(width * 0.935, width * 0.32),
        CName::Turquía => Point::new(width * 0.72, width * 0.32),
        CName::Israel => Point::new(width * 0.717, width * 0.355),
        CName::Sumatra => Point::new(width * 0.86, width * 0.42),
        CName::Borneo => Point::new(width * 0.89, width * 0.38),
        CName::Java => Point::new(width * 0.932, width * 0.38),
        CName::Australia => Point::new(width * 0.932, width * 0.445),
    }
}
