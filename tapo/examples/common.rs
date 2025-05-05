/// Common utilities for examples.
use std::env;

use log::LevelFilter;

pub fn setup_logger() {
    let log_level = env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .parse()
        .unwrap_or(LevelFilter::Info);

    pretty_env_logger::formatted_timed_builder()
        .filter(Some("tapo"), log_level)
        .init();
}

#[allow(dead_code)]
fn main() {
    println!("This is not a real example.");
    println!("This entry point has been included solely to prevent a warning about its absence.");
}
