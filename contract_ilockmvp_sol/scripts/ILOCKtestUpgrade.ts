import { ethers, upgrades } from "hardhat";


async function main () {
  const ILOCKV1 = await ethers.getContractFactory('ILOCKV2');
  const ilockv1 = await ILOCKV1.attach('0x51A1ceB83B83F1985a81C295d1fF28Afef186E02');
  //await ilockv1.testingIncrementMonth()
  //:w
  console.log((await ilockv1.monthsPassed()).toString())
  console.log((await ilockv1.newstorage()).toString())
  console.log((await ilockv1.newFeature()))
 // console.log(result)
  //const receipt = await result.wait();
  //console.log(receipt)

  //console.log((await box.retrieve()).toString());
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
