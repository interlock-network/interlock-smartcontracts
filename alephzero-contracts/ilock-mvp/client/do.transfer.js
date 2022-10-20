//
// INTERLOCK NETWORK - 
// PSP34 ACCESS NFT CLIENT LIBRARY
//
// !!!!! UNAUDITED, WARNING !!!!!
//

//
// bash calling syntax:
// node do.transfer.js <to> <value> <data>
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

async function transfer(to, value, data) {

	try {

		// setup session
		const wsProvider = new WsProvider('wss://ws.test.azero.dev');
		const keyring = new Keyring({type: 'sr25519'});
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit doer transaction request
		const txhash = await contract.tx['psp22::transfer']
  			({ storageDepositLimit, gasLimit }, to, value, data)
  			.signAndSend(OWNER_pair, result => {
    			if (result.status.isInBlock) {
      				console.log('in a block');
    			} else if (result.status.isFinalized) {
      				console.log('finalized');
				for (const key in result.events) {
  					if (result.events.hasOwnProperty(key)) {
    						console.log(`${key}: ${result.events[key]}`);
					}
  				}

    			}
  		});

	} catch(error) {

		console.log(error);
		process.exit();
	}
}

transfer(process.argv[2], process.argv[3], process.argv[4]);

