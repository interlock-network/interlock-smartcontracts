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

  let stakeholderIdentifiers = [];
  for (const stakeholder of STAKE_DATA.stakeholders) {

    const identifiers = {
      "stakeholder": stakeholder,
      "identifiers": (await ilockv1.getStakeIdentifiers(stakeholder)).toString().split(',')
    };
    stakeholderIdentifiers.push(identifiers)
  }
  console.log(stakeholderIdentifiers);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
