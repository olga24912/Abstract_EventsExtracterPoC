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
