use crate::core::night_action::NightAction;
use crate::core::player::PlayerRole;
use crate::core::PlayerName;

pub enum Error {
    NameNotFound(PlayerName),
    InvalidNightAction(PlayerRole, NightAction),
    WrongStateForAction,
}

pub type Result<T> = std::result::Result<T, Error>;
