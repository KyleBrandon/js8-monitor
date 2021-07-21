use std::str::FromStr;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub enum Message {
    PING,
    RX_DIRECTED,
    RX_SPOT,
    RX_ACTIVITY,
    STATION_STATUS,
    RIG_PTT,
    TX_FRAME,
    CLOSE,
}

impl Message {
    fn message(&self) -> &str {
        match self {
            Self::PING => "Ping",
            Self::RX_DIRECTED => "RX Directed",
            Self::RX_SPOT => "RX Spot",
            Self::RX_ACTIVITY => "RX Activity",
            Self::STATION_STATUS => "Station Status",
            Self::RIG_PTT => "Rig PTT",
            Self::TX_FRAME => "TX Frame",
            Self::CLOSE => "Close", 
        }
    }
}

impl FromStr for Message {
    type Err = MessageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PING" => Ok(Self::PING),
            "RX.DIRECTED" => Ok(Self::RX_DIRECTED),
            "RX.SPOT" => Ok(Self::RX_SPOT),
            "RX.ACTIVITY" => Ok(Self::RX_ACTIVITY),
            "STATION_STATUS" => Ok(Self::STATION_STATUS),
            "RIG.PTT" => Ok(Self::RIG_PTT),
            "TX.FRAME" => Ok(Self::TX_FRAME),
            "CLOSE" => Ok(Self::CLOSE),
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