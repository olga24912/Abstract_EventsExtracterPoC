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
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"zks_getTransactionDetails\",\"params\":[\"$TX_HASH\"]}"
```

**Example output** (tx just included on L2, batch not yet on L1):
```json
{"jsonrpc":"2.0","id":1,"result":{"isL1Originated":false,"status":"included","fee":"0x4369df7ae40","gasPerPubdata":"0xc350","initiatorAddress":"0x8bc8a30928fa0757c2d7084077dc5536c7171489","receivedAt":"2026-02-11T20:29:47.952025Z","ethCommitTxHash":null,"ethProveTxHash":null,"ethExecuteTxHash":null,"ethPrecommitTxHash":null}}
```

L1 pipeline (when set, that step is done on Ethereum): **ethPrecommitTxHash** → **ethCommitTxHash** → **ethProveTxHash** → **ethExecuteTxHash**. Until then they are `null`.

---

## 2. Transaction status values

Usual progression: **pending** → **included** → **verified**. **failed** means the tx reverted (after execution).

| Value      | Description |
|-----------|-------------|
| `pending` | In mempool, not yet in a block. |
| `included`| In an L2 block, executed (soft confirmation). |
| `verified`| Batch proven on L1 (ZK proof verified). |
| `failed`  | Executed but reverted. |

Also useful: **l1BatchNumber** (batch id; `null` until batch exists), **l1BatchTxIndex** (tx index in batch).

---

## 3. Observations: which statuses appear and how often

*(Record check results: time after tx → status, when l1BatchNumber appeared, when verified.)*

| Date/time (UTC) | TX_HASH (short) | N min after tx | status   | l1BatchNumber |
|-----------------|-----------------|----------------|----------|---------------|
| *(example)*     | 0xaf8926...bccd6 | 1 min        | included | null          |
| *(example)*     | 0xaf8926...bccd6 | 60 min       | verified | 12345         |

---

## Links

- [Abstract: Transaction Lifecycle](https://docs.abs.xyz/how-abstract-works/architecture/transaction-lifecycle)
- [zkSync: Finality](https://docs.zksync.io/zksync-protocol/rollup/finality)
