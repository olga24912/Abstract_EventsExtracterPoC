//! Extract a single log by transaction hash and log index.
//! Returns: Data, EmitterAddress, Topics.

use anyhow::{Context, Result};
use clap::Parser;
use events_extractor::fetch_and_extract;
use reqwest::Client;

#[derive(Parser)]
#[command(name = "events-extractor")]
#[command(about = "Extract event log (data, emitter address, topics) by tx hash and log index")]
struct Cli {
    /// JSON-RPC URL (e.g. Abstract or Ethereum node)
    #[arg(long, env = "RPC_URL", default_value = "https://api.mainnet.abstract.network")]
    rpc_url: String,

    /// Transaction hash (0x-prefixed)
    #[arg(long)]
    tx_hash: String,

    /// Log index in the transaction receipt (0-based)
    #[arg(long)]
    log_index: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let tx_hash = cli.tx_hash.trim();
    if tx_hash.len() < 2 || !tx_hash.starts_with("0x") {
        anyhow::bail!("tx_hash must be 0x-prefixed hex");
    }

    let client = Client::new();
    let event = fetch_and_extract(&client, &cli.rpc_url, tx_hash, cli.log_index).await?;
    let out = serde_json::to_string_pretty(&event).context("serialize output")?;
    println!("{}", out);
    Ok(())
}
