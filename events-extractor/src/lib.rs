//! Library: fetch receipt and extract event log by tx hash and log index.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct ExtractedEvent {
    #[serde(rename = "data")]
    pub data: String,
    #[serde(rename = "emitter_address")]
    pub emitter_address: String,
    #[serde(rename = "topics")]
    pub topics: Vec<String>,
}

pub async fn rpc_call(client: &Client, url: &str, method: &str, params: Vec<Value>) -> Result<Value> {
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

pub fn extract_event(receipt: &Value, log_index: u64) -> Result<ExtractedEvent> {
    let logs = receipt
        .get("logs")
        .and_then(Value::as_array)
        .context("receipt missing logs")?;
    let log = logs
        .get(log_index as usize)
        .context("log_index out of range")?;

    let address = log
        .get("address")
        .and_then(Value::as_str)
        .context("log missing address")?;
    let data = log
        .get("data")
        .and_then(Value::as_str)
        .unwrap_or("0x")
        .to_string();
    let topics: Vec<String> = log
        .get("topics")
        .and_then(Value::as_array)
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|t| t.as_str().map(String::from))
        .collect();

    Ok(ExtractedEvent {
        data,
        emitter_address: address.to_string(),
        topics,
    })
}

/// Fetch transaction receipt from RPC and extract the log at the given index.
pub async fn fetch_and_extract(
    client: &Client,
    rpc_url: &str,
    tx_hash: &str,
    log_index: u64,
) -> Result<ExtractedEvent> {
    let receipt = rpc_call(
        client,
        rpc_url,
        "eth_getTransactionReceipt",
        vec![serde_json::json!(tx_hash)],
    )
    .await
    .context("eth_getTransactionReceipt")?;

    if receipt.is_null() {
        anyhow::bail!("No receipt for transaction {}", tx_hash);
    }

    extract_event(&receipt, log_index)
}
