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

async function main() {

	try {

		// setup session
		const wsProvider = new WsProvider('wss://ws.test.azero.dev');		
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, metadata_ILOCKMVP, contract_ILOCKMVP);
		const keyring = new Keyring({type: 'sr25519'});
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);


		// Subscribe to balance changes for our account
		const unsub = await contract.query((result) => {
			console.log(result);
		});


	} catch(error) {

		console.log(error);
	}
}

main();
