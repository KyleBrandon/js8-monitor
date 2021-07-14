use std::io::Result;
use std::net::UdpSocket;
use crate::message::Message;


pub struct Monitor {
    address: String,
}

impl Monitor {

    pub fn new(address: String) -> Self {
        Self {
            address,
        }
    }

    fn read_from(&self, socket: &UdpSocket) -> Result<Message> {
        let mut buffer = [0; 1024];
        match socket.recv_from(&mut buffer) {
            Ok((bytes_read, _)) => {
                Ok(Message::new(&buffer[0..bytes_read]))
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
