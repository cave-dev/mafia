use crate::player_connection::PlayerConnection;
use serde::{Deserialize, Serialize};

pub type PlayerName = String;
pub type PlayerNameRef<'a> = &'a str;

#[derive(Clone, Serialize, Deserialize)]
pub struct Player<PC: PlayerConnection> {
    name: PlayerName,
    connection: Option<PC>,
    state: PlayerState,
    role: Role,
    secret_key: String,
}

impl<PC: PlayerConnection> Player<PC> {
    pub fn get_name(&self) -> PlayerNameRef {
        self.name.as_ref()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PlayerState {
    Alive,
    Dead,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Role {
    Townie,
    Mafioso,
    Doctor,
    Bartender,
    Detective,
}
