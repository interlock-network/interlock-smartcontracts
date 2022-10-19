//
// INTERLOCK NETWORK - 
// PSP34 ACCESS NFT CLIENT LIBRARY
//
// !!!!! UNAUDITED, WARNING !!!!!
//

//
// access_selectors:
// 'VIP_MEMBERSHIP'
// 'BOUNCER_LICENSE'
//
// bash calling syntax:
// node call.mintAccessnft.js 

// imports
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');
require('dotenv').config();

// constants
const metadata_VIPMEMBERSHIP = require('../vipmembership/target/ink/metadata.json');
const metadata_BOUNCERLICENSE = require('../bouncerlicense/target/ink/metadata.json');
const contract_VIPMEMBERSHIP = process.env.CONTRACT_VIPMEMBERSHIP;
const contract_BOUNCERLICENSE = process.env.CONTRACT_BOUNCERLICENSE;
const OWNER_MNEMONIC = require('./.mnemonic.json');
const OWNER_mnemonic = OWNER_MNEMONIC.mnemonic;

// constants
const gasLimit = 100000;
const storageDepositLimit = null;

async function mintAccessnft(access_selector, jpegurl, recipient) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const wsProvider = new WsProvider('wss://ws.test.azero.dev');
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);

		const keyring = new Keyring({type: 'sr25519'});
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit doer transaction request
		const txhash = await contract.tx.mintAccessnft
  			({ storageDepositLimit, gasLimit }, recipient, jpegurl)
  			.signAndSend(OWNER_pair, result => {
    			if (result.status.isInBlock) {
      				console.log('in a block');
    			} else if (result.status.isFinalized) {
      				console.log('finalized');
    			}
  		});

	} catch(error) {

		console.log(error);
		process.exit();
	}
}

function checkSelector(access_selector) {
	var access_metadata;
	var access_contract;
	if (access_selector == 'VIP_MEMBERSHIP') {
		access_contract = contract_VIPMEMBERSHIP;
		access_metadata = metadata_VIPMEMBERSHIP;
	} else if (access_selector == 'BOUNCER_LICENSE') {
		access_contract = contract_BOUNCERLICENSE;
		access_metadata = metadata_BOUNCERLICENSE;
	} else {
		console.error('invalid access type selector, expecting VIP_MEMBERSHIP or BOUNCER_LICENSE');
	}
	return {access_contract, access_metadata};
}

//mintAccessnft(process.argv[2], process.argv[3], process.argv[4]);

mintAccessnft('VIP_MEMBERSHIP', 'test', '5CfCiRQtn2Cve6xkHzUsDTsndPqntVy2JsubDFkBwtuquZRs');

