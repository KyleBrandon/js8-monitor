use serde::{Serialize, Deserialize};
use std::convert::TryFrom;
use crate::js8call::event::Event;
use crate::js8call::parse_error::ParseError;
use super::message_type::MessageType;



#[derive(Debug, Serialize, Deserialize)]
pub struct Ping {
    #[serde(rename="NAME")]
    name: String,
    #[serde(rename="UTC")]
    utc: u64,
    #[serde(rename="VERSION")]
    version: String,
    #[serde(rename="_ID")]
    id: String,
}

impl TryFrom<Event> for Ping {
    type Error = ParseError;

    fn try_from(e: Event) -> Result<Ping, Self::Error> {
        if *e.message_type() != MessageType::Ping {
            return Err(ParseError::InvalidMessageType);
        }

        let ping: Ping = serde_json::from_str(&e.json()["params"].to_string())?;

        Ok(ping)
    }

}

 