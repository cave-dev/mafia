use crate::morning::MorningState;
use crate::state::StateExt;
use crate::util::{de_opt_dur, se_opt_dur};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Ruleset {
    #[serde(serialize_with = "se_opt_dur", deserialize_with = "de_opt_dur")]
    day_limit: Option<Duration>,
}

impl Ruleset {
    pub fn day_end(&self) -> Option<DateTime<Utc>> {
        match self.day_limit {
            Some(l) => Some(Utc::now() + l),
            None => None,
        }
    }

    pub fn init_morning(&self) -> StateExt {
        MorningState {}.into()
    }
}
