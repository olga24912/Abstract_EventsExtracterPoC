const hre = require("hardhat");

async function main() {
  const [deployer] = await hre.ethers.getSigners();
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
