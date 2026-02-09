/**
 * Call emitValue(value) on deployed EventEmitter.
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
  const tx = await emitter.emitValue(value);
  console.log("Transaction hash:", tx.hash);
  const receipt = await tx.wait();
  console.log("Block number:", receipt.blockNumber);
  console.log("Emitted ValueEmitted(msg.sender, " + value + ")");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
