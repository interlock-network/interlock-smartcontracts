"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const commander_1 = require("commander");
const fs_1 = __importDefault(require("fs"));
const axios_1 = __importDefault(require("axios"));
const BATCH_SIZE = 10000;
commander_1.program
    .version('0.0.0')
    .requiredOption('-i, --input <path>', 'input JSON file location containing a claims tree')
    .requiredOption('-c, --chain-id <number>', 'chain ID of the merkle kv root')
    .requiredOption('-t, --token <string>', 'Cloudflare API token')
    .requiredOption('-a, --account-identifier <string>', 'Cloudflare account identifier')
    .requiredOption('-n, --namespace-identifier <string>', 'Cloudflare KV namespace identifier');
commander_1.program.parse(process.argv);
const json = JSON.parse(fs_1.default.readFileSync(commander_1.program.input, { encoding: 'utf8' }));
if (typeof json !== 'object')
    throw new Error('Invalid JSON');
async function main() {
    const KV = Object.keys(json.claims).map((account) => {
        const claim = json.claims[account];
        return {
            key: `${commander_1.program.chainId}:${account}`,
            value: JSON.stringify(claim),
        };
    });
    let i = 0;
    while (i < KV.length) {
        await axios_1.default
            .put(`https://api.cloudflare.com/client/v4/accounts/${commander_1.program.accountIdentifier}/storage/kv/namespaces/${commander_1.program.namespaceIdentifier}/bulk`, JSON.stringify(KV.slice(i, (i += BATCH_SIZE))), {
            maxBodyLength: Infinity,
            headers: { Authorization: `Bearer ${commander_1.program.token}`, 'Content-Type': 'application/json' },
        })
            .then((response) => {
            if (!response.data.success) {
                throw Error(response.data.errors);
            }
        });
        console.log(`Uploaded ${i} records in total`);
    }
}
main();
