import { ethers as hardhatEthers, upgrades } from "hardhat";
import { ethers } from "ethers";

require("dotenv").config();

import * as dotenv from "dotenv";
dotenv.config({ path: './.env.dev' });

const CONTRACT = 'ILOCKV1';
const CONTRACT_ADDRESS = process.env.CONTRACT_ADDRESS;


async function main () {
  const ILOCKV1 = await hardhatEthers.getContractFactory(CONTRACT);
  const ilockv1 = await ILOCKV1.attach(CONTRACT_ADDRESS);

  const stakeholder = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';
  const paid = 0;
  const share = ethers.parseEther("100000");
  const pool = 1;

  const data = {
    paid: paid,
    share: share,
    pool: pool
  };

  console.log(await ilockv1.getStakeIdentifiers(stakeholder));
  //console.log((await ilockv1.newstorage()).toString())
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
