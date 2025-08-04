mod entropy_logger;

// pub fn log_drift_placeholder() {
//     println!("(Placeholder) Drift signature logic coming soon...");
// }

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "log" => entropy_logger::log_entropy_event("PoM logger test"),
            _ => println!("Unknown command"),
        }
    } else {
        println!("Usage: cargo run log");
    }

    // entropy_logger::log_drift_placeholder(); // PoD integration coming here
}
