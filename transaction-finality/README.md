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
export TX_HASH="0xaf89269a918df9c9be87894b276655eec89de4f8aa90b5c73e1985a2819bccd6"
export RPC="https://api.testnet.abs.xyz"

curl -s -X POST "$RPC" -H "content-type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"zks_getTransactionDetails\",\"params\":[\"$TX_HASH\"]}" | jq '{status, l1BatchNumber, l1BatchTxIndex}'
```

---

## 2. Transaction status values

From **zks_getTransactionDetails**, the `status` field can be:

| Value      | Description |
|-----------|-------------|
| `pending` | Transaction is in the mempool, not yet included in a block. |
| `included`| Included in an L2 block and executed on L2 (soft confirmation). |
| `verified`| The batch containing this transaction has been proven (ZK proof verified on L1). |
| `failed`  | Transaction was executed but reverted. |

Also useful:
- **l1BatchNumber** — L1 batch number; if `null`, the batch is not formed yet.
- **l1BatchTxIndex** — Index of the transaction within the batch (e.g. for proofs).

---

## 3. Observations: which statuses appear and how often

*(Use this section to record check results: time after sending tx → status, when l1BatchNumber appeared, when it became verified, etc.)*

| Date/time (UTC) | TX_HASH (short) | N min after tx | status   | l1BatchNumber |
|-----------------|-----------------|----------------|----------|---------------|
| *(example)*     | 0xaf8926...bccd6 | 1 min        | included | null          |
| *(example)*     | 0xaf8926...bccd6 | 60 min       | verified | 12345         |

*(Extend the table with more runs as you experiment.)*

---

## Links

- [Abstract: Transaction Lifecycle](https://docs.abs.xyz/how-abstract-works/architecture/transaction-lifecycle)
- [zkSync: Finality](https://docs.zksync.io/zksync-protocol/rollup/finality)
