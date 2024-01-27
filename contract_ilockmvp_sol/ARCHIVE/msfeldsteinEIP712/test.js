"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const hardhat_1 = require("hardhat");
const signWhitelist_1 = __importDefault(require("./signWhitelist"));
let mintingKey;
let whitelistKey;
let maliciousKey;
const accounts = await hardhat_1.ethers.getSigners();
mintingKey = accounts[0];
whitelistKey = accounts[1];
console.log(mintingKey.address);
console.log(whitelistKey.address);
let { chainId } = 1;
const sig = (0, signWhitelist_1.default)(chainId, '0xd9145CCE52D386f254917e481eB44e9943F39138', whitelistKey, mintingKey.address);
