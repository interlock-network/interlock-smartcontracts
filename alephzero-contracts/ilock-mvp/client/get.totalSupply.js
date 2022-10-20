//
// INTERLOCK NETWORK - 
// PSP34 ACCESS NFT CLIENT LIBRARY
//
// !!!!! UNAUDITED, WARNING !!!!!
//

//
// bash calling syntax:
// node get.owner.js
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
const gasLimit = 10000 * MEG;
const storageDepositLimit = null;

async function owner() {

	try {

		// setup session
		const wsProvider = new WsProvider('wss://ws.test.azero.dev');
		const keyring = new Keyring({type: 'sr25519'});
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit getter request
		const { gasRequired, storageDeposit, result, output } =
			await contract.query['ownable::owner'](
  			OWNER_pair.address,
  			{
    				gasLimit,
    				storageDepositLimit,
  			},
		);

		// check if the call was successful
		// put stuff here to return
		if (result.isOk) {
  			console.log('Success.');
			console.log('Output:' + output.toHuman());
		} else {
  			console.error('Error', result.asErr);
		}

	} catch(error) {

		console.log(error);
	}
}

owner();
