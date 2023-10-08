import { ethers as hardhatEthers, upgrades } from "hardhat";
import { ethers } from "ethers";
import { readFileSync } from "fs";

import * as dotenv from "dotenv";
dotenv.config({ path: './.env.dev' });

const CONTRACT = process.env.CONTRACT;
const CONTRACT_ADDRESS = process.env.CONTRACT_ADDRESS;
const STAKE_DATA = JSON.parse(readFileSync(process.env.STAKE_DATA).toString());

async function main () {
  const ILOCKV1 = await hardhatEthers.getContractFactory(CONTRACT);
  const ilockv1 = await ILOCKV1.attach(CONTRACT_ADDRESS);


  for (const stake of STAKE_DATA.stakes) {

    const data = {
	    "paid": 0,
	    "share": stake.share,
	    "pool": stake.pool
    }
  await ilockv1.registerStake(stake.stakeholder, data);
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
