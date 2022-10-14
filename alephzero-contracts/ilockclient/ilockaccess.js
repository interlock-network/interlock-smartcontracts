//
// INTERLOCK NETWORK - 
// PSP34 ACCESS CLIENT LIBRARY - BOUNCER LICENSE
//
// !!!!! INCOMPLETE AND UNAUDITED, WARNING !!!!!
//

// imports
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');

// constants
const metadata_VIPMEMBERSHIP = require('../ilocknft/ilockaccess/vipmembership/target/ink/metadata.json');
const metadata_BOUNCERLICENSE = require('../ilocknft/ilockaccess/bouncerlicense/target/ink/metadata.json');
const contract_VIPMEMBERSHIP = '5HkYNEx7rbSVk1iHLy637ZSJtTupzbny4ikRTyLHGn4HM2Nb';
const contract_BOUNCERLICENSE = '5HkYNEx7rbSVk1iHLy637ZSJtTupzbny4ikRTyLHGn4HM2Nb';
const OWNER_mnemonic = 'fiber amused more summer huge height eyebrow mean roof motion buffalo small';

const ADDRESS1 = '5CfCiRQtn2Cve6xkHzUsDTsndPqntVy2JsubDFkBwtuquZRs';
const ADDRESS2 = '5CqfsS4Le7Si6osL5ysrrX3Mk8F94nSvhiQ6EGYAx3REHtAX';

const keyring = new Keyring({type: 'sr25519'});
const wsProvider = new WsProvider('wss://ws.test.azero.dev');

// constants
const gasLimit = -1;
const storageDepositLimit = null;

/////// doers //////////////////////////////////////////////////

async function mintAccessNFT(recipient, jpeg_url, access_selector) {

	try {
		// choose which contract to access based off access_selector
		const {access_contract, access_metadata} = checkSelector(access_selector);

		// setup session
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, access_metadata, access_contract);
		const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

		// submit doer transaction request
		const txhash = await contract.tx.mintAccessNFT
  			({ storageDepositLimit, gasLimit }, recipient, jpeg_url)
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

setAuthenticated(22, 'VIPMEM').then(() => console.log('completed'))

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

async function getAttribute(access_selector) {

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
			id,
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

