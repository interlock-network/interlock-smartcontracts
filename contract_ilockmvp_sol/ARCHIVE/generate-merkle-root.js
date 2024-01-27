"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const program = require('commander');
const fs = require('fs');
const parse_balance_map_1 = require("./parse-balance-map");
program
    .version('0.0.0')
    .requiredOption('-i, --input <path>', 'input JSON file location containing a map of account addresses to string balances');
program.parse(process.argv);
const json = JSON.parse(fs.readFileSync(process.argv[3], { encoding: 'utf8' }));
if (typeof json !== 'object')
    throw new Error('Invalid JSON');
console.log(JSON.stringify((0, parse_balance_map_1.parseBalanceMap)(json)));
