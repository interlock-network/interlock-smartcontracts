import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
const {ethers} = require("hardhat");

export default async function signWhitelist(
  chainId: number,
  contractAddress: string,
  whitelistKey: SignerWithAddress,
  mintingAddress: string
) {
  // Domain data should match whats specified in the DOMAIN_SEPARATOR constructed in the contract
  // https://github.com/msfeldstein/EIP712-whitelisting/blob/main/contracts/EIP712Whitelisting.sol#L33-L43
  const domain = {
    name: "WhitelistToken",
    version: "1",
    chainId: 1,
    verifyingContract: contractAddress,
  };




  // The types should match the TYPEHASH specified in the contract
  // https://github.com/msfeldstein/EIP712-whitelisting/blob/main/contracts/EIP712Whitelisting.sol#L27-L28
  const types = {
    Minter: [{ name: "wallet", type: "address" }],
  };

  console.log(mintingAddress);
  console.log(whitelistKey.address);
  const sig = await whitelistKey._signTypedData(domain, types, {
    wallet: mintingAddress,
  });

  return sig
}


