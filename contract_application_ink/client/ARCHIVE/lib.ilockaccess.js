//
// INTERLOCK NETWORK - 
// PSP34 ACCESS CLIENT LIBRARY - BOUNCER LICENSE
//
// !!!!! INCOMPLETE AND UNAUDITED, WARNING !!!!!
//

//
// INTERFACE
//
// doers
//
// mintAccessnft	(recipient: string, jpegurl: string) -> ()
// renounceOwnership	() -> ()
// transferOwnership	(newowner: string) -> ()
// approve		(operator: string, id: u16, approved: bool) -> ()
// transfer		(to: string, id: u16, data: bytes) -> ()
// setAuthenticated	(id: u16) -> ()
// setNotAuthenticated	(id: u16) -> ()
// upgradeContract	(codehash: hash) -> ()
//
// getters
//
// owner		() -> address
// allowance		(owner: string, operator: string, id: u16) -> bool
// balanceOf		(address: string) -> integer
// collectionId		() -> bytes
// ownerOf		(id: u16) -> address
// totalSupply		() -> integer
// getAttribute		(id: u16, key: string) -> string | bytes
//

// 
// TODO
// . determine gas limit
// . depending on py2js needs, create returns for getters
//

// imports
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');
//require('dotenv').config();

// constants
const metadata_VIPMEMBERSHIP = require('../ilocknft/ilockaccess/vipmembership/target/ink/metadata.json');
const metadata_BOUNCERLICENSE = require('../ilocknft/ilockaccess/bouncerlicense/target/ink/metadata.json');
const contract_VIPMEMBERSHIP = process.env.CONTRACT_VIPMEMBERSHIP;
const contract_BOUNCERLICENSE = process.env.CONTRACT_BOUNCERLICENSE;
const OWNER_MNEMONIC = require('./.mnemonic.json');
const OWNER_mnemonic = OWNER_MNEMONIC.mnemonic;

const keyring = new Keyring({type: 'sr25519'});
const wsProvider = new WsProvider('wss://ws.test.azero.dev');

// constants
const gasLimit = 100000;
const storageDepositLimit = null;

/////// doers //////////////////////////////////////////////////


function helloWorld() {
	console.log("hello world");
}

async function mintAccessnft(recipient, jpegurl, access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
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
	}
}

async function renounceOwnership(access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit doer transaction request
		const txhash = await contract.tx['ownable::renounceOwnership']
  			({ storageDepositLimit, gasLimit })
  			.signAndSend(OWNER_pair, result => {
    			if (result.status.isInBlock) {
      				console.log('in a block');
    			} else if (result.status.isFinalized) {
      				console.log('finalized');
    			}
  		});

	} catch(error) {

		console.log(error);
	}
}

async function transferOwnership(newowner, access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit doer transaction request
		const txhash = await contract.tx['ownable::transferOwnership']
  			({ storageDepositLimit, gasLimit }, newowner)
  			.signAndSend(OWNER_pair, result => {
    			if (result.status.isInBlock) {
      				console.log('in a block');
    			} else if (result.status.isFinalized) {
      				console.log('finalized');
    			}
  		});

	} catch(error) {

		console.log(error);
	}
}

async function approve(operator, id, approved, access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit doer transaction request
		const txhash = await contract.tx['psp34::approve']
  			({ storageDepositLimit, gasLimit }, operator, {u16: id}, approved)
  			.signAndSend(OWNER_pair, result => {
    			if (result.status.isInBlock) {
      				console.log('in a block');
    			} else if (result.status.isFinalized) {
      				console.log('finalized');
    			}
  		});

	} catch(error) {

		console.log(error);
	}
}

async function transfer(to, id, data, access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit doer transaction request
		const txhash = await contract.tx['psp34::transfer']
  			({ storageDepositLimit, gasLimit }, to, {u16: id}, data)
  			.signAndSend(OWNER_pair, result => {
    			if (result.status.isInBlock) {
      				console.log('in a block');
    			} else if (result.status.isFinalized) {
      				console.log('finalized');
    			}
  		});

	} catch(error) {

		console.log(error);
	}
}

