use crate::js8call::Event;
use crate::js8call::JS8PubSub;
use super::pubsub::JS8RedisPubSub; 
use log::{error, trace};
use std::convert::TryFrom;
use std::net::UdpSocket;
use tokio::task::JoinHandle;

/// Create a monitor for JS8Call events and run it in a task.
/// 
pub async fn monitor_factory(js8_address: String, redis_address: String) -> JoinHandle<()> {
    tokio::spawn(async move {

        let pubsub = JS8RedisPubSub::new(redis_address);
        trace!("Listening on: {}", js8_address);

        let socket = UdpSocket::bind(&js8_address).unwrap();
        loop {

            // TODO: Need shared state to determine if we should exit

            let mut buffer = [0; 1024];
            match socket.recv_from(&mut buffer) {
                Ok((len, _)) => {
                    //trace!(target: "monitor-trace", "Message received");
                    match Event::try_from(&buffer[..len]) {
                        Ok(event) => {
                            let result = pubsub.publish(&event);
                            if let Err(e) = result {
                                error!("Failed to subscribe to channel: {}", e);
                            }
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
    })
}
