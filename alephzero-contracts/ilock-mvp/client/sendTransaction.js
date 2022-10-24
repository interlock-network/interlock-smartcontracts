//
// INTERLOCK NETWORK - 
// PSP22 TOKEN & MVP CLIENT SCRIPT
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
// checkTime
// 	args:	0
//
// decrementCirculation
// 	args:	1
// 	1:	amount, 	big number
//
// distributeTokens
// 	args:	1
// 	1:	stakeholder, 	string (b58 address)
//
// incrementCirculation
// 	args:	1
// 	1:	amount, 	big number
//
// ownable::renounceOwnership
// 	args:	0
//
// ownable::transferOwnership
// 	args: 	1
// 	1:	newowner	string (b58 address)
//
// psp22::approve
// 	args:	2
// 	1:	spender, 	string (b58 address)
// 	2:	value,		big number
//
// psp22::decreaseAllowance
// 	args:	2
// 	1:	spender,	string (b58 address)
// 	2:	deltavalue,	big number
//
// psp22::increaseAllowance
// 	args:	2
// 	1:	spender,	string (b58 address)
// 	2:	deltavalue,	big number
//
// psp22::transfer
// 	args:	0
// 	1:	to,		string (b58 address)
// 	2:	value,		big number
// 	3:	data,		byte string
//
// psp22::transferFrom
// 	args:	4
// 	1:	from,		string (b58 address)
// 	2:	to,		string (b58 address)
// 	3:	value,		big number
// 	4:	data,		byte string
//
// psp22Burnable::burn
// 	args:	2
// 	1:	account,	string (b58 address)
// 	2:	amount,		big number
//
// registerStakeholder
// 	args:	3
// 	1:	stakeholder,	string (b58 address)
// 	2:	share,		big number
// 	3:	pool,		u8
//
// rewardUser
// 	args:	2
// 	1:	reward,		big number
// 	2:	user,		string (b58 address)
//
// updateContract
// 	args:	1
// 	1:	codehash,	bytestring
//

// imports
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');
require('dotenv').config();

// constants
const metadata_ILOCKMVP = require('../target/ink/metadata.json');
const contract_ILOCKMVP = process.env.CONTRACT_ILOCKMVP;
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
			let error = output.toHuman().Err;
			console.log(`Transaction reverts due to error: ${error}`);
			process.exit();
		}

		// submit doer tx
		let extrinsic = await contract.tx[method]
  			({ storageDeposit, gasRequired }, ...args)
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
