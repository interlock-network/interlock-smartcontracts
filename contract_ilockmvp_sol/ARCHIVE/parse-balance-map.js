"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.parseBalanceMap = void 0;
const ethers_1 = require("ethers");
const balance_tree_1 = __importDefault(require("./balance-tree"));
const { isAddress, getAddress } = ethers_1.utils;
function parseBalanceMap(balances) {
    // if balances are in an old format, process them
    const balancesInNewFormat = Array.isArray(balances)
        ? balances
        : Object.keys(balances).map((account) => ({
            address: account,
            earnings: `0x${balances[account][0].toString(16)}`,
            oweswhat: `0x${balances[account][1].toString(16)}`,
            whichpool: `0x${balances[account][2].toString(16)}`,
            reasons: '',
        }));
    const dataByAddress = balancesInNewFormat.reduce((memo, { address: account, earnings, oweswhat, whichpool, reasons }) => {
        if (!isAddress(account)) {
            throw new Error(`Found invalid address: ${account}`);
        }
        const parsed = getAddress(account);
        if (memo[parsed])
            throw new Error(`Duplicate address: ${parsed}`);
        const parsedNum1 = ethers_1.BigNumber.from(earnings);
        if (parsedNum1.lte(0))
            throw new Error(`Invalid share for account: ${account}`);
        const parsedNum2 = ethers_1.BigNumber.from(oweswhat);
        //if (parsedNum2.lte(0)) throw new Error(`Invalid owes for account: ${account}`)
        const parsedNum3 = ethers_1.BigNumber.from(whichpool);
        //if (parsedNum3.lte(0)) throw new Error(`Invalid pool for account: ${account}`)
        const flags = {
            isSOCKS: reasons.includes('socks'),
            isLP: reasons.includes('lp'),
            isUser: reasons.includes('user'),
        };
        memo[parsed] = { share: parsedNum1, owes: parsedNum2, pool: parsedNum3, ...(reasons === '' ? {} : { flags }) };
        return memo;
    }, {});
    const sortedAddresses = Object.keys(dataByAddress).sort();
    // construct a tree
    const tree = new balance_tree_1.default(sortedAddresses.map((address) => ({ account: address, share: dataByAddress[address].share, owes: dataByAddress[address].owes, pool: dataByAddress[address].pool })));
    // generate claims
    const claims = sortedAddresses.reduce((memo, address, index) => {
        const { share, owes, pool, flags } = dataByAddress[address];
        memo[address] = {
            index,
            share: share.toHexString(),
            owes: owes.toHexString(),
            pool: pool.toHexString(),
            proof: tree.getProof(index, address, share, owes, pool),
            ...(flags ? { flags } : {}),
        };
        return memo;
    }, {});
    const tokenTotal = sortedAddresses.reduce((memo, key) => memo.add(dataByAddress[key].share), ethers_1.BigNumber.from(0));
    return {
        merkleRoot: tree.getHexRoot(),
        tokenTotal: tokenTotal.toHexString(),
        claims,
    };
}
exports.parseBalanceMap = parseBalanceMap;
