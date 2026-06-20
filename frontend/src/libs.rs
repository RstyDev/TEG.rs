use std::{collections::HashMap, sync::{Arc, LazyLock}};

use futures::{SinkExt, StreamExt, channel::mpsc::UnboundedSender};
use gloo_net::websocket::{Message, futures::WebSocket};
use macros::string;
use structs::{CStatus, Map, MessageDTO, Player, ResponseDTO, RoomMaster};
use sycamore::{reactive::{ReadSignal, Signal}, web::{console_dbg, console_error, console_log}};
use uuid::Uuid;
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Clipboard, js_sys::Date, window};

use crate::{app::get_point, structs::{AppStatus, Notification}};
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

pub async fn connect(ConnectParams { map, users, status, ws_sender, notification, app_status, this_player, room_master } : ConnectParams) {
    console_log!("{}",HOST.as_str());
    
    let ws = match WebSocket::open(&HOST) {
        Ok(ws) => ws,
        Err(e) => {
            console_error!("Failed to connect to WebSocket: {}", e);
            return;
        },
    };
    
    console_log!("Connected to WebSocket, state: {:#?}", ws.state());
    let (mut write, mut read) = ws.split();
    let (tx,mut rx) = futures::channel::mpsc::unbounded();
    ws_sender.set(Some(tx));
    spawn_local(async move {
        while let Some(Ok(Message::Text(msg))) = read.next().await {
            match serde_json::from_str::<ResponseDTO>(&msg) {
                Ok(msg_dto) => match msg_dto {
                    ResponseDTO::UpdateState { statuses } => for st in statuses {
                        let country_st = match status.get_clone().get(&st.country_id) {
                            Some(s) => s,
                            None => {
                                console_error!("Received status for unknown country: {}", st.country_id);
                                continue},
                        };
                        

                    },
                    ResponseDTO::UpdateRoom { room_id, players, statuses } => {
                        users.set(players);
                        status.set(statuses);
                    }
                    ResponseDTO::GameStarted { room, players, status: status_ } => {
                        users.set(players);
                        status.set(status_);
                        app_status.set(AppStatus::InGame);
                    },
                    ResponseDTO::MissionCompleted { player } => (),
                    ResponseDTO::Error { message } => notification.set(Notification::Error(message)),
                    ResponseDTO::LoggedIn { users: users_, this_player: this_player_, room } => {
                        console_log!("Logged in: users: {:#?}, this_player: {:#?}, room: {:#?}", users_, this_player_, room);
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
                    },
                    ResponseDTO::CompleteUpdate { room, players, status: status_, this_player: this_player_ } => {
                        console_log!("Complete Update received: master: {:#?}, players: {:#?}, status: {:#?}", room, players, status_);
                        // console_dbg!(&msg);
                        this_player.set_silent(players.get(&this_player_).cloned());
                        room_master.set(Some(room));
                        users.set(players);

                        // status.set(status_);
                        app_status.set_fn(|st|st.next());
                    },
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
    pub map: Arc<Map>,
    pub users: Signal<HashMap<Uuid, Player>>,
    pub status: Signal<HashMap<Uuid, CStatus>>,
    pub ws_sender: Signal<Option<UnboundedSender<Message>>>,
    pub notification: Signal<Notification>,
    pub app_status: Signal<AppStatus>,
    pub this_player: Signal<Option<Player>>,
    pub room_master: Signal<Option<RoomMaster>>,
}