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
const STAKE_LOG_PATH = process.env.STAKE_LOG_PATH;
const STAKE_DATA = JSON.parse((0, fs_1.readFileSync)(process.env.STAKE_DATA).toString());
let stakeholderStakes = [];
async function main() {
    const ILOCKV1 = await hardhat_1.ethers.getContractFactory(CONTRACT);
    const ilockv1 = await ILOCKV1.attach(PROXY_ADDRESS);
    for (const stakeholder of STAKE_DATA.stakeholders) {
        const identifiers = (await ilockv1.getStakeIdentifiers(stakeholder))
            .toString()
            .split(',');
        let stakes = [];
        for (const identifier of identifiers) {
            let stake = (await ilockv1.getStake(identifier))
                .toString()
                .split(',');
            stake = {
                "identifier": identifier,
                "share": stake[1],
                "paid": stake[2],
                "pool": stake[3]
            };
            stakes.push(stake);
        }
        stakes = {
            "stakeholder": stakeholder,
            "stakes": stakes
        };
        stakeholderStakes.push(stakes);
    }
    stakeholderStakes = {
        "stakeholderStakes": stakeholderStakes
    };
    (0, fs_1.writeFileSync)(STAKE_LOG_PATH, JSON.stringify(stakeholderStakes, null, 2), 'utf-8');
}
main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
