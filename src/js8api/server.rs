extern crate rocket;
use log::{trace};
use rocket::{Error as RocketError};
use std::convert::TryFrom;
use crate::js8api::api;
use js8event::event::{Event, JS8PubSub};
use js8event::pubsub::JS8RedisPubSub;
use js8event::message::MessageType;
use js8event::message::rx_activity::RxActivity;
use js8event::message::rx_spot::RxSpot;


fn subscribe(redis_address: String) {
    let pubsub = JS8RedisPubSub::new(redis_address);
    pubsub.subscribe(|event: Event| {
        if *event.message_type() == MessageType::RxActivity {
            
            let activity = RxActivity::try_from(event);
            trace!("WebServer: {:?}", activity);

        } else if *event.message_type() == MessageType::RxSpot {
            let spot = RxSpot::try_from(event);
            trace!("WebServer: {:?}", spot);

        } else {
            trace!("WebServer: {}", event.message_type());
        }
    }).unwrap();
    trace!("Registered pubsub scribe callback");
}

/// Create a server for the API for JS8Call commands
/// 
pub async fn server_factory(address: String, port: u16, redis_address: String) -> Result<(), RocketError> {
    trace!("Start the API server");

    // subscribe to the Redis queue
    subscribe(redis_address);

    let figment = rocket::Config::figment()
        .merge(("address", address))
        .merge(("port", port));

    rocket::custom(figment)
        .mount("/", routes![api::world])
        .launch().await
}

