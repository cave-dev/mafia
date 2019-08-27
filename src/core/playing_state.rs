use super::PlayerName;
use crate::core::night_action::NightAction;
use chrono::{DateTime, Utc};
use im::HashMap;

pub type SDateTime = DateTime<Utc>;

#[derive(Debug, Clone)]
pub enum PlayingState {
    Morning {
        ends_at: SDateTime,
    },
    MorningVote {
        ends_at: SDateTime,
        votes: HashMap<PlayerName, PlayerName>,
    },
    MorningVoteResults {
        ends_at: SDateTime,
        votes: HashMap<PlayerName, PlayerName>,
        voted: Option<PlayerName>,
    },
    LastWords {
        ends_at: SDateTime,
        voted: PlayerName,
    },
    PostVote {
        ends_at: SDateTime,
    },
    Night {
        ends_at: SDateTime,
        actions: HashMap<PlayerName, NightAction>,
    },
}

impl PlayingState {
    #[cfg(test)]
    pub fn gen_night() -> Self {
        PlayingState::Night {
            ends_at: Utc::now() + Duration::minutes(2),
            actions: HashMap::new(),
        }
    }
}
