import "@openzeppelin/hardhat-upgrades";
import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "@nomicfoundation/hardhat-ethers";

import * as dotenv from "dotenv";
dotenv.config({ path: './.env.dev' });

const config: HardhatUserConfig = {
  solidity: {
    compilers: [
      {
        version: "0.8.19",
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
        },
      }
    ]
  },
  networks: {
    // for testnet
    "base-goerli": {
      url: "https://goerli.base.org",
      accounts: [process.env.OWNER_PRIKEY as string],
      gasPrice: 1000000000,
    },
    "goerli": {
      url: process.env.ALCHEMY_URL,
      accounts: [process.env.OWNER_PRIKEY as string],
    },
  },
  etherscan: {
    url: "https://api-goerli.basescan.org/api",
    apiKey: process.env.ETHERSCAN_APIKEY as string
  }
};

export default config;
