import { ethers, upgrades } from "hardhat";
import { readFileSync, writeFileSync } from "fs";

import * as dotenv from "dotenv";
dotenv.config({ path: './.env.dev' });

const CONTRACT = process.env.CONTRACT;
const PROXY_ADDRESS = process.env.PROXY_ADDRESS;
const SAFE_ADDRESS = process.env.SAFE_ADDRESS;
const ADMIN_LOG_PATH = process.env.ADMIN_LOG_PATH;

async function main () {

  const ILOCKV1 = await ethers.getContractFactory(CONTRACT);
  const ilockv1 = await ILOCKV1.attach(PROXY_ADDRESS);

  const response = await ilockv1.triggerTGE(SAFE_ADDRESS);
  const receipt = await response.wait();

  let TGEreceipt = {
    "hash": receipt.hash,
    "blockHash": receipt.blockHash,
    "dateAndTime": new Date().toUTCString()
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
