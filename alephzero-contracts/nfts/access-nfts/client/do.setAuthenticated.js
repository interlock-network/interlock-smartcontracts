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
// node do.setAuthenticated.js <access_selector> <id>
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
const OWNER_MNEMONIC = require('./.mnemonic.json');
const OWNER_mnemonic = OWNER_MNEMONIC.mnemonic;

// constants
const MEG = 1000000;
const gasLimit = 100000 * MEG;
const storageDepositLimit = null;

async function setAuthenticated(access_selector, id) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const wsProvider = new WsProvider('wss://ws.test.azero.dev');
		const keyring = new Keyring({type: 'sr25519'});
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// perform dry run to check for errors
		const { gasRequired, storageDeposit, result, output } =
			await contract.query[method](
  			OWNER_pair.address, {}, {u16: id});

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
  			({ storageDeposit, gasRequired }, {u16: id})
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

setAuthenticated(process.argv[2], process.argv[3]);
