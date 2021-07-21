use monitor::Monitor;

pub mod js8call;
pub mod monitor;

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let monitor = Monitor::new("127.0.0.1:2242".to_string());
    monitor.run();
}