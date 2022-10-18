//
// THIS IS A WORKBENCH .
//

// import
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');

// to be stored in a .env file
const keyring = new Keyring({type: 'sr25519'});
const OWNERmnemonic = 'fiber amused more summer huge height eyebrow mean roof motion buffalo small';
const OWNERpair = keyring.addFromUri(OWNERmnemonic);
const metadata_VIPMEMBERSHIP = require('../ilocknft/ilockaccess/vipmembership/target/ink/metadata.json');
//const metadata_BOUNCERLICENSE = require('../ilocknft/ilockaccess/bouncerlicense/target/ink/metadata.json');
const contract_VIPMEMBERSHIP = '5HkYNEx7rbSVk1iHLy637ZSJtTupzbny4ikRTyLHGn4HM2Nb';
const contract_BOUNCERLICENSE = '5HkYNEx7rbSVk1iHLy637ZSJtTupzbny4ikRTyLHGn4HM2Nb';

// setup

// websocket URI
const wsProvider = new WsProvider('wss://ws.test.azero.dev');

// TX constants
const gasLimit = -1;
const storageDepositLimit = null;


///// GETTERS ///////////////////////////////////////////////

async function main () {

	try {
		// instantiate objects
		const api = await ApiPromise.create({ provider: wsProvider });
		const contract = new ContractPromise(api, metadata_VIPMEMBERSHIP, contract_VIPMEMBERSHIP);

		// (We perform the send from an account, here using Alice's address)
		const { gasRequired, storageDeposit, result, output } =
			await contract.query['psp34::balanceOf'](
  			OWNERpair.address,
  			{
    				gasLimit,
    				storageDepositLimit,
  			},
			OWNERpair.address
		);

		// print and return if call successful
		if (result.isOk) {

  			// output the return value
  			console.log('Success:', output.toHuman());
			return output;
		}

	} catch(error) {

		console.log(error);
	}
}

main().then(() => console.log('completed'))

/*

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
*/
