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
const SAFE_ADDRESS = process.env.SAFE_ADDRESS;
const ADMIN_LOG_PATH = process.env.ADMIN_LOG_PATH;
async function main() {
    const ILOCKV1 = await hardhat_1.ethers.getContractFactory(CONTRACT);
    const ilockv1 = await ILOCKV1.attach(PROXY_ADDRESS);
    const response = await ilockv1.triggerTGE(SAFE_ADDRESS);
    const receipt = await response.wait();
    let TGEreceipt = {
        "hash": receipt.hash,
        "blockHash": receipt.blockHash,
        "dateAndTime": new Date().toUTCString()
    };
    TGEreceipt = {
        "TGEreceipt": TGEreceipt
    };
    console.log(TGEreceipt);
    const buffer = JSON.parse((0, fs_1.readFileSync)(ADMIN_LOG_PATH, 'utf8'));
    buffer.push(TGEreceipt);
    (0, fs_1.writeFileSync)(ADMIN_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');
}
main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
