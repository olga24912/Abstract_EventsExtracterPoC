/**
 * Call emitValueWithL1Log(value) on deployed EventEmitter.
 * Usage: CONTRACT_ADDRESS=0x... npx hardhat run scripts/emitValue.js --network abstractTestnet
 *        Or set value: VALUE=42 CONTRACT_ADDRESS=0x... npx hardhat run scripts/emitValue.js --network abstractTestnet
 */
const hre = require("hardhat");

async function main() {
  const contractAddress = process.env.CONTRACT_ADDRESS;
  if (!contractAddress) {
    console.error("Set CONTRACT_ADDRESS env (deployed EventEmitter address)");
    process.exit(1);
  }
  const value = process.env.VALUE ? parseInt(process.env.VALUE, 10) : 42;

  const emitter = await hre.ethers.getContractAt("EventEmitter", contractAddress);
  const tx = await emitter.emitValueWithL1Log(value);
  console.log("Transaction hash:", tx.hash);
  const receipt = await tx.wait();
  console.log("Block number:", receipt.blockNumber);
  console.log("Called emitValueWithL1Log(" + value + ")");
  console.log("This emits ValueEmitted and sends L2->L1 message via L1Messenger");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
