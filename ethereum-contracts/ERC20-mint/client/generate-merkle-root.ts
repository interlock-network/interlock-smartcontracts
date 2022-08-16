const program = require('commander');
const fs = require('fs');
import { parseBalanceMap } from './parse-balance-map'

program
  .version('0.0.0')
  .requiredOption(
    '-i, --input <path>',
    'input JSON file location containing a map of account addresses to string balances'
  )

program.parse(process.argv)

const json = JSON.parse(fs.readFileSync(process.argv[3], { encoding: 'utf8' }))

if (typeof json !== 'object') throw new Error('Invalid JSON')

console.log(JSON.stringify(parseBalanceMap(json)))
