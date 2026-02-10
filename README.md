# Abstract Event Proofs PoC

PoC: fetching blocks and events from Abstract (testnet) and proving that an event belongs to a block.

## Step 0: Contract (Hardhat)

Everything targets **testnet**; you create and fund the account yourself.

### Build

```bash
yarn install
yarn compile
```

### Deploy to Abstract Testnet

Set your account private key (or use `.env` with `PRIVATE_KEY=0x...`):

```bash
PRIVATE_KEY=0x... yarn deploy
```

Save the printed contract address (`EventEmitter deployed to: 0x...`).

### Call emitValue (emit event)

```bash
CONTRACT_ADDRESS=0x... PRIVATE_KEY=0x... yarn hardhat run scripts/emitValue.js --network abstractTestnet
```

Optional: `VALUE=123` (default is 42). The output includes transaction hash and block number — useful for the next steps (fetching the event, verification).

### Verify contract on block explorer

After deploy, verify the contract on [Abstract Testnet Explorer](https://sepolia.abscan.org) so the source is public.

1. **Get an Etherscan API key**  
   Create one at [Etherscan](https://docs.etherscan.io/getting-started/viewing-api-usage-statistics) (same key is used for Abstract verification).

2. **Set the key in `.env`**  
   Add: `ETHERSCAN_API_KEY=your-api-key`

3. **Run verify** (no constructor args for `EventEmitter`):

```bash
yarn hardhat verify --network abstractTestnet <CONTRACT_ADDRESS>
```

Example:

```bash
yarn hardhat verify --network abstractTestnet 0x1234...abcd
```

If the contract had constructor arguments, you would append them after the address. See [Abstract: Verifying contracts](https://docs.abs.xyz/build-on-abstract/smart-contracts/hardhat/verifying-contracts).

---

## Step 1: Fetch block, event, and proof data (Rust)

Fetches the block header, events for your contract in that block, block receipts, and proof metadata. All output is saved to files in the current directory (or `output_dir` from config).

### Config

Edit `config.toml`:

- `rpc_url` — Abstract RPC (default: testnet).
- `block_number` — Block where the event was emitted (e.g. from `emitValue` output).
- `contract_address` — Deployed EventEmitter address.
- `output_dir` — (optional) Directory for output files; default is current directory.

### Build and run fetch

```bash
cargo build
cargo run -- fetch config.toml
```

Default (no subcommand) also runs fetch with `config.toml`. Custom config: `cargo run -- fetch /path/to/config.toml`.

### Output files

| File | Content |
|------|---------|
| `block_header.json` | Block header from `eth_getBlockByNumber` (includes `receiptsRoot`, `transactionsRoot`). |
| `events.json` | All logs from `eth_getLogs` for that block and contract. |
| `event.json` | First matching event (the one we will prove). |
| `block_receipts.json` | All transaction receipts from `eth_getBlockReceipts`. |
| `event_proof.json` | Proof metadata: `receipt_index_in_block`, `transaction_hash`, `log_index`, `receipts_root` — used later to verify the event belongs to the block. |

---

## Step 2: Verify (event belongs to block)

Reads the saved data from a directory (e.g. `data/`), checks that the event’s transaction is in the block, that the event log is in the corresponding receipt, and that the block’s `receiptsRoot` matches the proof.

```bash
cargo run -- verify data
```

Or: `cargo run -- verify /path/to/data/dir`.  
If all checks pass, the program prints: `Verify OK: event belongs to block (...)`.
