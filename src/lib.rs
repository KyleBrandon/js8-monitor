pub mod monitor;

#[cfg(test)]
mod tests {
    use super::monitor::Monitor;

    #[test]
    fn it_works() {
        let monitor = Monitor::new("172.0.0.1:2242".to_string());
        monitor.run();
    }
}