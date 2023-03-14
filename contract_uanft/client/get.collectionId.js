//
// INTERLOCK NETWORK - 
// PSP34 ACCESS NFT CLIENT LIBRARY
//

//
// access_selectors:
// 'VIP_MEMBERSHIP'
// 'BOUNCER_LICENSE'
//
// bash calling syntax:
// node get.collectionId.js <access_selector>
//

// imports
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');
require('dotenv').config();

// constants
const metadata_VIPMEMBERSHIP = require('../vipmembership/target/ink/metadata.json');
const metadata_BOUNCERLICENSE = require('../bouncerlicense/target/ink/metadata.json');
const contract_VIPMEMBERSHIP = process.env.CONTRACT_VIPMEMBERSHIP;
const contract_BOUNCERLICENSE = process.env.CONTRACT_BOUNCERLICENSE;
const OWNER_mnemonic = process.env.OWNER_MNEMONIC;


async function collectionId(access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const wsProvider = new WsProvider('wss://ws.test.azero.dev');
		const keyring = new Keyring({type: 'sr25519'});
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit getter request
		const { gasRequired, storageDeposit, result, output } =
			await contract.query['psp34::collectionId'](
  			OWNER_pair.address, {});

		// check if the call was successful
		if (result.isOk) {
			console.log('Result: ' + output);
			process.exit();
		} else {
  			console.error('Error: ', result.asErr);
			process.exit();
		}

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

collectionId(process.argv[2]).then(() => process.exit());
