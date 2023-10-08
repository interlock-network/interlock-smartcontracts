import { ethers as hardhatEthers, upgrades } from "hardhat";
import { ethers } from "ethers";
import { readFileSync, writeFileSync, existsSync } from "fs";

import * as dotenv from "dotenv";
dotenv.config({ path: './.env.dev' });

const CONTRACT = process.env.CONTRACT;
const CONTRACT_ADDRESS = process.env.CONTRACT_ADDRESS;
const IDENTIFIER_LOG_PATH = process.env.IDENTIFIER_LOG_PATH;
const CLAIM_LOG_PATH = process.env.CLAIM_LOG_PATH;
const STAKE_DATA = JSON.parse(readFileSync(process.env.STAKE_DATA).toString());

let claimReceipts = [];
let claimIdentifiers = [];
async function main () {

  if (!existsSync(CLAIM_LOG_PATH)) { 
		writeFileSync(CLAIM_LOG_PATH, JSON.stringify([], null, 2), 'utf-8');
	}
  if (!existsSync(IDENTIFIER_LOG_PATH)) { 
		writeFileSync(IDENTIFIER_LOG_PATH, JSON.stringify([], null, 2), 'utf-8');
	}

  const ILOCKV1 = await hardhatEthers.getContractFactory(CONTRACT);
  const ilockv1 = await ILOCKV1.attach(CONTRACT_ADDRESS);

  for (const stake of STAKE_DATA.stakes) {

    const data = {
      "paid": 0,
      "share": stake.share,
      "pool": stake.pool
    }
    const response = await ilockv1.registerStake(stake.stakeholder, data);
    const receipt = await response.wait();

    const identifier = (await ilockv1.getStakeIdentifiers(stake.stakeholder))
                                     .toString()
                                     .split(',')
                                     .pop();
    let claimReceipt = {
      "stakeholder": stake.stakeholder,
      "registrationHash": receipt.hash,
      "registrationBlockHash": receipt.blockHash,
      "stakeIdentifier": identifier
    }
    claimReceipt = {
	    "claimReceipt": claimReceipt
    };
    claimReceipts = [claimReceipt].concat(claimReceipts);
    claimIdentifiers = [identifier].concat(claimIdentifiers);
  }

  console.log(claimReceipts);
	console.log(claimIdentifiers);

  let buffer = JSON.parse(readFileSync(CLAIM_LOG_PATH, 'utf8'));
  buffer = claimReceipts.concat(buffer);
  writeFileSync(CLAIM_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');

	buffer = JSON.parse(readFileSync(IDENTIFIER_LOG_PATH, 'utf8'));
	buffer = claimIdentifiers.concat(buffer);
	writeFileSync(IDENTIFIER_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');
}

main().catch((error) => {

  console.error(error);
  
	if (claimReceipts.length > 0) {

  	console.log(claimReceipts);
		console.log(claimIdentifiers);

  	let buffer = JSON.parse(readFileSync(CLAIM_LOG_PATH, 'utf8'));
  	buffer = claimReceipts.concat(buffer);
  	writeFileSync(CLAIM_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');

  	buffer = JSON.parse(readFileSync(IDENTIFIER_LOG_PATH, 'utf8'));
  	buffer = claimIdentifiers.concat(buffer);
  	writeFileSync(IDENTIFIER_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');

  	console.log('gracefully logged incomplete batch of claim receipts and identifiers');
	}
  process.exitCode = 1;
});
