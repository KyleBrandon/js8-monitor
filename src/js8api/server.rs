extern crate rocket;
use log::{trace};
use rocket::{Error as RocketError};
use std::convert::TryFrom;
use crate::js8api::api;
use crate::js8call::event::{Event, JS8PubSub};
use crate::js8call::message::MessageType;
use crate::js8call::message::rx_activity::RxActivity;
use crate::js8call::message::rx_spot::RxSpot;
use crate::js8monitor::pubsub::JS8RedisPubSub;


/// Create a server for the API for JS8Call commands
/// 
pub async fn server_factory(address: String, port: u16, redis_address: String) -> Result<(), RocketError> {
    trace!("Start the API server");

    let figment = rocket::Config::figment()
        .merge(("address", address))
        .merge(("port", port));

    let pubsub = JS8RedisPubSub::new(redis_address);
    pubsub.subscribe(|event: Event| {
        if *event.message_type() == MessageType::RxActivity {
            
            let activity = RxActivity::try_from(event);
            trace!("{:?}", activity);

        } else if *event.message_type() == MessageType::RxSpot {
            let spot = RxSpot::try_from(event);
            trace!("{:?}", spot);

        } else {
            trace!("WebServer received: {}", event.message_type());
        }
    }).unwrap();
    trace!("Registered pubsub scribe callback");

    rocket::custom(figment)
        .mount("/", routes![api::world])
        .launch().await
}

