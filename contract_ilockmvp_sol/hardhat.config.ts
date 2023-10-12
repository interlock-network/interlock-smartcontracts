import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "@nomicfoundation/hardhat-ethers";
import "@openzeppelin/hardhat-upgrades";



const config: HardhatUserConfig = {
  solidity: "0.8.19",
  // for testnet
  "base-goerli": {
    url: "https://goerli.base.org",
    accounts: [process.env.PRIVATE_KEY as string]
    gasPrice: 1000000000,
  },
};

export default config;
