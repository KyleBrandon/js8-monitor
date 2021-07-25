use crate::js8call::parse_error::ParseError;
use log::{error, trace};
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
            "\"PING\"" => {
                trace!("MessageType::Ping");
                Ok(Self::Ping)
            },
            "\"RX.DIRECTED\"" => {
                trace!("MessageType::RxDirected");
                Ok(Self::RxDirected)
            },
            "\"RX.SPOT\"" => {
                trace!("MessageType::RxSpot");
                Ok(Self::RxSpot)
            },
            "\"RX.ACTIVITY\"" => {
                trace!("MessageType::RxActivity");
                Ok(Self::RxActivity)
            },
            "\"STATION.STATUS\"" => {
                trace!("MessageType::StationStatus");
                Ok(Self::StationStatus)
            },
            "\"RIG.PTT\"" => {
                trace!("MessageType::RigPtt");
                Ok(Self::RigPtt)
            },
            "\"TX.FRAME\"" => {
                trace!("MessageType::TxFrame");
                Ok(Self::TxFrame)
            },
            "\"CLOSE\"" => {
                trace!("MessageType::Close");
                Ok(Self::Close)
            },
            _ => {
                error!("MessageType::Unknown {}", s);
                Err(ParseError::InvalidMessage)
            }
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
