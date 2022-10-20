//
// INTERLOCK NETWORK - 
// PSP34 ACCESS NFT CLIENT LIBRARY
//
// !!!!! UNAUDITED, WARNING !!!!!
//

//
// bash calling syntax:
// node do.transferFrom.js <from> <to> <value> <data>
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

async function transferFrom(from, to, value, data) {

	try {

		// setup session
		const wsProvider = new WsProvider('wss://ws.test.azero.dev');
		const keyring = new Keyring({type: 'sr25519'});
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit doer transaction request
		const txhash = await contract.tx['psp22::transferFrom']
  			({ storageDepositLimit, gasLimit }, from, to, value, data)
  			.signAndSend(OWNER_pair, result => {
    			if (result.status.isInBlock) {
      				console.log('in a block');
    			} else if (result.status.isFinalized) {
      				console.log('finalized');

				for (const key in result.events.data) {
  					if (result.events.hasOwnProperty(key)) {
    						//console.log(`${key}: ${result.events[key]}`);
						console.log(result.events.toHuman());
					}
				}
				//console.log(result.toHuman());
  				

    			}
  		});

	} catch(error) {

		console.log(error);
		process.exit();
	}
}

transferFrom(process.argv[2], process.argv[3], process.argv[4], process.argv[5]);

