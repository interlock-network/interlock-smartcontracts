"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const merkle_tree_1 = __importDefault(require("./merkle-tree"));
const ethers_1 = require("ethers");
class BalanceTree {
    constructor(balances) {
        this.tree = new merkle_tree_1.default(balances.map(({ account, share, owes, pool }, index) => {
            return BalanceTree.toNode(index, account, share, owes, pool);
        }));
    }
    static verifyProof(index, account, share, owes, pool, proof, root) {
        let pair = BalanceTree.toNode(index, account, share, owes, pool);
        for (const item of proof) {
            pair = merkle_tree_1.default.combinedHash(pair, item);
        }
        return pair.equals(root);
    }
    // keccak256(abi.encode(index, account, share))
    static toNode(index, account, share, owes, pool) {
        return Buffer.from(ethers_1.utils.solidityKeccak256(['uint256', 'address', 'uint256', 'uint256', 'uint256'], [index, account, share, owes, pool]).substr(2), 'hex');
    }
    getHexRoot() {
        return this.tree.getHexRoot();
    }
    // returns the hex bytes32 values of the proof
    getProof(index, account, share, owes, pool) {
        return this.tree.getHexProof(BalanceTree.toNode(index, account, share, owes, pool));
    }
}
exports.default = BalanceTree;
