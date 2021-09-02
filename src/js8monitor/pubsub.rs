use crate::js8call::event::{JS8PubSub, Event};
use log::{error, trace};
use redis::{Client, Connection, ControlFlow, Msg, PubSubCommands};


pub struct JS8RedisPubSub {

}

impl JS8RedisPubSub {

    pub fn new() -> Self {
        Self {

        }

    }
}

impl JS8PubSub for JS8RedisPubSub {

    fn publish(&self, event: &Event) {
        trace!("Event received: {}", event.message_type());
        /*
        let client: Client = redis::Client::open("redis://127.0.0.1:6379")?;
        let mut con: Connection = client.get_connection()?;
    
        let json: String = serde_json::to_string(&message)?;
    
        con.publish(message.channel, json)?;
        */
    
    }

    fn subscribe(&self, channel: String) {
        /*
        let _ = tokio::spawn(async move {
            let client: Client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
            let mut con: Connection = client.get_connection().unwrap();
    
            println!("subscribed");
            let _: () = con.subscribe(&[channel], |msg: Msg| {
                println!("received msg");
                let received: String = msg.get_payload().unwrap();
                let message_obj: Message = serde_json::from_str::<Message>(&received).unwrap();
    
                message_handler::handle(message_obj);
                return ControlFlow::Continue;
            }).unwrap();
        });
        */ 
    }

}

