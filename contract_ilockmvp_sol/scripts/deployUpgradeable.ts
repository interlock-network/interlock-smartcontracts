import { ethers, upgrades } from "hardhat";


async function main () {
  const Box = await ethers.getContractFactory('Box');
  console.log('Deploying Box...');
  const box = await upgrades.deployProxy(Box, [42], { initializer: 'store' });
  console.log(box)
  await box.waitForDeployment()
  console.log('Box deployed to:', box);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
