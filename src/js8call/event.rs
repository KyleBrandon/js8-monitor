use super::parse_error::ParseError;
use crate::js8call::message::message_type::MessageType;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;


/// JS8PubSub
///     Define how to publish and subscribe to JS8 Events. 
/// 
/// 
pub trait JS8PubSub {

    /// publish
    ///     Publish a new JS8 Event
    /// members:
    ///     event - this is the JS8 Event to publish.
    fn publish(&self, event: &Event) -> Result<(), JS8PubSubError>;

    /// subscribe
    ///     Subscribe to a new channel to receive JS8 Events
    /// members:
    ///     channel -   the event channel to subscribe to.
    ///     func    -   the function to call when the event is received.
    fn subscribe<F>(&self, func: F) -> Result<(), JS8PubSubError> where
        F: FnMut(Event);
}

pub enum JS8PubSubError {
    UnableToPublish,
    UnableToSubscribe,
}

impl JS8PubSubError {
    fn message(&self) -> &str {
        match self {
            Self::UnableToPublish => "Unable to publish.",
            Self::UnableToSubscribe => "Unable to subscribe",
        }
    }
}

impl Display for JS8PubSubError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for JS8PubSubError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

/// Event
///     This structure is used to represent an event from JS8Call.
/// 
/// members:
///     raw_event       -   this is a string representation of the event JSON.
///     json            -   this is a serde_json structure
///     message_type    -   this is the type of JS8Call message that was triggered.
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    raw_event: String,
    json: Value,
    message_type: MessageType,
}

impl Event {

    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }

    pub fn json(&self) -> &Value {
        &self.json
    }
}

impl TryFrom<&[u8]> for Event {
    type Error = ParseError;

    /// try_from
    ///     Convert from a u8 buffer to the Event structure.
    /// 
    fn try_from(buf: &[u8]) -> Result<Event, Self::Error> {
        let raw_event = str::from_utf8(buf)?;
        let json: Value = serde_json::from_str(raw_event)?;
        let event_type = json["type"].to_string();
        let message_type: MessageType = event_type.parse()?;

        Ok(Self {
            raw_event: String::from(raw_event),
            json: json,
            message_type: message_type,
        })
    }
}
