use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use structs::Player;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Notification {
    Error(String),
    Warning(String),
    Info(String),
    None
}

#[derive(Copy,Clone,PartialEq, Eq, Debug)]
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