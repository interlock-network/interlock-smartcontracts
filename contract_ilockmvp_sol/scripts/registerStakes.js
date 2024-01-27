"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    Object.defineProperty(o, k2, { enumerable: true, get: function() { return m[k]; } });
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
const hardhat_1 = require("hardhat");
const fs_1 = require("fs");
const dotenv = __importStar(require("dotenv"));
dotenv.config({ path: './.env.dev' });
const CONTRACT = process.env.CONTRACT;
const PROXY_ADDRESS = process.env.PROXY_ADDRESS;
const IDENTIFIER_LOG_PATH = process.env.IDENTIFIER_LOG_PATH;
const CLAIM_LOG_PATH = process.env.CLAIM_LOG_PATH;
const STAKE_DATA = JSON.parse((0, fs_1.readFileSync)(process.env.STAKE_DATA).toString());
let claimReceipts = [];
let stakeIdentifiers = [];
async function main() {
    const ILOCKV1 = await hardhat_1.ethers.getContractFactory(CONTRACT);
    const ilockv1 = await ILOCKV1.attach(PROXY_ADDRESS);
    for (const stake of STAKE_DATA.stakes) {
        const data = {
            "stakeholder": stake.stakeholder,
            "share": hardhat_1.ethers.parseEther(stake.share.toString()),
            "paid": hardhat_1.ethers.parseEther("0"),
            "pool": stake.pool
        };
        const response = await ilockv1.registerStake(data);
        const receipt = await response.wait();
        const stakeIdentifier = (await ilockv1.getStakeIdentifiers(stake.stakeholder))
            .toString()
            .split(',')
            .pop();
        console.log(stakeIdentifier);
        let claimReceipt = {
            "stakeholder": stake.stakeholder,
            "stakeIdentifier": stakeIdentifier,
            "registrationHash": receipt.hash,
            "registrationBlockHash": receipt.blockHash,
            "dateAndTime": new Date().toUTCString()
        };
        claimReceipt = {
            "claimReceipt": claimReceipt
        };
        claimReceipts = [claimReceipt].concat(claimReceipts);
        stakeIdentifiers = [stakeIdentifier].concat(stakeIdentifiers);
    }
    console.log(claimReceipts);
    console.log(stakeIdentifiers);
    let buffer = JSON.parse((0, fs_1.readFileSync)(CLAIM_LOG_PATH, 'utf8'));
    buffer = claimReceipts.concat(buffer);
    (0, fs_1.writeFileSync)(CLAIM_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');
    buffer = JSON.parse((0, fs_1.readFileSync)(IDENTIFIER_LOG_PATH, 'utf8'));
    buffer = stakeIdentifiers.concat(buffer);
    (0, fs_1.writeFileSync)(IDENTIFIER_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');
}
main().catch((error) => {
    console.error(error);
    if (claimReceipts.length > 0) {
        console.log(claimReceipts);
        console.log(stakeIdentifiers);
        let buffer = JSON.parse((0, fs_1.readFileSync)(CLAIM_LOG_PATH, 'utf8'));
        buffer = claimReceipts.concat(buffer);
        (0, fs_1.writeFileSync)(CLAIM_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');
        buffer = JSON.parse((0, fs_1.readFileSync)(IDENTIFIER_LOG_PATH, 'utf8'));
        buffer = stakeIdentifiers.concat(buffer);
        (0, fs_1.writeFileSync)(IDENTIFIER_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');
        console.log('gracefully logged incomplete batch of claim receipts and identifiers');
    }
    process.exitCode = 1;
});
