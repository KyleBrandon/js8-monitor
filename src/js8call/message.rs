use std::str::FromStr;

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

impl FromStr for Message {
    type Err = MessageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PING" => Ok(Self::PING),
            "RX_DIRECTED" => Ok(Self::RX_DIRECTED),
            "RX_SPOT" => Ok(Self::RX_SPOT),
            "RX_ACTIVITY" => Ok(Self::RX_ACTIVITY),
            "STATION_STATUS" => Ok(Self::STATION_STATUS),
            "RIG_PTT" => Ok(Self::RIG_PTT),
            "TX_FRAME" => Ok(Self::TX_FRAME),
            "CLOSE" => Ok(Self::CLOSE),
            _ => Err(MessageError)
        }
    }
}

pub struct MessageError;