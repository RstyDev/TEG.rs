use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Notification {
    Error(String),
    Warning(String),
    Info(String),
    None,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AppStatus {
    Login,
    Lobby,
    InGame,
}

impl AppStatus {
    pub fn next(&self) -> Self {
        match self {
            AppStatus::Login => AppStatus::Lobby,
            AppStatus::Lobby => AppStatus::InGame,
            AppStatus::InGame => AppStatus::InGame,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GameStage {
    AddingTroops { player_id: Uuid },
    Moving { player_id: Uuid },
    Won { player_id: Uuid },
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Mission {
    pub name: String,
    pub objective: Option<Uuid>,
}
