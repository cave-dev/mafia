use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Action {
    #[serde(flatten)]
    a: ActionE,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ActionE {
    Message(ActionMessage),
}

#[derive(Serialize, Deserialize)]
pub struct ActionMessage {
    text: String,
}
