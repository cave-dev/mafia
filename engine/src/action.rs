use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Action {
    #[serde(flatten)]
    pub a: ActionE,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ActionE {
    Message(ActionMessage),
}

#[derive(Serialize, Deserialize)]
pub struct ActionMessage {
    pub text: String,
}
