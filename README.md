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
