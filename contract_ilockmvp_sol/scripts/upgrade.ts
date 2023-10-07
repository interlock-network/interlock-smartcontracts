import { ethers, upgrades } from "hardhat";


async function main () {
  const BoxV2 = await ethers.getContractFactory('BoxV2');
  console.log('Upgrading Box...');
  const result = await upgrades.upgradeProxy('0x8A791620dd6260079BF849Dc5567aDC3F2FdC318', BoxV2);
  console.log('Box upgraded');
  console.log(result)
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
