#[macro_use] extern crate rocket;
use std::env;
use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, App, ArgMatches};
use tokio::join;

pub mod js8call;
pub mod monitor;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

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
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
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

fn get_api_address(matches: &ArgMatches) -> String {
    matches.value_of("api_address")
        .map(|s| s.to_owned())
        .or(env::var("API_ADDRESS").ok())
        .and_then(|addr| addr.parse().ok())
        .or_else(|| Some("127.0.0.1:8000".to_string()))
        .unwrap()
}

fn is_in_test_mode(matches: &ArgMatches) -> bool {
    matches.value_of("test")
        .map(|s| s.to_owned())
        .or(env::var("TEST").ok())
        .and_then(|addr| addr.parse().ok())
        .or_else(|| Some(false))
        .unwrap()
}

#[rocket::main]
async fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let matches = read_commandline();
    let js8_address = get_js8_address(&matches);
    let api_address = get_api_address(&matches);
    let is_in_test_mode = is_in_test_mode(&matches);

    let monitor_handle = monitor::monitor_factory(js8_address, is_in_test_mode);

    let rocket_handle = rocket::build()
        .mount("/", routes![world])
        .launch();

    join!(monitor_handle, rocket_handle);
}
