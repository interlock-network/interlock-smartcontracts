//
// INTERLOCK NETWORK - 
// PORT CONTRACT TEMPLATE
//

//
// BASH CALLING SYNTAX:
//
// node sendTransaction.js <method> <arg1> <arg2> ...
//

//
// METHODS:
//
// Enter each method below as a string in <method> field. Quotes are unneccessary.
//
// register
// 	args:	0
//
// callSocket
// 	args:	2
// 	1:	address,	b58 string
// 	2:	amount,		big number
//
// Further implemented methods below


// imports
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');
require('dotenv').config();

// constants
const metadata_ILOCKMVP = require('../target/ink/metadata.json');
const contract_ILOCKMVP = process.env.CONTRACT_PORT;
const OWNER_MNEMONIC = require('./.mnemonic.json');
const OWNER_mnemonic = OWNER_MNEMONIC.mnemonic;

// constants
const MEG = 1000000;
const gasLimit = 100000 * MEG;
const storageDepositLimit = null; // nolimit

// note about gas:
// Web UI specifies gas in terms of AZERO.
// Units for polkadot.js RCP calls are specified in microAZERO.
// Thus, here, specify gas as (AZERO * 10^6).

async function sendTransaction(...args) {

	try {

		// pop off `node` arg, pop off `sendTransaction.js` arg, pop off `method` arg
		args.shift();
		args.shift();
		let method = args.shift();

		// setup session
		const wsProvider = new WsProvider('wss://ws.test.azero.dev');
		const keyring = new Keyring({type: 'sr25519'});
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, metadata_ILOCKMVP, contract_ILOCKMVP);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// perform dry run to check for errors
		// if we do not first perform dry run, we lose the gas we spend when attemping tx
		const { gasRequired, storageDeposit, result, output } =
			await contract.query[method](
  			OWNER_pair.address, {}, ...args);

		// too much gas required?
		if (gasRequired > gasLimit) {
			console.log('tx aborted, gas required is greater than the acceptable gas limit.');
			process.exit();
		}

		// too much storage required?
		if (storageDeposit > storageDepositLimit) {
			console.log('tx aborted, storage required is greater than the acceptable storage limit.');
			process.exit();
		}

		// did the contract revert due to any errors?
		if (result.toHuman().Ok.flags == 'Revert') {
			let error = output;
			console.log(`Transaction reverts due to error: ${error}`);
			process.exit();
		}

		// submit doer tx
		let extrinsic = await contract.tx[method]
  			({ storageDepositLimit, gasLimit }, ...args)
  			.signAndSend(OWNER_pair, result => {
    			if (result.status.isInBlock) {
      				console.log('in a block');
    			} else if (result.status.isFinalized) {
      				console.log('finalized');
				process.exit();
    			}
  		});

	} catch(error) {

		console.log(error);
		process.exit();
	}
}

sendTransaction(...process.argv);
