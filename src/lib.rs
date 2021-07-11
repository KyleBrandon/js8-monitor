pub mod monitor;
pub mod message;

#[cfg(test)]
mod tests {
    use super::monitor::Monitor;

    #[test]
    fn it_works() {
        let monitor = Monitor::new("127.0.0.1:2242".to_string());
        monitor.run();
    }
}