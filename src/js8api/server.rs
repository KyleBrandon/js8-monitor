extern crate rocket;
use log::{trace};
use rocket::{Error as RocketError};
use crate::js8api::api;
use crate::js8call::event::Event;
use crate::js8call::event::JS8PubSub;
use crate::js8monitor::pubsub::JS8RedisPubSub;


/// Create a server for the API for JS8Call commands
/// 
pub async fn server_factory(address: String, port: u16) -> Result<(), RocketError> {
    trace!("Start the API server");

    let figment = rocket::Config::figment()
        .merge(("address", address))
        .merge(("port", port));

    let pubsub = JS8RedisPubSub::new(String::from("redis://127.0.0.1:6379"));
    pubsub.subscribe(|event: Event| {
        trace!("WebServer received: {}", event.message_type());
    }).unwrap();
    trace!("Registered pubsub scribe callback");

    rocket::custom(figment)
        .mount("/", routes![api::world])
        .launch().await
}

