use std::str::FromStr;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub enum Message {
    Ping,
    RxDirected,
    RxSpot,
    RxActivity,
    StationStatus,
    RigPtt,
    TxFrame,
    Close,
}

impl Message {
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

impl FromStr for Message {
    type Err = MessageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PING" => Ok(Self::Ping),
            "RX.DIRECTED" => Ok(Self::RxDirected),
            "RX.SPOT" => Ok(Self::RxSpot),
            "RX.ACTIVITY" => Ok(Self::RxActivity),
            "STATION_STATUS" => Ok(Self::StationStatus),
            "RIG.PTT" => Ok(Self::RigPtt),
            "TX.FRAME" => Ok(Self::TxFrame),
            "CLOSE" => Ok(Self::Close),
            _ => Err(MessageError)
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }

}

impl Debug for Message {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

pub struct MessageError;