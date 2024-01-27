"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const abi = require('./whitelistabi.json');
const { ethers } = require("hardhat");
const signingscript_1 = __importDefault(require("./signingscript"));
//const { expectRevert } = require("@openzeppelin/test-helpers");
async function test() {
    try {
        // let contract: Contract;
        let mintingKey;
        let whitelistKey;
        let maliciousKey;
        const accounts = await ethers.getSigners();
        mintingKey = accounts[0];
        whitelistKey = accounts[1];
        maliciousKey = accounts[2];
        console.log(whitelistKey);
        console.log(mintingKey);
        //const accounts = await ethers.getSigners();
        //mintingKey = accounts[0];
        //whitelistKey = accounts[1];
        //maliciousKey = accounts[2];
        //
        /*
          var wallet = Wallet.fromMnemonic("rain pulp aware feature witness virus soccer cup spray gown sort exit");
          wallet = new Wallet(wallet.privateKey);
        
            //const signer = ethers.Wallet.createRandom();
            console.log('signing wallet address\n');
            const contract = new Contract('0xcD6a42782d230D7c13A74ddec5dD140e55499Df9',abi,mintingKey);
        */
        //////const signerWithAddress = new SignerWithAddress(wallet.address, wallet);
        /*
            const Token = await ethers.getContractFactory("NFT");
            contract = await Token.deploy();
            await contract.deployed();*/
        //it("Should allow minting with whitelist enabled if a valid signature is sent", async function () {
        //await contract.setWhitelistSigningAddress(whitelistKey.address);
        //let { chainId } = await ethers.provider.getNetwork();
        const sig = await (0, signingscript_1.default)(1, '0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC', whitelistKey, mintingKey.address);
        console.log(whitelistKey.address);
        console.log('signature: ' + sig);
        // await contract.whitelistMint(sig);
        // });
        /*
          it("Should not allow minting with whitelist enabled if a different signature is sent", async function () {
            await contract.setWhitelistSigningAddress(whitelistKey.address);
            let { chainId } = await ethers.provider.getNetwork();
            const sig = signWhitelist(chainId, contract.address, maliciousKey, mintingKey.address)
            await expectRevert(contract.whitelistMint(sig), "Invalid Signature");
          });*/
    }
    catch (error) {
        console.log(error);
    }
}
;
test();
