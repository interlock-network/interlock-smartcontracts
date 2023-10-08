import { ethers, upgrades } from "hardhat";
import { readFileSync, writeFileSync } from "fs";

import * as dotenv from "dotenv";
dotenv.config({ path: './.env.dev' });

const CONTRACT_ADDRESS = process.env.CONTRACT_ADDRESS;
const CONTRACT_UPGRADE = process.env.CONTRACT_UPGRADE;
const ADMIN_LOG_PATH = process.env.ADMIN_LOG_PATH;

async function main () {

  const ILOCKV2 = await ethers.getContractFactory(CONTRACT_UPGRADE);

  console.log('Upgrading ILOCK token contract...');
  const response = await upgrades.upgradeProxy(CONTRACT_ADDRESS, ILOCKV2);
  console.log('ILOCK token contract upgraded.');

  let upgradeReceipt = {
    "hash": response.deployTransaction.hash,
    "blockHash": response.deployTransaction.blockHash
  };
  upgradeReceipt = {
    "upgradeReceipt": upgradeReceipt
  };

  console.log(upgradeReceipt);
	const buffer = JSON.parse(readFileSync(ADMIN_LOG_PATH, 'utf8'));
  buffer.push(upgradeReceipt);
  writeFileSync(ADMIN_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
