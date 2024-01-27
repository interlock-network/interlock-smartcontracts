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
const SAFE_ADDRESS = process.env.SAFE_ADDRESS;
const OWNER_ADDRESS = process.env.OWNER_ADDRESS;
const ADMIN_LOG_PATH = process.env.ADMIN_LOG_PATH;
async function main() {
    console.log('Transferring admin contract ownership to Safe multisig account...');
    await hardhat_1.upgrades.admin.transferProxyAdminOwnership(SAFE_ADDRESS);
    console.log('Admin contract ownership transferred to Safe multisig account.');
    let multisigEnabledReceipt = {
        "oldAddress": OWNER_ADDRESS,
        "safeAddress": SAFE_ADDRESS,
        "dateAndTime": new Date().toUTCString()
    };
    multisigEnabledReceipt = {
        "multisigEnabledReceipt": multisigEnabledReceipt
    };
    console.log(multisigEnabledReceipt);
    const buffer = JSON.parse((0, fs_1.readFileSync)(ADMIN_LOG_PATH, 'utf8'));
    buffer.push(multisigEnabledReceipt);
    (0, fs_1.writeFileSync)(ADMIN_LOG_PATH, JSON.stringify(buffer, null, 2), 'utf-8');
}
main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
