//
// INTERLOCK NETWORK - 
// PORT CONTRACT TEMPLATE
//

//
// BASH CALLING SYNTAX:
//
// node queryContract.js <method> <arg1> <arg2> ...
//

// METHODS:
//
// Enter each method below as a string in <method> field. Quotes are unneccessary.
//
// No methods yet.


// imports
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');
require('dotenv').config();

// constants
const metadata_ILOCKMVP = require('../target/ink/metadata.json');
const contract_ILOCKMVP = process.env.CONTRACT_PORT;
const OWNER_MNEMONIC = require('./.mnemonic.json');
const OWNER_mnemonic = OWNER_MNEMONIC.mnemonic;

async function queryContract(...args) {

	try {

		// pop off `node` arg, pop off `script` arg, pop off `method` arg
		args.shift();
		args.shift();
		let method = args.shift();

		// setup session
		const wsProvider = new WsProvider('wss://ws.test.azero.dev');
		const keyring = new Keyring({type: 'sr25519'});
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, metadata_ILOCKMVP, contract_ILOCKMVP);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit getter request
		const { gasRequired, storageDeposit, result, output } =
			await contract.query[method](
  			OWNER_pair.address, {}, ...args);

		// check if the call was successful
		if (result.isOk) {
			console.log('Result: ' + output);
			process.exit();
		} else {
  			console.error('Error: ', result);
			process.exit();
		}

	} catch(error) {

		console.log(error);
		process.exit()
	}
}

queryContract(...process.argv);
