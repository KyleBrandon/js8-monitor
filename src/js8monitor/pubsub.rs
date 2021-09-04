use crate::js8call::event::{JS8PubSub, JS8PubSubError, Event};
use log::trace;
use redis::{Commands, Client, Connection, ControlFlow, Msg, PubSubCommands, RedisError};


/// JS8RedisPubSub
///     This will implement the JS8PubSub methods for Redis.
/// 
pub struct JS8RedisPubSub {
    address: String,
    client: Client,
}

/// JS8RedisPubSub
///     Behavior for the JS8RedisPubSub struct.
/// 
impl JS8RedisPubSub {

    pub fn new(address: String) -> Self {

        Self {
            address: address.clone(),
            client: redis::Client::open(address).unwrap(),
        }

    }
}

/// Convert from a RedisError to a JS8PubSubError
///     
impl From<RedisError> for JS8PubSubError {
    fn from(_: RedisError) -> Self {
        Self::UnableToPublish
    }
}

/// Convert from a serde_json::Error to a JS8PubSubError
/// 
impl From<serde_json::Error> for JS8PubSubError {
    fn from(_: serde_json::Error) -> Self {
        Self::UnableToPublish
    }
}

impl JS8PubSub for JS8RedisPubSub {

    fn publish(&self, event: &Event) -> Result<(), JS8PubSubError> {
        trace!("Event published: {}", event.message_type());
        let mut con: Connection = self.client.get_connection().unwrap();
        let json: String = serde_json::to_string(event)?;
        con.publish(String::from("JS8CALL"), json)?;

        Ok(())
    }


    fn subscribe<F>(&self, mut func: F) -> Result<(), JS8PubSubError> where
        F: FnMut(Event) {

        let mut con: Connection = self.client.get_connection().unwrap();

        con.subscribe(&[String::from("JS8CALL")], move |msg: Msg| {
            let received: String = msg.get_payload().unwrap();
            let event: Event = serde_json::from_str::<Event>(&received).unwrap();

            func(event);

            return ControlFlow::Continue;
        })?
    }

}
