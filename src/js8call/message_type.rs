use super::parse_error::ParseError;
use std::str::FromStr;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub enum MessageType {
    Ping,
    RxDirected,
    RxSpot,
    RxActivity,
    StationStatus,
    RigPtt,
    TxFrame,
    Close,
}

impl MessageType {
    fn message(&self) -> &str {
        match self {
            Self::Ping => "Ping",
            Self::RxDirected => "RX Directed",
            Self::RxSpot => "RX Spot",
            Self::RxActivity => "RX Activity",
            Self::StationStatus => "Station Status",
            Self::RigPtt => "Rig PTT",
            Self::TxFrame => "TX Frame",
            Self::Close => "Close", 
        }
    }
}

impl PartialEq for MessageType {
    fn eq(&self, other: &Self) -> bool {
        self.message() == other.message()
    }
}

impl FromStr for MessageType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "\"PING\"" => Ok(Self::Ping),
            "\"RX.DIRECTED\"" => Ok(Self::RxDirected),
            "\"RX.SPOT\"" => Ok(Self::RxSpot),
            "\"RX.ACTIVITY\"" => Ok(Self::RxActivity),
            "\"STATION_STATUS\"" => Ok(Self::StationStatus),
            "\"RIG.PTT\"" => Ok(Self::RigPtt),
            "\"TX.FRAME\"" => Ok(Self::TxFrame),
            "\"CLOSE\"" => Ok(Self::Close),
            _ => Err(ParseError::InvalidMessage)
        }
    }
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }

}

impl Debug for MessageType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
