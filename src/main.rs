use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Config {
    rpc_url: String,
    block_number: u64,
    contract_address: String,
    #[serde(default)]
    output_dir: String,
}

fn load_config(path: &str) -> Result<Config> {
    let s = fs::read_to_string(path).with_context(|| format!("read config: {path}"))?;
    toml::from_str(&s).with_context(|| format!("parse config: {path}"))
}

async fn rpc_call(
    client: &reqwest::Client,
    url: &str,
    method: &str,
    params: Vec<Value>,
) -> Result<Value> {
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });
    let resp = client
        .post(url)
        .json(&body)
        .send()
        .await
        .context("RPC request failed")?;
    let json: Value = resp.json().await.context("RPC response not JSON")?;
    if let Some(err) = json.get("error") {
        anyhow::bail!("RPC error: {}", err);
    }
    json.get("result")
        .cloned()
        .context("RPC response missing result")
}

fn save_json(path: &Path, value: &Value) -> Result<()> {
    let s = serde_json::to_string_pretty(value).context("serialize")?;
    fs::write(path, s).with_context(|| format!("write {}", path.display()))
}

fn ensure_output_dir(out_dir: &Path) -> Result<()> {
    if out_dir != Path::new(".") {
        fs::create_dir_all(out_dir).context("create output dir")?;
    }
    Ok(())
}

async fn fetch_block_header(
    client: &reqwest::Client,
    rpc_url: &str,
    block_hex: &str,
) -> Result<Value> {
    rpc_call(
        client,
        rpc_url,
        "eth_getBlockByNumber",
        vec![serde_json::json!(block_hex), serde_json::json!(false)],
    )
    .await
    .context("eth_getBlockByNumber")
}

async fn fetch_events(
    client: &reqwest::Client,
    rpc_url: &str,
    block_hex: &str,
    contract_address: &str,
) -> Result<Value> {
    let filter = serde_json::json!({
        "fromBlock": block_hex,
        "toBlock": block_hex,
        "address": contract_address
    });
    rpc_call(client, rpc_url, "eth_getLogs", vec![filter])
        .await
        .context("eth_getLogs")
}

async fn fetch_block_receipts(
    client: &reqwest::Client,
    rpc_url: &str,
    block_hex: &str,
) -> Result<Value> {
    rpc_call(client, rpc_url, "eth_getBlockReceipts", vec![serde_json::json!(block_hex)])
        .await
        .context("eth_getBlockReceipts")
}

fn write_block_header(out_dir: &Path, block: &Value) -> Result<()> {
    let path = out_dir.join("block_header.json");
    save_json(&path, block)?;
    eprintln!("Saved block header to {}", path.display());
    Ok(())
}

/// Returns the first event (for event.json) if any.
fn write_events(out_dir: &Path, events: &Value, block_number: u64, contract_address: &str) -> Result<Option<Value>> {
    let events_array = events.as_array().context("eth_getLogs result not array")?;
    let events_path = out_dir.join("events.json");
    save_json(&events_path, events)?;
    eprintln!("Saved {} event(s) to {}", events_array.len(), events_path.display());

    let first = events_array.first().cloned();
    if let Some(ref ev) = first {
        let event_path = out_dir.join("event.json");
        save_json(&event_path, ev)?;
        eprintln!("Saved selected event to {}", event_path.display());
    } else {
        eprintln!(
            "No events in block {} for contract {}",
            block_number, contract_address
        );
    }
    Ok(first)
}

fn write_block_receipts(out_dir: &Path, receipts: &Value) -> Result<()> {
    let path = out_dir.join("block_receipts.json");
    save_json(&path, receipts)?;
    eprintln!("Saved block receipts to {}", path.display());
    Ok(())
}

fn write_event_proof(
    out_dir: &Path,
    block_number: u64,
    block: &Value,
    event: &Value,
    receipts: &Value,
) -> Result<()> {
    let tx_hash = event
        .get("transactionHash")
        .and_then(Value::as_str)
        .context("event missing transactionHash")?;
    let log_index = event
        .get("logIndex")
        .and_then(Value::as_str)
        .context("event missing logIndex")?;
    let receipts_arr = receipts.as_array().context("block_receipts not array")?;
    let receipt_index = receipts_arr
        .iter()
        .enumerate()
        .find_map(|(i, r)| {
            if r.get("transactionHash").and_then(Value::as_str) == Some(tx_hash) {
                Some(i as u64)
            } else {
                None
            }
        });
    let receipts_root = block
        .get("receiptsRoot")
        .and_then(Value::as_str)
        .context("block missing receiptsRoot")?;
    let proof = serde_json::json!({
        "block_number": block_number,
        "block_hash": block.get("hash").and_then(Value::as_str),
        "receipts_root": receipts_root,
        "transaction_hash": tx_hash,
        "log_index": log_index,
        "receipt_index_in_block": receipt_index,
        "note": "Verify by recomputing receiptsRoot from block_receipts.json and comparing to receipts_root"
    });
    let path = out_dir.join("event_proof.json");
    save_json(&path, &proof)?;
    eprintln!("Saved event proof metadata to {}", path.display());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config.toml".to_string());
    let config = load_config(&config_path)?;

    let out_dir = if config.output_dir.is_empty() {
        Path::new(".")
    } else {
        Path::new(&config.output_dir)
    };
    ensure_output_dir(out_dir)?;

    let client = reqwest::Client::new();
    let block_hex = format!("0x{:x}", config.block_number);

    let block = fetch_block_header(&client, &config.rpc_url, &block_hex).await?;
    write_block_header(out_dir, &block)?;

    let events = fetch_events(
        &client,
        &config.rpc_url,
        &block_hex,
        &config.contract_address,
    )
    .await?;
    let event = write_events(
        out_dir,
        &events,
        config.block_number,
        &config.contract_address,
    )?;

    let receipts = fetch_block_receipts(&client, &config.rpc_url, &block_hex).await?;
    write_block_receipts(out_dir, &receipts)?;

    if let Some(ref ev) = event {
        write_event_proof(out_dir, config.block_number, &block, ev, &receipts)?;
    }

    Ok(())
}
