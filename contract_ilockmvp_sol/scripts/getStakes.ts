import { ethers, upgrades } from "hardhat";
import { readFileSync, writeFileSync, existsSync } from "fs";

import * as dotenv from "dotenv";
dotenv.config({ path: './.env.dev' });

const CONTRACT = process.env.CONTRACT;
const CONTRACT_ADDRESS = process.env.CONTRACT_ADDRESS;
const STAKE_LOG_PATH = process.env.STAKE_LOG_PATH;
const STAKE_DATA = JSON.parse(readFileSync(process.env.STAKE_DATA).toString());

let stakeholderStakes = [];
async function main () {

  const ILOCKV1 = await ethers.getContractFactory(CONTRACT);
  const ilockv1 = await ILOCKV1.attach(CONTRACT_ADDRESS);

  for (const stakeholder of STAKE_DATA.stakeholders) {

    const identifiers = (await ilockv1.getStakeIdentifiers(stakeholder))
                                     .toString()
                                     .split(',');
    let stakes = [];
    for (const identifier of identifiers) {
      
      let stake = (await ilockv1.getStake(stakeholder, identifier))
                                .toString()
                                .split(',');
      stake = {
        "identifier": identifier,
        "share": stake[0],
        "paid": stake[1],
        "pool": stake[2]
      };
      stakes.push(stake);
    }

    stakes = {
      "stakeholder": stakeholder,
      "stakes": stakes
    };
    stakeholderStakes.push(stakes);
  }

  stakeholderStakes = {
    "stakeholderStakes": stakeholderStakes
  };
  writeFileSync(STAKE_LOG_PATH, JSON.stringify(stakeholderStakes, null, 2), 'utf-8');
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
