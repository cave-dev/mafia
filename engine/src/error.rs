use crate::player::PlayerName;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize)]
pub enum Error {
    InvalidPlayerName(PlayerName),
}
