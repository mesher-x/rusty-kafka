use serde::{Deserialize, Serialize};
use serde_json::Result as JsonResult;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    InitMessage(InitMessage),
    RegularMessage(RegularMessage),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitMessage {
    pub method: String,
    pub topic: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegularMessage {
    pub message: String,
}

impl Message {
    pub fn to_json(&self) -> JsonResult<String> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> JsonResult<Self> {
        serde_json::from_str(json)
    }
}
