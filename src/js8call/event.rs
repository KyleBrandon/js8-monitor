use super::message::{Message, MessageError};
use serde_json::Value;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;


#[derive(Debug)]
pub struct Event<'buf> {
    raw_event: &'buf str,
    message: Message,
}

impl<'buf> TryFrom<&'buf [u8]> for Event<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Event<'buf>, Self::Error> {
        let raw_event = str::from_utf8(buf)?;

        let msg_json: Value = serde_json::from_str(raw_event)?;
        let event_type = msg_json["type"].to_string();
        let message: Message = event_type.parse()?;

        Ok(Self {
            raw_event,
            message,
        })
    }

}

pub enum ParseError {
    InvalidEvent,
    InvalidMessage,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidEvent => "Invalid Event",
            Self::InvalidMessage => "Invalid Message",
        }
    }
}

impl From<MessageError> for ParseError {
    fn from(_: MessageError) -> Self {
        Self::InvalidMessage
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEvent
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(_: serde_json::Error) -> Self {
        Self::InvalidEvent
    }
}
