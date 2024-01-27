"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const hardhat_1 = require("hardhat");
const signWhitelist_1 = __importDefault(require("./signWhitelist"));
const { expectRevert } = require("@openzeppelin/test-helpers");
describe("Token", function () {
    let contract;
    let mintingKey;
    let whitelistKey;
    let maliciousKey;
    beforeEach(async function () {
        const accounts = await hardhat_1.ethers.getSigners();
        mintingKey = accounts[0];
        whitelistKey = accounts[1];
        maliciousKey = accounts[2];
        const signer = hardhat_1.ethers.Wallet.createRandom();
        console.log('signing wallet address\n');
        const contract = new hardhat_1.ethers.Contract();
        const Token = await hardhat_1.ethers.getContractFactory("NFT");
        contract = await Token.deploy();
        await contract.deployed();
    });
    it("Should allow minting with whitelist enabled if a valid signature is sent", async function () {
        await contract.setWhitelistSigningAddress(whitelistKey.address);
        let { chainId } = await hardhat_1.ethers.provider.getNetwork();
        const sig = (0, signWhitelist_1.default)(chainId, contract.address, whitelistKey, mintingKey.address);
        console.log('signature: ' + sig);
        await contract.whitelistMint(sig);
    });
    it("Should not allow minting with whitelist enabled if a different signature is sent", async function () {
        await contract.setWhitelistSigningAddress(whitelistKey.address);
        let { chainId } = await hardhat_1.ethers.provider.getNetwork();
        const sig = (0, signWhitelist_1.default)(chainId, contract.address, maliciousKey, mintingKey.address);
        await expectRevert(contract.whitelistMint(sig), "Invalid Signature");
    });
});
