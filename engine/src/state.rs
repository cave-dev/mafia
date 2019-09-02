use crate::phases::{Evening, LastWords, Lobby, Morning, Night, Vote};
use crate::player::{Player, PlayerName};
use crate::player_connection::PlayerConnection;
use crate::ruleset::Ruleset;
use chrono::{DateTime, Utc};
use im::{vector, HashSet, Vector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RootState<PC: PlayerConnection> {
    pub day: u32,
    pub players: Vector<Player<PC>>,
    pub rules: Ruleset,
    pub vote_skip: HashSet<PlayerName>,
    pub next_state_time: Option<DateTime<Utc>>,
    pub host: PlayerName,
}

#[derive(Serialize, Deserialize)]
pub struct State<PC: PlayerConnection> {
    #[serde(flatten)]
    root: RootState<PC>,

    #[serde(flatten)]
    phase: Phase,
}

impl<PC: PlayerConnection> State<PC> {
    pub fn new(rules: Ruleset, host: Player<PC>) -> Self {
        let next_state_time = rules.morning_end();
        let phase = rules.init_phase();
        let host_name = host.get_name().to_string();
        State {
            root: RootState {
                day: 1,
                players: vector![host],
                rules,
                vote_skip: HashSet::new(),
                next_state_time,
                host: host_name,
            },
            phase,
        }
    }

    pub fn get_playername(&self, secret: &str) -> Option<PlayerName> {
        self.root
            .players
            .iter()
            .find(|p| &p.secret == secret)
            .map(|p| p.get_name().to_string())
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
    pub fn same_phase(&self, other: &Phase) -> bool {
        use Phase::*;
        match (self, other) {
            (Lobby(_), Lobby(_))
            | (Morning(_), Morning(_))
            | (Vote(_), Vote(_))
            | (LastWords(_), LastWords(_))
            | (Evening(_), Evening(_))
            | (Night(_), Night(_)) => true,
            (Lobby(_), _)
            | (Morning(_), _)
            | (Vote(_), _)
            | (LastWords(_), _)
            | (Evening(_), _)
            | (Night(_), _) => false,
        }
    }
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
