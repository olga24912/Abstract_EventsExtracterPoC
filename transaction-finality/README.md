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

## 3. Observations: which statuses appear and how often


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
| `"safe"`      | Safe block (node’s view of a block that is unlikely to reorg). |
| `"finalized"` | Finalized block (L2 finality; on some chains tied to L1). |
| `"pending"`   | Pending / not yet sealed. |
| `"0x1234"`    | Block number in hex (e.g. the block that contains your tx). |

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

**Examples (use Abstract RPC and optional `TX_HASH` / block from receipt):**
```bash
export RPC="https://api.testnet.abs.xyz"

curl -s -X POST "$RPC" -H "content-type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"eth_getBlockByNumber","params":["finalized", false]}' | jq
```

Second parameter `false` = return block without full tx objects; use `true` for full transactions. 