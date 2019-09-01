use chrono::Duration;
use serde::{Deserialize, Deserializer, Serializer};

pub fn se_opt_dur<S: Serializer>(dur: &Option<Duration>, s: S) -> Result<S::Ok, S::Error> {
    match dur {
        Some(d) => s.serialize_some(&d.num_seconds()),
        None => s.serialize_none(),
    }
}

pub fn de_opt_dur<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Duration>, D::Error> {
    Ok(Option::deserialize(d)?.map(Duration::seconds))
}
