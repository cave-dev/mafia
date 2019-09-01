use crate::phases::{Evening, LastWords, Lobby, Morning, Night, Vote};
use crate::player::{Player, PlayerName};
use crate::player_connection::PlayerConnection;
use crate::ruleset::Ruleset;
use chrono::{DateTime, Utc};
use im::{vector, HashSet, Vector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct State<PC: PlayerConnection> {
    day: u32,
    players: Vector<Player<PC>>,
    rules: Ruleset,
    vote_skip: HashSet<PlayerName>,
    next_state_time: Option<DateTime<Utc>>,
    host: PlayerName,

    #[serde(flatten)]
    phase: Phase,
}

impl<PC: PlayerConnection> State<PC> {
    pub fn new(rules: Ruleset, host: Player<PC>) -> Self {
        let next_state_time = rules.day_end();
        let phase = rules.init_phase();
        let host_name = host.get_name().to_string();
        State {
            day: 1,
            players: vector![host],
            rules,
            vote_skip: HashSet::new(),
            next_state_time,
            host: host_name,
            phase,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "phase")]
pub enum Phase {
    Lobby(Lobby),
    Morning(Morning),
    Vote(Vote),
    LastWords(LastWords),
    Evening(Evening),
    Night(Night),
}

impl Phase {
    pub fn same_state(&self, other: &Phase) -> bool {
        use Phase::*;
        match (self, other) {
            (Lobby(_), Lobby(_))
            | (Morning(_), Morning(_))
            | (Vote(_), Vote(_))
            | (LastWords(_), LastWords(_))
            | (Evening(_), Evening(_))
            | (Night(_), Night(_)) => true,
            _ => false,
        }
    }

    pub fn next_state(&mut self) {}
}

impl From<Lobby> for Phase {
    fn from(s: Lobby) -> Self {
        Phase::Lobby(s)
    }
}

impl From<Morning> for Phase {
    fn from(s: Morning) -> Self {
        Phase::Morning(s)
    }
}

impl From<Vote> for Phase {
    fn from(s: Vote) -> Self {
        Phase::Vote(s)
    }
}

impl From<LastWords> for Phase {
    fn from(s: LastWords) -> Self {
        Phase::LastWords(s)
    }
}

impl From<Evening> for Phase {
    fn from(s: Evening) -> Self {
        Phase::Evening(s)
    }
}

impl From<Night> for Phase {
    fn from(s: Night) -> Self {
        Phase::Night(s)
    }
}
