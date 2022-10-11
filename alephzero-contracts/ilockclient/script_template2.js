//
// THIS IS A WORKBENCH .
//

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
		const CONTRACT = '5G7HRZviWs3XAR18mysFugY1RrVXYCR3nvVwjZmHBSb1z6J6';
		const keyring = new Keyring({type: 'sr25519'});
		const ALICE = keyring.addFromUri('//Alice', { name: 'Alice default' });
		const ALICEpair = keyring.getPair(ALICE.address);
	//	const code = new CodePromise(api, metadata, wasm);
		
		// construct contract object
		const contract = new ContractPromise(api, metadata, CONTRACT);

		const unsub1 = await api.query.system.account(ALICE.address, ({ nonce, data: balance }) => {
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


		await contract.tx
  			.test({ storageDepositLimit, gasLimit })
  			.signAndSend(ALICEpair, result => {
    			if (result.status.isInBlock) {
      				console.log('in a block');
    			} else if (result.status.isFinalized) {
      				console.log('finalized');
    			}
  		});

		// (We perform the send from an account, here using Alice's address)
		const { gasRequired, storageDeposit, result, output } = await contract.query.test(
  			ALICEpair.address,
  			{
    				gasLimit,
    				storageDepositLimit,
  			}
		);

		// check if the call was successful
		if (result.isOk) {
  			// output the return value
  			console.log('Success', output.toHuman());
		} else {
  			console.error('Error', result.asErr);
		}


		/*const tx = code.tx.new({ gasLimit, storageDepositLimit })

		let address;

		const unsub2 = await tx.signAndSend(alicePair, ({ contract, status }) => {
  			if (status.isInBlock || status.isFinalized) {
    				address = contract.address.toString();
    				unsub2();
  			}
		});*/

		//const callValue = await contract.query.test(ALICE.address, {gasLimit: -1});

		// print supply
		//console.log(`test is ${callValue.toHuman()}.`);

	} catch(error) {

		console.log(error);

	}
}

main().then(() => console.log('completed'))
