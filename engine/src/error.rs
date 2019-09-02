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
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            InvalidPlayerName(p) => write!(f, "invalid player name: {}", p),
            InvalidSession => write!(f, "invalid session id"),
            InvalidSecret => write!(f, "invalid secret"),
        }
    }
}

impl ErrorT for Error {}
