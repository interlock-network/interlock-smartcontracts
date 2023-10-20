import { ethers, upgrades } from "hardhat";
import { readFileSync, writeFileSync, existsSync } from "fs";

import * as dotenv from "dotenv";
dotenv.config({ path: './.env.dev' });

const CONTRACT = process.env.CONTRACT;
const PROXY_ADDRESS = process.env.PROXY_ADDRESS;
const IDENTIFIER_LOG_PATH = process.env.IDENTIFIER_LOG_PATH;
const CLAIM_LOG_PATH = process.env.CLAIM_LOG_PATH;
const STAKE_DATA = JSON.parse(readFileSync(process.env.STAKE_DATA).toString());

let claimReceipts = [];
let stakeIdentifiers = [];
async function main () {

  const ILOCKV1 = await ethers.getContractFactory(CONTRACT);
  const ilockv1 = await ILOCKV1.attach(PROXY_ADDRESS);

  for (const stake of STAKE_DATA.stakes) {

    const data = {
      "stakeholder": stake.stakeholder,
      "share": ethers.parseEther(stake.share.toString()),
      "paid": ethers.parseEther("0"),
      "pool": stake.pool
    }
    const response = await ilockv1.registerStake(data);
    const receipt = await response.wait();

    const stakeIdentifier = (await ilockv1.getStakeIdentifiers(stake.stakeholder))
                                          .toString()
                                          .split(',')
                                          .pop();
    console.log(stakeIdentifier);
    let claimReceipt = {
      "stakeholder": stake.stakeholder,
      "stakeIdentifier": stakeIdentifier,
      "registrationHash": receipt.hash,
      "registrationBlockHash": receipt.blockHash,
      "dateAndTime": new Date().toUTCString()
    }
    claimReceipt = {
      "claimReceipt": claimReceipt
    };

    claimReceipts = [claimReceipt].concat(claimReceipts);
    stakeIdentifiers = [stakeIdentifier].concat(stakeIdentifiers);
  }

  console.log(claimReceipts);
  console.log(stakeIdentifiers);

  let buffer = JSON.parse(readFileSync(CLAIM_LOG_PATH, 'utf8'));
  buffer = claimReceipts.concat(buffer);
  writeFileSync(CLAIM_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');

  buffer = JSON.parse(readFileSync(IDENTIFIER_LOG_PATH, 'utf8'));
  buffer = stakeIdentifiers.concat(buffer);
  writeFileSync(IDENTIFIER_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');
}

main().catch((error) => {

  console.error(error);
  
  if (claimReceipts.length > 0) {

    console.log(claimReceipts);
    console.log(stakeIdentifiers);

    let buffer = JSON.parse(readFileSync(CLAIM_LOG_PATH, 'utf8'));
    buffer = claimReceipts.concat(buffer);
    writeFileSync(CLAIM_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');

    buffer = JSON.parse(readFileSync(IDENTIFIER_LOG_PATH, 'utf8'));
    buffer = stakeIdentifiers.concat(buffer);
    writeFileSync(IDENTIFIER_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');

    console.log('gracefully logged incomplete batch of claim receipts and identifiers');
  }
  process.exitCode = 1;
});
