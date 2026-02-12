//! Integration test: Abstract Sepolia tx 0xa48034c78390584fd4ee6aa7aa30d5ab5cabcfc527a8c78d9cf4b02734ce17d5, log index 2.
//! Expected: ValueEmitted(address indexed from, uint256 value) from contract 0x334F261477fB0F091D0D6bD701989c05B5c49F2B.

use events_extractor::fetch_and_extract;
use primitive_types::U256;
use reqwest::Client;
use sha3::{Digest, Keccak256};

const ABSTRACT_SEPOLIA_RPC: &str = "https://api.testnet.abs.xyz";
const TX_HASH: &str = "0xa48034c78390584fd4ee6aa7aa30d5ab5cabcfc527a8c78d9cf4b02734ce17d5";
const LOG_INDEX: u64 = 2;

/// Canonical event signature (no parameter names, no spaces) — topic0 = keccak256(this).
const VALUE_EMITTED_SIGNATURE: &str = "ValueEmitted(address,uint256)";

const EXPECTED_EMITTER: &str = "0x334f261477fb0f091d0d6bd701989c05b5c49f2b";
const EXPECTED_DATA: &str = "0x000000000000000000000000000000000000000000000000000000000000002a";

/// Compute topic0 for an event: keccak256(canonical signature).
fn event_topic0(signature: &str) -> String {
    let hash = Keccak256::digest(signature.as_bytes());
    format!("0x{}", hash.iter().map(|b| format!("{:02x}", b)).collect::<String>())
}

fn bytes_to_hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

/// Decode ValueEmitted(address indexed from, uint256 value) from raw log.
fn decode_value_emitted(data: &[u8], topics: &[[u8; 32]]) -> (String, U256) {
    // from: topics[1] is 32-byte word, address is right-padded (last 20 bytes)
    let from = if let Some(t1) = topics.get(1) {
        format!("0x{}", bytes_to_hex(&t1[12..32]))
    } else {
        "0x?".to_string()
    };

    // value: data is single uint256, 32 bytes big-endian
    let value = {
        let mut bytes = [0u8; 32];
        let n = data.len().min(32);
        bytes[32 - n..].copy_from_slice(&data[data.len().saturating_sub(n)..]);
        U256::from_big_endian(&bytes)
    };

    (from, value)
}

#[tokio::test]
async fn sepolia_log_index_2_value_emitted() {
    let client = Client::new();
    let event = fetch_and_extract(&client, ABSTRACT_SEPOLIA_RPC, TX_HASH, LOG_INDEX)
        .await
        .expect("fetch and extract");

    // Print full raw info
    println!("--- Abstract Sepolia: tx {} log_index {} ---", TX_HASH, LOG_INDEX);
    println!("data:            0x{}", bytes_to_hex(&event.data));
    println!("emitter_address: 0x{}", bytes_to_hex(&event.emitter_address));
    println!("topics:");
    for (i, t) in event.topics.iter().enumerate() {
        println!("  [{}] 0x{}", i, bytes_to_hex(t));
    }
    println!("--- JSON (raw) ---");
    println!("{}", serde_json::to_string_pretty(&event).unwrap());
    println!("---");

    // Topic0: event signature hash
    let expected_topic0 = event_topic0(VALUE_EMITTED_SIGNATURE);
    println!("--- Topic0 (event signature hash) ---");
    println!("  signature: {}", VALUE_EMITTED_SIGNATURE);
    println!("  topic0 = keccak256(signature): {}", expected_topic0);
    println!("  received topic[0]:             0x{}", bytes_to_hex(&event.topics[0]));
    println!("---");

    // Decode ValueEmitted(address indexed from, uint256 value)
    let (from, value) = decode_value_emitted(&event.data, &event.topics);
    println!("--- Decoded ValueEmitted(address indexed from, uint256 value) ---");
    println!("  from (indexed): {}", from);
    println!("  value (uint256): {}", value);
    println!("---");

    let emitter_hex = format!("0x{}", bytes_to_hex(&event.emitter_address)).to_lowercase();
    assert_eq!(
        emitter_hex,
        EXPECTED_EMITTER.to_lowercase(),
        "emitter_address mismatch"
    );
    assert!(
        !event.topics.is_empty(),
        "ValueEmitted has at least topic0"
    );
    assert_eq!(
        format!("0x{}", bytes_to_hex(&event.topics[0])).to_lowercase(),
        expected_topic0.to_lowercase(),
        "topic0 must equal keccak256(\"ValueEmitted(address,uint256)\")"
    );
    assert_eq!(
        format!("0x{}", bytes_to_hex(&event.data)).to_lowercase(),
        EXPECTED_DATA.to_lowercase(),
        "data (value 42)"
    );

    assert_eq!(from.to_lowercase(), "0x8bc8a30928fa0757c2d7084077dc5536c7171489");
    assert_eq!(value, U256::from(42));
}
