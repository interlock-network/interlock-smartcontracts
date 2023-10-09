import { ethers, upgrades } from "hardhat";
import { readFileSync } from "fs";

import * as dotenv from "dotenv";
dotenv.config({ path: './.env.dev' });

const CONTRACT = process.env.CONTRACT;
const PROXY_ADDRESS = process.env.PROXY_ADDRESS;
const STAKE_DATA = JSON.parse(readFileSync(process.env.STAKE_DATA).toString());
const IDENTIFIER_LOG_DATA = JSON.parse(readFileSync(process.env.IDENTIFIER_LOG_PATH).toString());
const STAKE_LOG_DATA = JSON.parse(readFileSync(process.env.STAKE_LOG_PATH).toString());

async function main () {
  const ILOCKV1 = await ethers.getContractFactory(CONTRACT);
  const ilockv1 = await ILOCKV1.attach(PROXY_ADDRESS);

	let stakeholders = []

  for (stake of STAKE_DATA.stakes) {

					if (!stakeholders.includes(stake.stakeholder)) {
									stakeholders.push(stake.stakeholder);
					}
	}
	if (stakeholders.length != STAKE_DATA.stakeholders.length) {
					console.log('registered stakeholders are either missing or contain extra stakes');
					process.exitCode = 1;
	}

	let loggedStakes = []
	const stakeholderStakes = STAKE_LOG_DATA.stakeholderStakes;
	for (stakeholder of stakeholderStakes) {
					if (!loggedStakes.includes(stakes.identifier)) {
									console.log('missing stakeholder');
					}

    



/*
 *
 *
 *
 *
 *
 *
 *
 *
 *
 * This is to do. Not that important at the time being 10/09/23
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 */



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
