use monitor::Monitor;

pub mod monitor;
pub mod message;

fn main() {
    let monitor = Monitor::new("127.0.0.1:2242".to_string());
    monitor.run();
}