import { Contract } from "@ethersproject/contracts";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
import { ethers } from "hardhat";
import signWhitelist from "./signWhitelist";

  let mintingKey: SignerWithAddress;
  let whitelistKey: SignerWithAddress;
  let maliciousKey: SignerWithAddress;

    const accounts = await ethers.getSigners();
    mintingKey = accounts[0];
    whitelistKey = accounts[1];
    console.log(mintingKey.address);
    console.log(whitelistKey.address);



    let { chainId } = 1;
    const sig = signWhitelist(
      chainId,
      '0xd9145CCE52D386f254917e481eB44e9943F39138',
      whitelistKey,
      mintingKey.address
    );


