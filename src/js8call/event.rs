use super::parse_error::ParseError;
use crate::js8call::message::message_type::MessageType;
//use log::trace;
use serde_json::Value;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::str;

/// Event
///     This structure is used to represent an event from JS8Call.
/// 
/// members:
///     raw_event       -   this is a string representation of the event JSON.
///     json            -   this is a serde_json structure
///     message_type    -   this is the type of JS8Call message that was triggered.
#[derive(Debug)]
pub struct Event<'buf> {
    raw_event: &'buf str,
    json: Value,
    message_type: MessageType,
}

impl<'buf> Event<'buf> {
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }

    pub fn json(&self) -> &Value {
        &self.json
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Event<'buf> {
    type Error = ParseError;

    /// try_from
    ///     Convert from a u8 buffer to the Event structure.
    /// 
    fn try_from(buf: &'buf [u8]) -> Result<Event<'buf>, Self::Error> {
        let raw_event = str::from_utf8(buf)?;
        let json: Value = serde_json::from_str(raw_event)?;
        let event_type = json["type"].to_string();
        let message_type: MessageType = event_type.parse()?;

        Ok(Self {
            raw_event,
            json,
            message_type,
        })
    }
}
