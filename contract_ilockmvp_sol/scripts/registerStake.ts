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

  let claimStubs = [];
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
    const claimStub = {
      "stakeholder": stake.stakeholder,
      "registrationHash": receipt.hash,
      "registrationBlockHash": receipt.blockHash,
			"stakeIdentifier": identifier
    }
    claimStubs.push(claimStub);
  }
  console.log(claimStubs);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
