//! H110 IR Example
//!
//! Connects to a Tapo H110 hub via the H100 handler and sends stored IR keys
//! to child remotes (`SMART.TAPOREMOTE`) configured in the Tapo app.
//!
//! # Environment
//!
//! ```bash
//! export TAPO_USERNAME="you@example.com"
//! export TAPO_PASSWORD="your-tapo-password"
//! export IP_ADDRESS="10.0.10.22"
//! export REMOTE=TV   # optional default for --remote
//! ```
//!
//! # Usage
//!
//! From the workspace root (`~/mywork/tapo`):
//!
//! ```bash
//! cargo run --example tapo_h110_ir -p tapo --features debug -- [OPTIONS] [KEY]
//! ```
//!
//! Or from this crate directory (`~/mywork/tapo/tapo`):
//!
//! ```bash
//! cargo run --example tapo_h110_ir --features debug -- [OPTIONS] [KEY]
//! ```
//!
//! Run with `--help` for all options.
//!
//! # Examples
//!
//! ```bash
//! # Send POWER to the TV remote (default)
//! cargo run --example tapo_h110_ir -p tapo --features debug
//!
//! # List available keys on the TV remote
//! cargo run --example tapo_h110_ir -p tapo --features debug -- --list-keys
//!
//! # List all IR remotes on the hub
//! cargo run --example tapo_h110_ir -p tapo --features debug -- --list-remotes
//!
//! # Send volume up
//! cargo run --example tapo_h110_ir -p tapo --features debug -- --remote TV "VOL+"
//! ```
use clap::Parser;
use log::info;
use tapo::ApiClient;

mod common;

#[derive(Parser)]
#[command(
    name = "tapo_h110_ir",
    about = "Send stored IR keys via Tapo H110",
    after_help = "Environment:\n  \
                  TAPO_USERNAME  Tapo account email\n  \
                  TAPO_PASSWORD  Tapo account password\n  \
                  IP_ADDRESS     H110 hub IP address"
)]
struct Cli {
    /// IR key name to send
    #[arg(default_value = "POWER")]
    key: String,

    /// List keys for the selected remote and exit
    #[arg(long)]
    list_keys: bool,

    /// List all IR remotes on the hub and exit
    #[arg(long, conflicts_with = "list_keys")]
    list_remotes: bool,

    /// Remote model or nickname
    #[arg(short, long, default_value = "TV", env = "REMOTE")]
    remote: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup_logger();

    let [tapo_username, tapo_password, ip_address] =
        common::require_env_vars(["TAPO_USERNAME", "TAPO_PASSWORD", "IP_ADDRESS"])?;
    let cli = Cli::parse();

    let hub = ApiClient::new(tapo_username, tapo_password)
        .h100(ip_address)
        .await?;

    let device_info = hub.get_device_info().await?;
    info!("Hub: {} ({})", device_info.nickname, device_info.model);

    let remotes = list_ir_remotes(&hub).await?;

    if cli.list_remotes {
        print_remotes(&remotes);
        return Ok(());
    }

    let remote = select_remote(&remotes, &cli.remote)?;

    if cli.list_keys {
        print_keys(remote);
        return Ok(());
    }

    if !remote.keys.iter().any(|k| k == &cli.key) {
        return Err(format!(
            "Unknown key '{}' on remote '{}'. Available: {}",
            cli.key,
            remote.nickname,
            remote.keys.join(", ")
        )
        .into());
    }

    info!(
        "Sending IR key '{}' on remote '{}' ({})",
        cli.key, remote.nickname, remote.device_id
    );
    hub.send_ir_cmd_by_id(&remote.device_id, &cli.key).await?;
    info!("IR command sent successfully.");

    Ok(())
}

fn print_remotes(remotes: &[IrRemote]) {
    println!("IR remotes on the hub:\n");
    println!("{:<20} {:<20} KEYS", "MODEL", "NICKNAME");
    println!("{}", "-".repeat(56));
    for remote in remotes {
        println!(
            "{:<20} {:<20} {}",
            remote.model,
            remote.nickname,
            remote.keys.len()
        );
    }
}

fn print_keys(remote: &IrRemote) {
    println!(
        "IR keys for remote \"{}\" (model: {}):\n",
        remote.nickname, remote.model
    );
    for key in &remote.keys {
        println!("  {key}");
    }
}

fn select_remote<'a>(remotes: &'a [IrRemote], selector: &str) -> Result<&'a IrRemote, String> {
    remotes
        .iter()
        .find(|r| r.model.eq_ignore_ascii_case(selector))
        .or_else(|| {
            remotes
                .iter()
                .find(|r| r.nickname.eq_ignore_ascii_case(selector))
        })
        .ok_or_else(|| {
            format!("No IR remote matching '{selector}'. Use --list-remotes to see options.")
        })
}

struct IrRemote {
    device_id: String,
    nickname: String,
    model: String,
    keys: Vec<String>,
}

async fn list_ir_remotes(
    hub: &tapo::HubHandler,
) -> Result<Vec<IrRemote>, Box<dyn std::error::Error>> {
    let mut start = 0u64;
    let mut remotes = Vec::new();

    loop {
        let page = hub.get_child_device_list_json(start).await?;
        let children = page["child_device_list"]
            .as_array()
            .ok_or("missing child_device_list in hub response")?;

        for child in children {
            if child["category"].as_str() != Some("ir.remote") {
                continue;
            }

            let device_id = child["device_id"]
                .as_str()
                .ok_or("IR remote missing device_id")?
                .to_string();
            let nickname = child["nickname"].as_str().unwrap_or("").to_string();
            let model = child["model"].as_str().unwrap_or("unknown").to_string();
            let keys = child["key_list"]
                .as_array()
                .ok_or("IR remote missing key_list")?
                .iter()
                .filter_map(|key| key["name"].as_str().map(str::to_string))
                .collect::<Vec<_>>();

            if keys.is_empty() {
                return Err(format!("IR remote '{nickname}' has no keys configured").into());
            }

            remotes.push(IrRemote {
                device_id,
                nickname,
                model,
                keys,
            });
        }

        if children.len() < 10 {
            break;
        }
        start += 10;
    }

    if remotes.is_empty() {
        return Err("No IR remotes found on the H110 hub".into());
    }

    Ok(remotes)
}
