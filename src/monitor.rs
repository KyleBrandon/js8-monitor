use std::io;
use std::io::Result;
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


    pub fn run(self) -> Result<()> {
        dbg!("Listening on: {}", self.address);

        let mut buffer = [0; 1024];

        let listener = UdpSocket::bind(self.address)?;
        loop {
            match listener.recv_from(&mut buffer) {
                Ok((_, _)) => {
                    println!("Received: {}", String::from_utf8_lossy(&buffer));
                }
                Err(e) => dbg!("Error reading from socket: {}", e),
            }

        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let monitor = Monitor::new("".to_string());
    }
}