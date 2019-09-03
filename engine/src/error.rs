use crate::player::PlayerName;
use serde::Serialize;
use std::error::Error as ErrorT;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    InvalidPlayerName(PlayerName),
    InvalidSession,
    InvalidSecret,
    PlayerNameTaken(PlayerName),
    InternalError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            InvalidPlayerName(p) => write!(f, "invalid player name: {}", p),
            InvalidSession => write!(f, "invalid session id"),
            InvalidSecret => write!(f, "invalid secret"),
            PlayerNameTaken(p) => write!(f, "name {} is already taken!", p),
            InternalError => write!(f, "internal error"),
        }
    }
}

impl ErrorT for Error {}
