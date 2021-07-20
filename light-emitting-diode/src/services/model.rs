// src/requests/model.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommandTypes {
    Blink,
    Switch
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub pin: u8,
    pub command_type: CommandTypes,
    pub duration: u64
}