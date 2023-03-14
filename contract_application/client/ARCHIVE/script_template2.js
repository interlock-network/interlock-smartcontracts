//
// THIS IS A WORKBENCH .
//


const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');
const metadata = require('../vipmembership/target/ink/metadata.json');

async function main () {

	try {

		// setup session
		const wsProvider = new WsProvider('wss://ws.test.azero.dev');
		const api = await ApiPromise.create({ provider: wsProvider });
		const ADDRESS = '5CfCiRQtn2Cve6xkHzUsDTsndPqntVy2JsubDFkBwtuquZRs';
		const CONTRACT = '5HWajiThA41ud6JqdXBnTu7EraJiepjRFAqtTuPP2AF4SfZP';
		const keyring = new Keyring({type: 'sr25519'});
		//const ALICE = keyring.addFromUri('//Alice', { name: 'Alice default' });
		//const ALICEpair = keyring.getPair(ALICE.address);
		const MNEMONIC = 'fiber amused more summer huge height eyebrow mean roof motion buffalo small';
		const OWNERpair = keyring.addFromUri(MNEMONIC);
		const contract = new ContractPromise(api, metadata, CONTRACT);


		const gasLimit = 100000*1000000;	//100000n * 1000000n
		const storageDepositLimit = null

		// THE BELOW CALL AND RESPONSE IS A WORKING DOER
		// (NOT ANY MORE, C 10/18)

		await contract.tx.mintAccessnft
  			({ storageDepositLimit, gasLimit }, ADDRESS, 'test')
  			.signAndSend(OWNERpair, result => {
    			if (result.status.isInBlock) {
      				console.log('in a block');
    			} else if (result.status.isFinalized) {
      				console.log('finalized');
    			}
  		});

		// THE BELOW CALL AND RESPONSE IS A WORKING GETTER
		// (NOT ANY MORE, C 10/18)
/*
		const { gasRequired, storageDeposit, result, output } =
			await contract.query['psp34::totalSupply'](
  			OWNERpair.address,
  			{
    				gasLimit,
    				storageDepositLimit,
  			},
		);

		if (result.isOk) {
  			// output the return value
  			console.log('Success', output.toHuman());
		} else {
  			console.error('Error', result.asErr);
		}
*/
	} catch(error) {

		console.log(error);
	}
}

main().then(() => console.log('completed'))
