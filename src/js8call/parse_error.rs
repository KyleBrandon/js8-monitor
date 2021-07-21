use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;

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

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(_: serde_json::Error) -> Self {
        Self::InvalidEvent
    }
}