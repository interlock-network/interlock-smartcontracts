"use strict";
exports.__esModule = true;
var program = require('commander');
var fs = require('fs');
var parse_balance_map_1 = require("../src/parse-balance-map");
program
    .version('0.0.0')
    .requiredOption('-i, --input <path>', 'input JSON file location containing a map of account addresses to string balances');
program.parse(process.argv);
var json = JSON.parse(fs.readFileSync(program.input, { encoding: 'utf8' }));
if (typeof json !== 'object')
    throw new Error('Invalid JSON');
console.log(JSON.stringify((0, parse_balance_map_1.parseBalanceMap)(json)));
