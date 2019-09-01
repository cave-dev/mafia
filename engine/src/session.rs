use crate::player::Player;
use crate::player_connection::PlayerConnection;
use crate::ruleset::Ruleset;
use crate::state::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session<PC: PlayerConnection> {
    id: String,
    state: State<PC>,
}

impl<PC: PlayerConnection> Session<PC> {
    pub fn new(id: String, rules: Ruleset, host: Player<PC>) -> Self {
        Session {
            id,
            state: State::new(rules, host),
        }
    }
}
