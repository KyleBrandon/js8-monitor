use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct RxSpot {
    #[serde(rename="CALL")]
    call: String,
    #[serde(rename="DIAL")]
    dial: u64,
    #[serde(rename="FREQ")]
    freq: u64,
    #[serde(rename="GRID")]
    grid: String,
    #[serde(rename="OFFSET")]
    offset: i32,
    #[serde(rename="SNR")]
    snr: i8,
    #[serde(rename="_ID")]
    id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct RxDirected {
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

#[derive(Debug, Serialize, Deserialize)]
struct RxActivity {
    #[serde(rename="DIAL")]
    dial: u64,
    #[serde(rename="FREQ")]
    freq: u64,
    #[serde(rename="OFFSET")]
    offset: i32,
    #[serde(rename="SNR")]
    snr: i8,
    #[serde(rename="SPEED")]
    speed: u8,
    #[serde(rename="TDRIFT")]
    tdrift: f64,
    #[serde(rename="UTC")]
    utc: u64,
    #[serde(rename="_ID")]
    id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ping {
    #[serde(rename="NAME")]
    name: String,
    #[serde(rename="UTC")]
    utc: u64,
    #[serde(rename="VERSION")]
    version: String,
    #[serde(rename="_ID")]
    id: String,
}
 
pub struct Message {
    msg_type: String,
    msg_value: String,
    msg_params: String,
}

impl Message {
    

    pub fn new(buffer: &[u8]) -> Self {
        let msg: Value = serde_json::from_slice(&buffer).unwrap();
        let msg_type = msg["type"].to_string();

        match msg_type.as_str() {
            r#""PING""# => {
                // deserialize the PING message
                let ping: Ping = serde_json::from_str(&msg["params"].to_string()).unwrap();
                println!("{:?}", ping);
            }
            r#""RX.DIRECTED""# => {
                let rx_directed: RxDirected = serde_json::from_str(&msg["params"].to_string()).unwrap();
                println!("{:?}", rx_directed);
            }
            r#""RX.SPOT""# => {
                let rx_spot: RxSpot = serde_json::from_str(&msg["params"].to_string()).unwrap();
                println!("{:?}", rx_spot);
            }
            r#""RX.ACTIVITY""# => {
                let rx_activity: RxActivity = serde_json::from_str(&msg["params"].to_string()).unwrap();
                println!("{:?}", rx_activity);
            }
            _ => {
                println!("{}", msg);
            }
        }

        Self {
            msg_type: msg["type"].to_string(),
            msg_value: msg["value"].to_string(),
            msg_params: msg["params"].to_string(),
        }
    }

    pub fn display(self) {
        // println!("{}:{}", &self.msg_type, &self.msg_value);
        // println!("{}", &self.msg_params);
    }
}
