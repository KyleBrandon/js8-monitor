use serde::{Serialize, Deserialize};
use std::convert::TryFrom;
use crate::js8call::event::Event;
use crate::js8call::parse_error::ParseError;
use super::message_type::MessageType;


#[derive(Debug, Serialize, Deserialize)]
pub struct RxDirected {
    #[serde(rename="CMD")]
    cmd: String,
    #[serde(rename="DIAL")]
    dial: u64,
    #[serde(rename="EXTRA")]
    extra: String,
    #[serde(rename="FREQ")]
    freq: u64,
    #[serde(rename="FROM")]
    from: String,
    #[serde(rename="GRID")]
    grid: String,
    #[serde(rename="OFFSET")]
    offset: i32,
    #[serde(rename="SNR")]
    snr: i8,
    #[serde(rename="SPEED")]
    speed: u8,
    #[serde(rename="TDRIFT")]
    tdrift: f64,
    #[serde(rename="TEXT")]
    text: String,
    #[serde(rename="TO")]
    to: String,
    #[serde(rename="UTC")]
    utc: u64,
    #[serde(rename="_ID")]
    id: i64,
}


impl TryFrom<Event> for RxDirected {
    type Error = ParseError;

    fn try_from(e: Event) -> Result<RxDirected, Self::Error> {
        if *e.message_type() != MessageType::RxDirected {
            return Err(ParseError::InvalidMessageType);
        }

        let rx_directed: RxDirected = serde_json::from_str(&e.json()["params"].to_string())?;

        Ok(rx_directed)
    }

}
