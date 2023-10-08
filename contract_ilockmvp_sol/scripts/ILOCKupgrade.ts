import { ethers, upgrades } from "hardhat";

import * as dotenv from "dotenv";
dotenv.config({ path: './.env.dev' });

const CONTRACT_ADDRESS = process.env.CONTRACT_ADDRESS;
const CONTRACT_UPGRADE = process.env.CONTRACT_UPGRADE;

async function main () {
  const ILOCKV2 = await ethers.getContractFactory(CONTRACT_UPGRADE);

  console.log('Upgrading ILOCK token contract...');
  const response = await upgrades.upgradeProxy(CONTRACT_ADDRESS, ILOCKV2);
  console.log('ILOCK token contract upgraded.');

  const receipt = {
	  "upgradeHash": response.deployTransaction.hash,
	  "upgradeBlockHash": response.deployTransaction.blockHash
  };
  const upgradeReceipt = {
	  "upgradeReceipt": receipt
  };
  console.log(upgradeReceipt);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
