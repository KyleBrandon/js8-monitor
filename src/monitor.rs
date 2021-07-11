use std::io;
use std::net::UdpSocket;

pub struct Monitor {
    address: String,
}

impl Monitor {

    pub fn new(address: String) -> Self {
        Self {
            address
        }
    }

    fn read_from(&self, socket: &UdpSocket, buffer: &mut [u8]) -> io::Result<String> {
        match socket.recv_from(buffer) {
            Ok((_, _)) => {
                let string = String::from_utf8_lossy(&buffer);
                println!("Received: {}", string.to_string());
                Ok("".to_string())
            },
            Err(e) => {
                Err(e)
            },
        }
    }

    pub fn run(self) {
        println!("Listening on: {}", self.address);

        let mut buffer = [0; 1024];

        let socket = UdpSocket::bind(&self.address).unwrap();
        loop {
            match self.read_from(&socket, &mut buffer) {
                Ok(string) => {
                },
                Err(e) => {
                    dbg!("Error reading from socket: {}", e);
                },
            }

        }

    }
}
