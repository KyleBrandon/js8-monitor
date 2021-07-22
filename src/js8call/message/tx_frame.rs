use serde::{Serialize, Deserialize};
use std::convert::TryFrom;
use crate::js8call::event::Event;
use crate::js8call::parse_error::ParseError;
use crate::js8call::message_type::MessageType;



#[derive(Debug, Serialize, Deserialize)]
struct TxFrame {
    #[serde(rename="TONES")]
    tones: Vec<u8>,
    #[serde(rename="_ID")]
    id: i64,
}

impl<'buf> TryFrom<Event<'buf>> for TxFrame {
    type Error = ParseError;

    fn try_from(e: Event<'buf>) -> Result<TxFrame, Self::Error> {
        if *e.message_type() != MessageType::TxFrame {
            return Err(ParseError::InvalidMessageType);
        }

        let tx_frame: TxFrame = serde_json::from_str(&e.json()["params"].to_string())?;

        Ok(tx_frame)
    }

}
