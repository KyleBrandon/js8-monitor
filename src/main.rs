#[macro_use] extern crate rocket;

use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, App, ArgMatches};
use std::env;
use tokio::join;
use js8monitor::monitor;
use js8api::server;

pub mod js8api;
pub mod js8monitor;


// Read the command line arguments
// TODO: Move this out
fn read_commandline() -> ArgMatches<'static> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("js8_address")
            .short("ja")
            .long("js8_address")
            .value_name("IP_ADDRESS:PORT")
            .help("Sets the address that the JS8Call API is configured to")
            .takes_value(true))
        .arg(Arg::with_name("api_address")
            .short("aa")
            .long("api_address")
            .value_name("IP_ADDRESS:PORT")
            .help("Sets the REST API address")
            .takes_value(true))
        .arg(Arg::with_name("test")
            .short("t")
            .long("test")
            .help("Runs a test version of the JS8 Monitor")
            .takes_value(true))
        .get_matches();

    matches
}

fn get_js8_address(matches: &ArgMatches) -> String {
    matches.value_of("js8_address")
        .map(|s| s.to_owned())
        .or(env::var("JS8_ADDRESS").ok())
        .and_then(|addr| addr.parse().ok())
        .or_else(|| Some("127.0.0.1:2242".to_string()))
        .unwrap()
}

fn get_redis_address(matches: &ArgMatches) -> String {
    matches.value_of("redis_address")
        .map(|s| s.to_owned())
        .or(env::var("REDIS_ADDRESS").ok())
        .and_then(|addr| addr.parse().ok())
        .or_else(|| Some("redis://10.0.4.109:6379".to_string()))
        .unwrap()
}

fn get_api_address(matches: &ArgMatches) -> (String, u16) {
    let address = matches.value_of("api_address")
        .map(|s| s.to_owned())
        .or(env::var("API_ADDRESS").ok())
        .and_then(|addr| addr.parse().ok())
        .or_else(|| Some("127.0.0.1".to_string()))
        .unwrap();

    let port = matches.value_of("api_port")
        .map(|s| s.to_owned())
        .or(env::var("API_PORT").ok())
        .and_then(|addr| addr.parse().ok())
        .or_else(|| Some("8000".to_string()))
        .unwrap();

    (address, port.parse::<u16>().unwrap())
}

fn is_in_test_mode(matches: &ArgMatches) -> bool {
    matches.value_of("test")
        .map(|s| s.to_owned())
        .or(env::var("TEST").ok())
        .and_then(|addr| addr.parse().ok())
        .or_else(|| Some(false))
        .unwrap()
}

pub struct Configuration {
    js8_address: String,
    redis_address: String,
    api_address: String,
    api_port: u16,
}

#[rocket::main]
async fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let matches = read_commandline();
    let js8_address = get_js8_address(&matches);
    let redis_address = get_redis_address(&matches);
    let api = get_api_address(&matches);
    let _is_in_test_mode = is_in_test_mode(&matches);

    let config = Configuration {
        js8_address,
        redis_address,
        api_address: api.0,
        api_port: api.1,
    };

    let monitor_handle = monitor::monitor_factory(config.js8_address, config.redis_address.clone());
    let api_handle = server::server_factory(config.api_address, config.api_port);

    join!(monitor_handle, api_handle);
}
