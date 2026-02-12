# events-extractor

Get one event log by **tx hash** and **log index**: returns `data`, `emitter_address`, `topics`.  
Uses `eth_getTransactionReceipt`; log = `result.logs[log_index]`.

## Raw RPC (curl)

```bash
RPC="https://api.testnet.abs.xyz"
TX_HASH="0xa48034c78390584fd4ee6aa7aa30d5ab5cabcfc527a8c78d9cf4b02734ce17d5"
curl -s -X POST "$RPC" -H "content-type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"eth_getTransactionReceipt\",\"params\":[\"$TX_HASH\"]}" | jq
```

## Example response (trimmed)

*In the example: `…` = omitted (shortened for readability; real response has more fields and full hex).*

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "transactionHash": "0xa48034c78390584fd4ee6aa7aa30d5ab5cabcfc527a8c78d9cf4b02734ce17d5",
    "blockNumber": "0xfc9d3b",
    "from": "0x8bc8a30928fa0757c2d7084077dc5536c7171489",
    "to": "0x334f261477fb0f091d0d6bd701989c05b5c49f2b",
    "logs": [
      { "address": "0x...800a", "topics": ["0xddf252ad...", "..."], "data": "0x...", "logIndex": "0x0", ... },
      { "address": "0x...800a", "topics": ["0xddf252ad...", "..."], "data": "0x...", "logIndex": "0x1", ... },
      { "address": "0x334f261477fb0f091d0d6bd701989c05b5c49f2b", "topics": ["0x65db2c1a...", "0x000...8bc8a309..."], "data": "0x00...002a", "logIndex": "0x2", ... }
    ],
    "status": "0x1",
    ...
  }
}
```

Log at index 2 = `ValueEmitted(address,uint256)` from the contract: `data` = value 42.

## Rust CLI

```bash
cargo build --release
./target/release/events-extractor --rpc-url "$RPC" --tx-hash "$TX_HASH" --log-index 2
```

Output: `{"data":"0x...","emitter_address":"0x...","topics":["0x...",...]}`
