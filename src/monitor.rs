use std::io::Result;
use std::net::UdpSocket;
use std::str;
use crate::message::Message2;
use serde_json::Value;

pub struct Monitor {
    address: String,
}

impl Monitor {

    pub fn new(address: String) -> Self {
        Self {
            address,
        }
    }

    // TODO: Update the lifetime of the buffer that's read to keep it persistent
    fn read_from(&self, socket: &UdpSocket) -> Result<Message2> {
        let mut buffer = [0; 1024];
        match socket.recv_from(&mut buffer) {
            Ok((bytes_read, _)) => {
                // TODO: change this to use '?' and create a MonitorError
                let new_msg = str::from_utf8(&buffer).unwrap();

                // TODO: Update to Message::try_from(&buffer[..bytes_read])
                Ok(Message2::new(&new_msg))
            },
            Err(e) => {
                Err(e)
            },
        }
    }

    pub fn run(self) {
        println!("Listening on: {}", self.address);


        let socket = UdpSocket::bind(&self.address).unwrap();
        loop {
            match self.read_from(&socket) {
                Ok(message) => {
                    message.display();
                },
                Err(e) => {
                    dbg!("Error reading from socket: {}", e);
                },
            }

        }

    }
}
