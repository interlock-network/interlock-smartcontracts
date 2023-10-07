import { ethers, upgrades } from "hardhat";


async function main () {
  const ILOCKV2 = await ethers.getContractFactory('ILOCKV2');
  console.log('Upgrading ILOCK...');
  const result = await upgrades.upgradeProxy('0x51A1ceB83B83F1985a81C295d1fF28Afef186E02', ILOCKV2);
  console.log('ILOCK upgraded');
  console.log(result)
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
