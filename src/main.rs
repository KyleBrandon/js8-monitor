#[macro_use] extern crate rocket;
use tokio::join;

pub mod js8call;
pub mod monitor;

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let monitor_handle = monitor::monitor_js8("127.0.0.1:2242".to_string());

    let rocket_handle = rocket::build()
        .mount("/", routes![world])
        .launch();

    join!(monitor_handle, rocket_handle);
}
