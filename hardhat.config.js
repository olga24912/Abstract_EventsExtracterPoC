require("hardhat/config");
require("@nomicfoundation/hardhat-ethers");
require("@nomicfoundation/hardhat-verify");
const path = require("path");
require("dotenv").config({ path: path.resolve(__dirname, ".env") });

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
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
    },
  },
  etherscan: {
    apiKey: {
      abstractTestnet: process.env.ETHERSCAN_API_KEY || "",
      abstractMainnet: process.env.ETHERSCAN_API_KEY || "",
    },
    customChains: [
      {
        network: "abstractTestnet",
        chainId: 11124,
        urls: {
          apiURL: "https://api.etherscan.io/v2/api?chainid=11124",
          browserURL: "https://sepolia.abscan.org",
        },
      },
      {
        network: "abstractMainnet",
        chainId: 2741,
        urls: {
          apiURL: "https://api.etherscan.io/v2/api?chainid=2741",
          browserURL: "https://abscan.org",
        },
      },
    ],
  },
};
