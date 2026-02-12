//! Library: fetch receipt and extract event log by tx hash and log index.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Serialize, Serializer};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ExtractedEvent {
    pub data: Vec<u8>,
    pub emitter_address: [u8; 20],
    pub topics: Vec<[u8; 32]>,
}

fn hex_to_bytes(s: &str) -> Result<Vec<u8>> {
    let s = s.trim_start_matches("0x");
    if s.is_empty() {
        return Ok(Vec::new());
    }
    let mut out = Vec::with_capacity((s.len() + 1) / 2);
    for i in (0..s.len()).step_by(2) {
        let end = (i + 2).min(s.len());
        let byte = u8::from_str_radix(s.get(i..end).unwrap_or("0"), 16)
            .context("invalid hex in log")?;
        out.push(byte);
    }
    Ok(out)
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

impl Serialize for ExtractedEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut st = serializer.serialize_struct("ExtractedEvent", 3)?;
        st.serialize_field("data", &format!("0x{}", bytes_to_hex(&self.data)))?;
        st.serialize_field("emitter_address", &format!("0x{}", bytes_to_hex(&self.emitter_address)))?;
        st.serialize_field(
            "topics",
            &self
                .topics
                .iter()
                .map(|t| format!("0x{}", bytes_to_hex(t)))
                .collect::<Vec<_>>(),
        )?;
        st.end()
    }
}

fn parse_address(hex: &str) -> Result<[u8; 20]> {
    let b = hex_to_bytes(hex)?;
    let mut out = [0u8; 20];
    let n = b.len().min(20);
    out[20 - n..].copy_from_slice(&b[b.len().saturating_sub(n)..]);
    Ok(out)
}

fn parse_topic(hex: &str) -> Result<[u8; 32]> {
    let b = hex_to_bytes(hex)?;
    let mut out = [0u8; 32];
    let n = b.len().min(32);
    out[32 - n..].copy_from_slice(&b[b.len().saturating_sub(n)..]);
    Ok(out)
}

pub fn extract_event(receipt: &Value, log_index: u64) -> Result<ExtractedEvent> {
    let logs = receipt
        .get("logs")
        .and_then(Value::as_array)
        .context("receipt missing logs")?;
    let log = logs
        .get(log_index as usize)
        .context("log_index out of range")?;

    let address_str = log
        .get("address")
        .and_then(Value::as_str)
        .context("log missing address")?;
    let data_str = log
        .get("data")
        .and_then(Value::as_str)
        .unwrap_or("0x");
    let empty: Vec<Value> = vec![];
    let topics_arr = log
        .get("topics")
        .and_then(Value::as_array)
        .unwrap_or(&empty);

    let emitter_address = parse_address(address_str)?;
    let data = hex_to_bytes(data_str)?;
    let mut topics = Vec::with_capacity(topics_arr.len());
    for t in topics_arr {
        let s = t.as_str().context("topic not a string")?;
        topics.push(parse_topic(s)?);
    }

    Ok(ExtractedEvent {
        data,
        emitter_address,
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
