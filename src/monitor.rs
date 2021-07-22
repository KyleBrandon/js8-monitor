use crate::js8call::Event;
use crate::js8call::message::rx_spot::RxSpot;
use log::{error};
use std::convert::TryFrom;
use std::net::UdpSocket;

pub struct Monitor {
    address: String,
}

impl Monitor {

    pub fn new(address: String) -> Self {
        Self {
            address,
        }
    }

    pub fn run(self) {
        println!("Listening on: {}", self.address);

        let socket = UdpSocket::bind(&self.address).unwrap();
        loop {
            // TODO: Update the lifetime of the buffer that's read to keep it persistent
            let mut buffer = [0; 1024];
            match socket.recv_from(&mut buffer) {
                Ok((_, _)) => {
                    //trace!(target: "monitor-trace", "Message received");
                    match Event::try_from(&buffer[..]) {
                        Ok(event) => {

                            match RxSpot::try_from(event) {
                                Ok(_rx_spot) => {

                                    unimplemented!()
                                },
                                Err(e) => {
                                    error!("Could not convert to RX.SPOT: {}", e);
                                }
                            }
                        },
                        Err(e) => {
                            // TODO: Try to log the JSON that was in error
                            error!("Invalid message read from JS8Call: {}", e);
                        },
                    };

                    // TODO do something with Event
                },
                Err(e) => {
                    println!("Failed to read event: {}", e);
                },
            }
        }
    }
}
