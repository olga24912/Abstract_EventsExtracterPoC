#!/usr/bin/env bash
# Mainnet experiment: compare l1BatchNumber and batch status for "safe" vs "finalized" blocks.
# On Abstract: safe = batch committed on L1; finalized = batch executed on L1.

RPC="${RPC:-https://api.mainnet.abs.xyz}"

echo "RPC: $RPC"
echo ""

# --- Safe block: number + l1BatchNumber
echo "=== eth_getBlockByNumber(\"safe\") ==="
SAFE_BLOCK=$(curl -s -X POST "$RPC" -H "content-type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_getBlockByNumber","params":["safe", false]}')
echo "$SAFE_BLOCK" | jq '{number: .result.number, l1BatchNumber: .result.l1BatchNumber, hash: .result.hash}'

SAFE_L1_BATCH=$(echo "$SAFE_BLOCK" | jq -r '.result.l1BatchNumber // empty')
SAFE_L1_DEC=$((SAFE_L1_BATCH))

echo ""
echo "=== eth_getBlockByNumber(\"finalized\") ==="
FINAL_BLOCK=$(curl -s -X POST "$RPC" -H "content-type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_getBlockByNumber","params":["finalized", false]}')
echo "$FINAL_BLOCK" | jq '{number: .result.number, l1BatchNumber: .result.l1BatchNumber, hash: .result.hash}'

FINAL_L1_BATCH=$(echo "$FINAL_BLOCK" | jq -r '.result.l1BatchNumber // empty')
FINAL_L1_DEC=$((FINAL_L1_BATCH))

echo ""
echo "--- L1 batch details (status) ---"
echo ""

if [[ -n "$SAFE_L1_BATCH" && "$SAFE_L1_BATCH" != "null" ]]; then
  echo ">>> Batch for SAFE block (l1BatchNumber = $SAFE_L1_DEC):"
  curl -s -X POST "$RPC" -H "content-type: application/json" \
    -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"zks_getL1BatchDetails\",\"params\":[$SAFE_L1_DEC]}" | jq '.result | {number, status, commitTxHash, commitTxFinality, proveTxHash, proveTxFinality, executeTxHash, executeTxFinality}'
  echo ""
fi

if [[ -n "$FINAL_L1_BATCH" && "$FINAL_L1_BATCH" != "null" ]]; then
  echo ">>> Batch for FINALIZED block (l1BatchNumber = $FINAL_L1_DEC):"
  curl -s -X POST "$RPC" -H "content-type: application/json" \
    -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"zks_getL1BatchDetails\",\"params\":[$FINAL_L1_DEC]}" | jq '.result | {number, status, commitTxHash, commitTxFinality, proveTxHash, proveTxFinality, executeTxHash, executeTxFinality}'
fi
