const hre = require("hardhat");

async function main() {
  const signers = await hre.ethers.getSigners();
  if (!signers.length) {
    throw new Error("No signer: set PRIVATE_KEY in .env (and run from project root so dotenv loads it)");
  }
  const [deployer] = signers;
  console.log("Deploying with account:", deployer.address);

  const EventEmitter = await hre.ethers.getContractFactory("EventEmitter");
  const contract = await EventEmitter.deploy();
  await contract.waitForDeployment();
  const address = await contract.getAddress();
  console.log("EventEmitter deployed to:", address);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
