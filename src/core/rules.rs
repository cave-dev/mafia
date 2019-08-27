use crate::core::playing_state::PlayingState;
use chrono::{Duration, Utc};

#[derive(Debug, Clone)]
pub struct Rules {
    morning_duration: Duration,
    night_duration: Duration,
}

impl Rules {
    pub fn initial_state(&self) -> PlayingState {
        PlayingState::Morning {
            ends_at: Utc::now() + self.morning_duration,
        }
    }
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            morning_duration: Duration::minutes(5),
            night_duration: Duration::seconds(90),
        }
    }
}
