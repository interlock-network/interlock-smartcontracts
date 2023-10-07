import { ethers, upgrades } from "hardhat";

async function main () {
  const ILOCKV1 = await ethers.getContractFactory('ILOCKV1');
  console.log('Deploying ILOCKV1 token...');
  const ilockv1 = await upgrades.deployProxy(ILOCKV1, { initializer: 'initialize' });
  await ilockv1.waitForDeployment()
  console.log('ILOCKV1 token deployed to: ', ilockv1.target);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
