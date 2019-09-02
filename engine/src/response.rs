use crate::error::Error;
use crate::player::PlayerName;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    #[serde(flatten)]
    resp: ResponseE,
}

impl Response {
    pub fn message(src: Option<PlayerName>, text: String) -> Self {
        Response {
            resp: ResponseE::Message { from: src, text },
        }
    }
}

impl From<Error> for Response {
    fn from(e: Error) -> Self {
        Response { resp: e.into() }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ResponseE {
    Message {
        from: Option<PlayerName>,
        text: String,
    },
    Error(Error),
}

impl From<Error> for ResponseE {
    fn from(e: Error) -> Self {
        ResponseE::Error(e)
    }
}
