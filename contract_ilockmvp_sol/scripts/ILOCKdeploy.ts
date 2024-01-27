import { ethers, upgrades } from "hardhat";
import { writeFileSync } from "fs";

import * as dotenv from "dotenv";
dotenv.config({ path: './.env.dev' });

const CONTRACT = process.env.CONTRACT;
const ADMIN_LOG_PATH = process.env.ADMIN_LOG_PATH;
const CLAIM_LOG_PATH = process.env.CLAIM_LOG_PATH;
const IDENTIFIER_LOG_PATH = process.env.IDENTIFIER_LOG_PATH;

async function main () {

  const ILOCKV1 = await ethers.getContractFactory(CONTRACT);

  console.log('Deploying ILOCK token contract...');
  const ilockv1 = await upgrades.deployProxy(ILOCKV1, { initializer: 'initialize' });
  const response = await ilockv1.waitForDeployment()
  console.log('ILOCKV1 token contract deployed.');

  let deploymentReceipt = {
    "contractNetwork": "Arbitrum Mainnet",
    "contractProxyAddress": response.target,
    "dateAndTime": new Date().toUTCString()
  };
  deploymentReceipt = {
    "deploymentReceipt": deploymentReceipt
  };

  console.log(deploymentReceipt)
  writeFileSync(ADMIN_LOG_PATH, JSON.stringify([deploymentReceipt], null, 2), 'utf-8');
  writeFileSync(CLAIM_LOG_PATH, JSON.stringify([], null, 2), 'utf-8');
  writeFileSync(IDENTIFIER_LOG_PATH, JSON.stringify([], null, 2), 'utf-8');
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
