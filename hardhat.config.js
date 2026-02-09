require("hardhat/config");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: {
    version: "0.8.19",
    settings: {
      optimizer: { enabled: true, runs: 200 },
    },
  },
  networks: {
    abstractTestnet: {
      url: process.env.ABSTRACT_RPC_URL || "https://api.testnet.abs.xyz",
      chainId: 11124,
      // accounts: from env PRIVATE_KEY or hardhat's default (for local testing)
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
    },
  },
};
