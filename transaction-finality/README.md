# Transaction finality (Abstract)

This folder is for understanding how transaction finality works on Abstract: what statuses exist, how to check them, and what to observe in practice.

---

## 1. Send a transaction and check its status

**Send the transaction** (from repo root; `CONTRACT_ADDRESS` and `PRIVATE_KEY` from `.env`):
```bash
yarn hardhat run scripts/emitValue.js --network abstractTestnet
```
Save the printed transaction hash and block number.

**Check status** (set `TX_HASH` to that hash):
```bash
export TX_HASH="0xa48034c78390584fd4ee6aa7aa30d5ab5cabcfc527a8c78d9cf4b02734ce17d5"
export RPC="https://api.testnet.abs.xyz"

curl -s -X POST "$RPC" -H "content-type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"zks_getTransactionDetails\",\"params\":[\"$TX_HASH\"]}" | jq
```

**Example output** (tx just included on L2, batch not yet on L1):
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "isL1Originated": false,
    "status": "included",
    "fee": "0x4369df7ae40",
    "gasPerPubdata": "0xc350",
    "initiatorAddress": "0x8bc8a30928fa0757c2d7084077dc5536c7171489",
    "receivedAt": "2026-02-11T20:29:47.952025Z",
    "ethCommitTxHash": null,
    "ethProveTxHash": null,
    "ethExecuteTxHash": null,
    "ethPrecommitTxHash": null
  }
}
```

L1 pipeline (when set, that step is done on Ethereum): **ethCommitTxHash** → **ethProveTxHash** → **ethExecuteTxHash**. Until then they are `null`.

---

## 2. Transaction status values

Usual progression: **pending** → **included** → **verified**.

| Value      | Description |
|-----------|-------------|
| `pending` | In mempool, not yet in a block. |
| `included`| In an L2 block (soft confirmation). |
| `verified`| Batch proven on L1 (ZK proof verified). |

---

## 3. Observations: which statuses appear and how often (Sepolia)


| Date/time (UTC) | N min after tx | status    | When which hash appears        |
|-----------------|----------------|-----------|--------------------------------|
| *(example)*     | 0              | pending   | —                              |
| *(example)*     | 0              | included  | —                              |
| *(example)*     | 30             | included  | + ethCommitTxHash              |
| *(example)*     | 60             | verified  | + ethProveTxHash               |
| *(example)*     | 120            | verified  | + ethExecuteTxHash             |

All statuses: **pending** → **included** → **verified**.

---

## 4. Block tags and finality (Abstract: Ethereum-style API)

Abstract exposes **eth_getBlockByNumber**. The first parameter can be a **block number** (hex) or a **tag**:

| First param   | Meaning |
|---------------|--------|
| `"latest"`    | Latest L2 block (can change with new blocks). |
| `"safe"`      | Batch **committed** on L1. |
| `"finalized"` | Batch **executed** on L1. |

**Examples (use Abstract RPC and optional `TX_HASH` / block from receipt):**
```bash
export RPC="https://api.testnet.abs.xyz"

curl -s -X POST "$RPC" -H "content-type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_getBlockByNumber","params":["finalized", false]}' | jq
```

Second parameter `false` = return block without full tx objects; use `true` for full transactions.

**Example output** (excerpt: `number`, `l1BatchNumber`, `hash`):
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "hash": "0xe3f41867d8e43eee5a9c6aa79032bfbfe59d289b1b4507894fed445769f963f1",
    "number": "0xfc9c61",
    "l1BatchNumber": "0x4f4e",
    ...
  }
}
```

**Get L1 batch status** (by `l1BatchNumber` from a block). Example: `l1BatchNumber = 0x4f4e` = **20302** in decimal.
```bash
export RPC="https://api.testnet.abs.xyz"

curl -s -X POST "$RPC" -H "content-type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"zks_getL1BatchDetails","params":[20302]}' | jq
```

**Example response** (excerpt):
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "number": 20302,
    "status": "verified",
    "commitTxHash": "0x8b570651220f4d71552e2f6702516ef8a2dfe63dff19953d7dc5021974f9edc8",
    "committedAt": "2026-02-11T20:17:46.316223Z",
    "commitTxFinality": "finalized",
    "proveTxHash": "0x09f2c02c0610819b7532f019b70dc4eee0e9de58cdf288e9f7d1b92e41b32a3e",
    "provenAt": "2026-02-11T20:30:49.779802Z",
    "proveTxFinality": "finalized",
    "executeTxHash": "0x7dd7fa0cccfeb691d79dc3df477b3bc37f3d70a24911f01fc379177c49b59d17",
    "executedAt": "2026-02-11T20:30:49.910702Z",
    "executeTxFinality": "finalized",
    "precommitTxHash": null,
    "..."
  }
}
```
Response shows whether the batch is committed/proven/executed on L1 and the L1 tx hashes and timestamps.

---

## Check finality of a transaction (script)

**`check-finality.sh`** — same idea as on Ethereum: get tx block number, get current **finalized** block number (executed on L1), compare. If tx block ≤ finalized block → transaction is finalized (executed on L1).

Uses example `TX_HASH` by default; override with `TX_HASH=0x... ./check-finality.sh`.

**Run:**
```bash
cd transaction-finality
chmod +x check-finality.sh
./check-finality.sh
```

**`safe-finalized.sh`** — Fetches the current **safe** and **finalized** blocks via `eth_getBlockByNumber`, extracts **l1BatchNumber** from each, then calls **zks_getL1BatchDetails** for both. The batch for the **safe** block is only **committed** on L1; the batch for the **finalized** block is **executed** on L1. Confirms: safe ≈ committed, finalized ≈ executed.

**Run** (default RPC: mainnet):
```bash
cd transaction-finality
chmod +x safe-finalized.sh
./safe-finalized.sh
```

---

## Summary

- **Abstract** can be queried with the **standard Ethereum JSON-RPC API**; block tags `latest`, `safe`, and `finalized` are supported.
- On Abstract: **safe** = the transaction (batch) is **committed** on L1. **Finalized** = the transaction is **executed** on L1.
- For “tx is settled on L1” we should use **finalized** (compare the tx block to the current `finalized` block), not `safe`. 