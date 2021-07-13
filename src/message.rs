/*
use serde_json;
use serde_json::Value;
use serde_json::from_slice;
use serde_json::json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct JS8Message {
{
    "params": {
        "NAME": "JS8Call", 
        "UTC": 1624587300068, 
        "VERSION": "2.2.0", 
        "_ID": "125288100068"
    }, 
    "type": "PING", 
    "value": ""
}


}

let json: Value = from_slice(&read_buffer[0 .. bytes_read])?;


*/
 
pub struct Message {
    buffer: [u8; 1024],
}


impl Message {

    pub fn new(buffer: [u8; 1024]) -> Self {
        Self {
            buffer
        }
    }

    pub fn display(self) {
        println!("Received message: {}", String::from_utf8_lossy(&self.buffer));
    }
}
