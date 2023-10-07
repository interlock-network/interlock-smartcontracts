import { ethers as hardhatEthers, upgrades } from "hardhat";
import { ethers } from "ethers";

async function main () {
  const ILOCKV1 = await hardhatEthers.getContractFactory('ILOCKV1');
  const ilockv1 = await ILOCKV1.attach('0x51A1ceB83B83F1985a81C295d1fF28Afef186E02');

  const stakeholder = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266';
  const paid = 0;
  const share = ethers.parseEther("100000");
  const pool = 1;

  const data = {
    paid: paid,
    share: share,
    pool: pool
  };

  await ilockv1.registerStake(stakeholder, data);
  //console.log((await ilockv1.newstorage()).toString())
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
