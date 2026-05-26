use std::{collections::HashMap, sync::{Arc, LazyLock}};

use futures::{SinkExt, StreamExt, channel::mpsc::UnboundedSender};
use gloo_net::websocket::{Message, futures::WebSocket};
use macros::string;
use structs::{CStatus, Map, MessageDTO, Player, ResponseDTO};
use sycamore::{reactive::{ReadSignal, Signal}, web::{console_dbg, console_error, console_log}};
use uuid::Uuid;
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{js_sys::Date, window};

use crate::app::AppStatus;
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

pub async fn connect(ConnectParams { map, users, status, ws_sender, error, app_status } : ConnectParams) {
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
                Ok(msg) => match msg {
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
                    ResponseDTO::GameStarted => (),
                    ResponseDTO::MissionCompleted { player } => (),
                    ResponseDTO::Error { message } => error.set(Some(message)),
                    ResponseDTO::LoggedIn { users: users_, status: status_ } => {
                        users.set(users_);
                        status.set(status_);
                        app_status.set(AppStatus::Lobby);
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

pub struct ConnectParams {
    pub map: Arc<Map>,
    pub users: Signal<HashMap<Uuid, Player>>,
    pub status: Signal<HashMap<Uuid, CStatus>>,
    pub ws_sender: Signal<Option<UnboundedSender<Message>>>,
    pub error: Signal<Option<String>>,
    pub app_status: Signal<AppStatus>,
}