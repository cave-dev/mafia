use crate::error::Error;
use crate::player::PlayerName;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    #[serde(flatten)]
    resp: ResponseE,
}

impl From<Error> for Response {
    fn from(e: Error) -> Self {
        Response { resp: e.into() }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseE {
    Message { from: PlayerName, text: String },
    Error(Error),
}

impl From<Error> for ResponseE {
    fn from(e: Error) -> Self {
        ResponseE::Error(e)
    }
}