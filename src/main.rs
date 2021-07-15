use monitor::Monitor;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;

pub mod monitor;
pub mod message;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let monitor = Monitor::new("127.0.0.1:2242".to_string());
    monitor.run();
}