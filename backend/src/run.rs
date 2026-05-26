use std::{collections::HashMap, env, net::SocketAddr, sync::Arc, io::{ErrorKind, Error}};
use axum::{Router, extract::{State, WebSocketUpgrade, ws::WebSocket}, response::IntoResponse, routing::get};
use dotenv::dotenv;
use futures_util::StreamExt;
use macros::string;

use tokio::{
    net::TcpListener,
    sync::{Mutex, broadcast::Sender},
    task,
};
use structs::{CStatus, Map, Player};
use uuid::Uuid;

use crate::tasks::{receive_task::{ReceiveParams, receive_task}, send_task::{SendParams, send_task}};

#[derive(Debug,Clone)]
pub struct Room {
    pub id: Uuid,
    pub master: Option<Player>,
    pub players: Arc<Mutex<HashMap<Uuid,Player>>>,
    pub countries: Map,
    pub status: Arc<Mutex<HashMap<Uuid,CStatus>>>,
    pub tx: Sender<SenderMessage>,
}
#[derive(Clone,Debug, Copy)]
pub enum SenderMessage {
    Move { room_id: Uuid, player_id: Uuid, from: Uuid, to: Uuid, troops: u32 },
    UpdateState { room_id: Uuid },
    StartGame { room_id: Uuid },
    LoggedIn,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub rooms: Arc<Mutex<HashMap<Uuid, Room>>>,
}

pub async fn run() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv().unwrap();
    let state = AppState {
        rooms: Arc::new(Mutex::new(HashMap::new())),
    };
    let app = Router::new().route("/ws", get(ws_handler)).with_state(state);
    let addr: SocketAddr = match env::var(string!("HOST")) {
        Ok(e) => {
            e
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?
        },
        Err(e) => panic!("{e}"),
    };
    let listener = TcpListener::bind(addr).await?;
    println!("WebSocket server in {}", addr);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())

}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move { handle_socket(socket, state).await })
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    println!("Client connected");
    let this_user: Arc<Mutex<Option<RoomPlayer>>> = Arc::new(Mutex::new(None));
    let this_other = this_user.clone();
    let (send, recv) = socket.split();
    // let arc_users = state.users.clone();
    let arc_rooms = state.rooms.clone();

    let mut send_task = task::spawn(send_task(SendParams {
        this_player: this_other,
        arc_rooms,
        send,
    }));

    let mut recv_task = task::spawn(receive_task(ReceiveParams {
        recv,
        state,
        this_user,
    }));

    tokio::select! {
        _a = (&mut send_task) => recv_task.abort(),
        _b = (&mut recv_task) => send_task.abort(),
    }
}
#[derive(Debug, Clone)]
pub struct RoomPlayer {
    pub room_id: Uuid,
    pub player: Player,
}