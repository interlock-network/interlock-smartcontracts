import { ethers, upgrades } from "hardhat";
import { readFileSync, writeFileSync } from "fs";

import * as dotenv from "dotenv";
dotenv.config({ path: './.env.dev' });

const PROXY_ADDRESS = process.env.PROXY_ADDRESS;
const CONTRACT_UPGRADE = process.env.CONTRACT_UPGRADE;
const ADMIN_LOG_PATH = process.env.ADMIN_LOG_PATH;

async function main () {

  const ILOCKV2 = await ethers.getContractFactory(CONTRACT_UPGRADE);

  console.log('Preparing  ILOCK token contract upgrade...');
  const response = await upgrades.upgradeProxy(PROXY_ADDRESS, ILOCKV2);
  console.log('ILOCK token contract upgrade prepared.');

  let upgradePreparedReceipt = {
    "contractVersion": CONTRACT_UPGRADE,
    "hash": response.deployTransaction.hash,
    "blockHash": response.deployTransaction.blockHash,
    "dateAndTime": new Date().toUTCString()
  };
  upgradePreparedReceipt = {
    "upgradePreparedReceipt": upgradePreparedReceipt
  };

  console.log(upgradePreparedReceipt);
  const buffer = JSON.parse(readFileSync(ADMIN_LOG_PATH, 'utf8'));
  buffer.push(upgradePreparedReceipt);
  writeFileSync(ADMIN_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
