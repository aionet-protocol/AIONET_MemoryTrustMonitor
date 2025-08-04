use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

pub fn log_entropy_event(event: &str) {
    let now = Utc::now();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/entropy_log.txt")
        .expect("Failed to open entropy log file");

    writeln!(file, "[{}] {}", now.to_rfc3339(), event).expect("Failed to write to log");
}
