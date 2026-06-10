/// Common utilities for examples.
use std::env;

/// Reads the given required environment variables, returning their values in
/// order. If any are missing, the returned error lists *all* of them so the
/// user can set everything at once instead of discovering them one at a time.
#[allow(dead_code)]
pub fn require_env_vars<const N: usize>(names: [&str; N]) -> Result<[String; N], String> {
    let missing: Vec<&str> = names
        .iter()
        .copied()
        .filter(|name| env::var(name).is_err())
        .collect();

    if !missing.is_empty() {
        return Err(format!(
            "missing required environment variable(s): {}",
            missing.join(", ")
        ));
    }

    // safe: every name was just confirmed to be present above.
    Ok(names.map(|name| env::var(name).expect("checked present above")))
}

pub fn setup_logger() {
    let filters = env::var("RUST_LOG").unwrap_or_else(|_| "tapo=info".to_string());

    env_logger::Builder::new().parse_filters(&filters).init();
}

#[allow(dead_code)]
fn main() {
    println!("This is not a real example.");
    println!("This entry point has been included solely to prevent a warning about its absence.");
}