async function setAuthenticated(id, access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit doer transaction request
		const txhash = await contract.tx.setAuthenticated
  			({ storageDepositLimit, gasLimit }, {u16: id})
  			.signAndSend(OWNER_pair, result => {
    			if (result.status.isInBlock) {
      				console.log('in a block');
    			} else if (result.status.isFinalized) {
      				console.log('finalized');
    			}
  		});

	} catch(error) {

		console.log(error);
	}
}


async function setNotAuthenticated(id, access_selector) {

	try {

		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit doer transaction request
		const txhash = await contract.tx.setNotAuthenticated
  			({ storageDepositLimit, gasLimit }, {u16: id})
  			.signAndSend(OWNER_pair, result => {
    			if (result.status.isInBlock) {
      				console.log('in a block');
    			} else if (result.status.isFinalized) {
      				console.log('finalized');
    			}
  		});


	} catch(error) {

		console.log(error);
	}
}

async function upgradeContract(codehash, access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit doer transaction request
		const txhash = await contract.tx.upgradeContract
  			({ storageDepositLimit, gasLimit }, codehash)
  			.signAndSend(OWNER_pair, result => {
    			if (result.status.isInBlock) {
      				console.log('in a block');
    			} else if (result.status.isFinalized) {
      				console.log('finalized');
    			}
  		});

	} catch(error) {

		console.log(error);
	}
}

/////// getters ////////////////////////////////////////////////

async function owner(access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
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

async function allowance(owner, operator, id, access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit getter request
		const { gasRequired, storageDeposit, result, output } =
			await contract.query['psp34::allowance'](
  			OWNER_pair.address,
  			{
    				gasLimit,
    				storageDepositLimit,
  			},
			owner,
			operator,
			{u16: id}
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

async function balanceOf(address, access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);


		// submit getter request
		const { gasRequired, storageDeposit, result, output } =
			await contract.query['psp34::balanceOf'](
  			OWNER_pair.address,
  			{
    				gasLimit,
    				storageDepositLimit,
  			},
			address
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


async function collectionId(access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit getter request
		const { gasRequired, storageDeposit, result, output } =
			await contract.query['psp34::collectionId'](
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
			console.log('Output:' + output);
		} else {
  			console.error('Error', result.asErr);
		}

	} catch(error) {

		console.log(error);
	}
}

async function ownerOf(id, access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// condition id
		//var ID = new Uint16Array(1);
		//ID[0] = id;

		// submit getter request
		const { gasRequired, storageDeposit, result, output } =
			await contract.query['psp34::ownerOf'](
  			OWNER_pair.address,
  			{
    				gasLimit,
    				storageDepositLimit,
  			},
			{u16: id},
		);


		// check if the call was successful
		// put stuff here to return
		if (result.isOk) {
  			console.log('Success.');
			console.log('Output:' + output);
		} else {
  			console.error('Error', result.asErr);
		}

	} catch(error) {

		console.log(error);
	}
}

async function totalSupply(access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit getter request
		const { gasRequired, storageDeposit, result, output } =
			await contract.query['psp34::totalSupply'](
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

async function getAttribute(id, key, access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit getter request
		const { gasRequired, storageDeposit, result, output } =
			await contract.query['psp34Metadata::getAttribute'](
  			OWNER_pair.address,
  			{
    				gasLimit,
    				storageDepositLimit,
  			},
			{u16: id},
			key
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

/////// helpers ////////////////////////////////////////////////

function checkSelector(access_selector) {
	var access_metadata;
	var access_contract;
	if (access_selector == 'VIPMEM') {
		access_contract = contract_VIPMEMBERSHIP;
		access_metadata = metadata_VIPMEMBERSHIP;
	} else if (access_selector == 'BOUNCE') {
		access_contract = contract_BOUNCERLICENSE;
		access_metadata = metadata_BOUNCERLICENSE;
	} else {
		console.error('invalid access type selector, expecting VIPMEM or BOUNCE');
	}
	return {access_contract, access_metadata};
}

