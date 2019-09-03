use crate::player::{PlayerName, PlayerNameRef};
use crate::player_connection::PlayerConnection;
use crate::ruleset::Ruleset;
use crate::state::State;
use crate::Result;
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session<PC: PlayerConnection> {
    id: String,
    state: State<PC>,
}

impl<PC: PlayerConnection> Session<PC> {
    pub fn new(id: String, rules: Ruleset, host: PlayerName, host_secret: String) -> Self {
        debug!(
            "creating session: id={} host={} secret={}",
            id, host, host_secret
        );
        Session {
            id,
            state: State::new(rules, host, host_secret),
        }
    }

    pub fn get_playername(&self, secret: &str) -> Option<PlayerName> {
        self.state.get_playername(secret)
    }

    pub fn get_connection(&self, player: PlayerNameRef) -> Option<PC> {
        debug!("getting player connection {}", player);
        self.state.get_connection(player)
    }

    pub fn register_connection(&mut self, player: PlayerName, conn: Option<PC>) {
        debug!("registering player connection {}", player);
        self.state.register_connection(player, conn);
    }

    pub fn create_user(&mut self, player: PlayerName, secret: String) -> Result<()> {
        self.state.create_user(player, secret)
    }
}
