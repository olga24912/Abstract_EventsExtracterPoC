#!/usr/bin/env bash
# Check finality of a transaction on Abstract (same idea as on Ethereum):
# get tx block number, get safe block number (executed on L1), compare.
# Uses example TX_HASH; override with: TX_HASH=0x... ./check-finality.sh

RPC="${RPC:-https://api.testnet.abs.xyz}"
TX_HASH="${TX_HASH:-0xa48034c78390584fd4ee6aa7aa30d5ab5cabcfc527a8c78d9cf4b02734ce17d5}"

echo "RPC: $RPC"
echo "TX_HASH: $TX_HASH"
echo ""

# Tx block number (from receipt)
TX_BLOCK=$(curl -s -X POST "$RPC" -H "content-type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"eth_getTransactionByHash\",\"params\":[\"$TX_HASH\"]}" | jq -r '.result.blockNumber // empty')

if [[ -z "$TX_BLOCK" ]]; then
  echo "Tx not mined or unknown."
  exit 1
fi

# Current safe block number (executed on L1)
SAFE=$(curl -s -X POST "$RPC" -H "content-type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_getBlockByNumber","params":["safe", false]}' | jq -r '.result.number // empty')

echo "Tx block:    $TX_BLOCK"
echo "Safe block:  $SAFE"

# Compare (hex works in bash arithmetic)
if (( TX_BLOCK <= SAFE )); then
  echo ""
  echo ">>> Transaction is safe (executed on L1)."
else
  echo ""
  echo ">>> Transaction not yet safe."
fi
