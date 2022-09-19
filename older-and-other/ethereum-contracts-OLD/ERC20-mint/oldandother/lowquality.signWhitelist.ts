//import {ethers} from 'ethers';
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";

export default async function signWhitelist() {

try {

//var whitelistKey = new ethers.utils.SigningKey('0x65b48db9af3bd213e47aae184e66bd62f2189f5706b333048c480fa514d9bb18');
//whitelistKey = ethers.utils.formatBytes32String(whitelistKey);
	//
var whitelistwallet = new ethers.Wallet('0x65b48db9af3bd213e47aae184e66bd62f2189f5706b333048c480fa514d9bb18');
console.log(whitelistwallet);

  // Domain data should match whats specified in the DOMAIN_SEPARATOR constructed in the contract
  // https://github.com/msfeldstein/EIP712-whitelisting/blob/main/contracts/EIP712Whitelisting.sol#L33-L43
  const domain = {
    name: "Validator",
    version: "1",
    chainId: 1,
    verifyingContract: '0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC',
  };

  // The types should match the TYPEHASH specified in the contract
  // https://github.com/msfeldstein/EIP712-whitelisting/blob/main/contracts/EIP712Whitelisting.sol#L27-L28
  const types = {
    Validation: [
	    { name: "wallet", type: "address" },
	    { name: "share", type: "uint256" },
	    { name: "pool", type: "uint8" },
  	],
  };

  const sig = await whitelistwallet._signTypedData(domain, types, {
    wallet: "0xeeBA65D9C7E5832918d1F4277DE0a78b78efEC43",
    share: 1000,
    pool: 5
  });
  console.log(sig);

  console.log(whitelistwallet.address);

}catch (error){
	console.log(error);
}
}

signWhitelist();
