extern crate rocket;
use log::{trace};
use rocket::{Error as RocketError};
use crate::js8api::api;


/// Create a server for the API for JS8Call commands
/// 
pub async fn server_factory(address: String, port: u16, is_in_test_mode: bool) -> Result<(), RocketError> {
    trace!("Start the API server");

    let figment = rocket::Config::figment()
        .merge(("address", address))
        .merge(("port", port));

    rocket::custom(figment)
        .mount("/", routes![api::world])
        .launch().await
}

