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
require("@nomicfoundation/hardhat-toolbox");
require("@nomicfoundation/hardhat-ethers");
require("@openzeppelin/hardhat-upgrades");
const dotenv = __importStar(require("dotenv"));
dotenv.config({ path: './.env.dev' });
const config = {
    solidity: {
        compilers: [
            {
                version: "0.8.19",
                settings: {
                    optimizer: {
                        enabled: true,
                        runs: 200,
                    },
                },
            }
        ]
    },
    networks: {
        // for testnet
        "base-goerli": {
            url: "https://goerli.base.org",
            accounts: [process.env.OWNER_PRIKEY],
            gasPrice: 1000000000,
        },
        "goerli": {
            url: process.env.ALCHEMY_URL,
            accounts: [process.env.OWNER_PRIKEY],
        },
    },
    etherscan: {
        url: "https://api-goerli.basescan.org/api",
        apiKey: process.env.ETHERSCAN_APIKEY
    }
};
exports.default = config;
