use crate::player_connection::PlayerConnection;
use serde::{Deserialize, Serialize};

pub type PlayerName = String;
pub type PlayerNameRef<'a> = &'a str;

#[derive(Clone, Serialize, Deserialize)]
pub struct Player<PC: PlayerConnection> {
    name: PlayerName,
    pub connection: Option<PC>,
    pub state: PlayerState,
    pub role: Role,
    pub secret_key: String,
}

impl<PC: PlayerConnection> Player<PC> {
    pub fn get_name(&self) -> PlayerNameRef {
        self.name.as_ref()
    }
}

#[derive(Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum PlayerState {
    Alive,
    Dead,
}

impl PlayerState {
    pub fn is_alive(&self) -> bool {
        *self == PlayerState::Alive
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Role {
    Townie,
    Mafioso,
    Doctor,
    Bartender,
    Detective,
}
