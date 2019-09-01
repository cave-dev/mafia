use crate::lobby::LobbyState;
use crate::morning::MorningState;
use crate::player::{Player, PlayerName};
use crate::player_connection::PlayerConnection;
use crate::ruleset::Ruleset;
use crate::vote::VoteState;
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
    ext: StateExt,
}

impl<PC: PlayerConnection> State<PC> {
    pub fn new(rules: Ruleset, host: Player<PC>) -> Self {
        let next_state_time = rules.day_end();
        let ext = rules.init_morning();
        let host_name = host.get_name().to_string();
        State {
            day: 1,
            players: vector![host],
            rules,
            vote_skip: HashSet::new(),
            next_state_time,
            host: host_name,
            ext,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "stage")]
pub enum StateExt {
    Lobby(LobbyState),
    Morning(MorningState),
    Vote(VoteState),
}

impl StateExt {
    pub fn same_state(&self, other: &StateExt) -> bool {
        use StateExt::*;
        match (self, other) {
            (Lobby(_), Lobby(_)) | (Morning(_), Morning(_)) | (Vote(_), Vote(_)) => true,
            _ => false,
        }
    }
}

impl From<LobbyState> for StateExt {
    fn from(s: LobbyState) -> Self {
        StateExt::Lobby(s)
    }
}

impl From<MorningState> for StateExt {
    fn from(s: MorningState) -> Self {
        StateExt::Morning(s)
    }
}

impl From<VoteState> for StateExt {
    fn from(s: VoteState) -> Self {
        StateExt::Vote(s)
    }
}
