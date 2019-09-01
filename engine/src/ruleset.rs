use crate::phases::Morning;
use crate::state::Phase;
use crate::util::{de_opt_dur, se_opt_dur};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Ruleset {
    #[serde(serialize_with = "se_opt_dur", deserialize_with = "de_opt_dur")]
    morning_limit: Option<Duration>,

    #[serde(serialize_with = "se_opt_dur", deserialize_with = "de_opt_dur")]
    vote_limit: Option<Duration>,
}

impl Ruleset {
    pub fn morning_end(&self) -> Option<DateTime<Utc>> {
        match self.morning_limit {
            Some(l) => Some(Utc::now() + l),
            None => None,
        }
    }

    pub fn vote_end(&self) -> Option<DateTime<Utc>> {
        match self.vote_limit {
            Some(l) => Some(Utc::now() + l),
            None => None,
        }
    }

    pub fn init_phase(&self) -> Phase {
        Morning {}.into()
    }
}
