use serde::{Serialize, Deserialize};
use std::convert::TryFrom;
use crate::js8call::event::Event;
use crate::js8call::parse_error::ParseError;
use super::message_type::MessageType;


#[derive(Debug, Serialize, Deserialize)]
pub struct RigPtt {
    #[serde(rename="PTT")]
    ptt: bool,
    #[serde(rename="UTC")]
    utc: u64,
    #[serde(rename="_ID")]
    id: i64,
}


impl<'buf> TryFrom<Event<'buf>> for RigPtt {
    type Error = ParseError;

    fn try_from(e: Event<'buf>) -> Result<RigPtt, Self::Error> {
        if *e.message_type() != MessageType::RigPtt {
            return Err(ParseError::InvalidMessageType);
        }

        let rig_ptt: RigPtt = serde_json::from_str(&e.json()["params"].to_string())?;

        Ok(rig_ptt)
    }

}
