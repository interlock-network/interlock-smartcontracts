import { ethers, upgrades } from "hardhat";
import { readFileSync, writeFileSync } from "fs";

import * as dotenv from "dotenv";
dotenv.config({ path: './.env.dev' });

const CONTRACT = process.env.CONTRACT;
const CONTRACT_ADDRESS = process.env.CONTRACT_ADDRESS;
const ADMIN_LOG_PATH = process.env.ADMIN_LOG_PATH;

async function main () {

  const ILOCKV1 = await ethers.getContractFactory(CONTRACT);
  const ilockv1 = await ILOCKV1.attach(CONTRACT_ADDRESS);

  const response = await ilockv1.triggerTGE();
  const receipt = await response.wait();

  let TGEreceipt = {
    "hash": receipt.hash,
    "blockHash": receipt.blockHash
  };
  TGEreceipt = {
    "TGEreceipt": TGEreceipt
  };

  console.log(TGEreceipt);
  const buffer = JSON.parse(readFileSync(ADMIN_LOG_PATH, 'utf8'));
  buffer.push(TGEreceipt);
  writeFileSync(ADMIN_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
