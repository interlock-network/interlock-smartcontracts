//
// INTERLOCK NETWORK - 
// PSP34 ACCESS NFT CLIENT LIBRARY
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
// monthsPassed
// 	args:	0
//
// ownable::owner
// 	args:	0
//
// poolData
// 	args:	1
// 	1:	pool number, 	u8
//
// psp22::allowance
// 	args:	2
// 	1:	owner, 		string (b58 address)
// 	2:	spender, 	string (b58 address)
//
// psp22::balanceOf
// 	args:	1
// 	1:	owner,		string (b58 address)
//
// psp22::totalSupply
// 	args: 	0
//
// psp22Metadata::tokenDecimals
// 	args:	0
//
// psp22Metadata::tokenName
// 	args:	0
//
// psp22Metadata::tokenSymbol
// 	args:	0
//
// rewardedTotal
// 	args:	0
//
// rewardedUserTotal
// 	args:	1
// 	1:	user,		string (b58 address)
//
// rewardsPoolBalance
// 	args:	0
//
// stakeholderData
// 	args:	1
// 	1:	stakeholder,	string (b58 address)
//
// remainingTimeUntilNextPayment
// 	args:	0
//
// whitelistPoolBalance
// 	args:	0
//
// publicsalePoolBalance
// 	args:	0
//
// partnersPoolBalance
// 	args:	0

// imports
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');
require('dotenv').config();

// constants
const metadata_ILOCKMVP = require('../target/ink/metadata.json');
const contract_ILOCKMVP = process.env.CONTRACT_ILOCKMVP;
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
