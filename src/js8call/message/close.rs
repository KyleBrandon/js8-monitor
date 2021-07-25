use serde::{Serialize, Deserialize};
use std::convert::TryFrom;
use crate::js8call::event::Event;
use crate::js8call::parse_error::ParseError;
use super::message_type::MessageType;


#[derive(Debug, Serialize, Deserialize)]
pub struct Close {
    #[serde(rename="_ID")]
    id: String,
}

impl<'buf> TryFrom<Event<'buf>> for Close {
    type Error = ParseError;

    fn try_from(e: Event<'buf>) -> Result<Close, Self::Error> {
        if *e.message_type() != MessageType::Close {
            return Err(ParseError::InvalidMessageType);
        }

        let result: Close = serde_json::from_str(&e.json()["params"].to_string())?;

        Ok(result)
    }

}
