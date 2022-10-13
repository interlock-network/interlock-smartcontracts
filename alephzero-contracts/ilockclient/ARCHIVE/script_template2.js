//
// THIS IS A WORKBENCH .
//

// utility functions
import {
	balanceOf,
} from "./ilockaccess.js";
// import
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');
const metadata = require('../ilocknft/ilockaccess/vipmembership/target/ink/metadata.json');
//const wasm = require('../ilocknft/ilockaccess/vipmembership/target/ink/vipmembership.wasm');

async function main () {

	try {

		// construct
		const wsProvider = new WsProvider('wss://ws.test.azero.dev');
		const api = await ApiPromise.create({ provider: wsProvider });
		const ADDRESS = '5CfCiRQtn2Cve6xkHzUsDTsndPqntVy2JsubDFkBwtuquZRs';
		const CONTRACT = '5HkYNEx7rbSVk1iHLy637ZSJtTupzbny4ikRTyLHGn4HM2Nb';
		const keyring = new Keyring({type: 'sr25519'});
		const ALICE = keyring.addFromUri('//Alice', { name: 'Alice default' });
		const ALICEpair = keyring.getPair(ALICE.address);
		const MNEMONIC = 'fiber amused more summer huge height eyebrow mean roof motion buffalo small';
		const OWNERpair = keyring.addFromUri(MNEMONIC);
		console.log(OWNERpair.address);
		//const OWNERpair = keyring.getPair(OWNER.address);
	//	const code = new CodePromise(api, metadata, wasm);
		
		// construct contract object
		const contract = new ContractPromise(api, metadata, CONTRACT);

		const unsub1 = await api.query.system.account(OWNERpair.address, ({ nonce, data: balance }) => {
  			console.log(
			`free balance is ${balance.free} with ${balance.reserved} reserved and a nonce of ${nonce}`);
		});

		// maximum gas to be consumed for the instantiation. if limit is too small the instantiation will fail.
		const gasLimit = -1;	//100000n * 1000000n
		// a limit to how much Balance to be used to pay for the storage created by the instantiation
		// if null is passed, unlimited balance can be used
		const storageDepositLimit = null
		// used to derive contract address, 
		// use null to prevent duplicate contracts
		//const salt = new Uint8Array()
		// balance to transfer to the contract account, formerly know as "endowment". 
		// use only with payable constructors, will fail otherwise. 
		//const value = api.registry.createType('Balance', 1000)
		

		// THE BELOW CALL AND RESPONSE IS A WORKING DOER

		await contract.tx.mintVipmembership
  			({ storageDepositLimit, gasLimit }, ADDRESS, 'jpeg')
  			.signAndSend(OWNERpair, result => {
    			if (result.status.isInBlock) {
      				console.log('in a block');
    			} else if (result.status.isFinalized) {
      				console.log('finalized');
    			}
  		});


		// THE BELOW CALL AND RESPONSE IS A WORKING GETTER

		// (We perform the send from an account, here using Alice's address)
		const { gasRequired, storageDeposit, result, output } =
			await contract.query['psp34::balanceOf'](
  			OWNERpair.address,
  			{
    				gasLimit,
    				storageDepositLimit,
  			},
			ADDRESS
		);

		// check if the call was successful
		if (result.isOk) {
  			// output the return value
  			console.log('Success', output.toHuman());
		} else {
  			console.error('Error', result.asErr);
		}

	} catch(error) {

		console.log(error);

	}
}

main().then(() => console.log('completed'))
