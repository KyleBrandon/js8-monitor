use crate::js8call::Event;
use crate::js8call::JS8PubSub;
use super::pubsub::JS8RedisPubSub; 
use log::{error, trace};
use std::convert::TryFrom;
use std::net::UdpSocket;
use tokio::task::JoinHandle;

/// Create a monitor for JS8Call events and run it in a task.
/// 
pub async fn monitor_factory(address: String, is_in_test_mode: bool) -> JoinHandle<()> {
    tokio::spawn(async move {
        if is_in_test_mode {
            trace!("Create a test monitor");
        } else {
            let pubsub = JS8RedisPubSub::new();
            JS8Monitor::new(address, Box::new(pubsub)).start();
        }
    })
}

/// JS8Monitor struct.
struct JS8Monitor {
    address: String,
    pubsub: Box<dyn JS8PubSub>
}

/// Implementation of the JS8Monitor.
impl JS8Monitor {

    /// Create a new JS8Monitor.
    ///
    fn new(address: String, pubsub: Box<dyn JS8PubSub>) -> Self {
        Self {
            address,
            pubsub,
        }
    }

    /// Start the monitor processing events from JS8Call and place then on the event queue.
    fn start(&self) {
        trace!("Listening on: {}", self.address);

        let socket = UdpSocket::bind(&self.address).unwrap();
        loop {
            let mut buffer = [0; 1024];
            match socket.recv_from(&mut buffer) {
                Ok((len, _)) => {
                    //trace!(target: "monitor-trace", "Message received");
                    match Event::try_from(&buffer[..len]) {
                        Ok(event) => {
                            self.pubsub.publish(&event);
                        },
                        Err(e) => {
                            error!("Invalid message read from JS8Call: {}", e);
                        },
                    };
                },
                Err(e) => {
                    error!("Failed to read event: {}", e);
                },
            }
        }
    }
}