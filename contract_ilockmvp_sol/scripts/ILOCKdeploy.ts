import { ethers, upgrades } from "hardhat";

import * as dotenv from "dotenv";
dotenv.config({ path: './.env.dev' });

const CONTRACT_ADDRESS = process.env.CONTRACT_ADDRESS;
const CONTRACT = process.env.CONTRACT;

async function main () {

  const ILOCKV1 = await ethers.getContractFactory(CONTRACT);

  console.log('Deploying ILOCK token contract...');
  const ilockv1 = await upgrades.deployProxy(ILOCKV1, { initializer: 'initialize' });
  const response = await ilockv1.waitForDeployment()
  console.log('ILOCKV1 token contract deployed.');

  const receipt = {
	  "contractNetwork": "Arbitrum Mainnet",
		"contractAddress": response.target
  };
  const deploymentReceipt = {
	  "deploymentReceipt": receipt
  };
  console.log(deploymentReceipt)
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
