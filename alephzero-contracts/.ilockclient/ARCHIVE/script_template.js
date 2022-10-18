//
// THIS IS A WORKBENCH .
//

// import
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { ContractPromise } = require('@polkadot/api-contract');
//const metadata = require('../ilocknft/ilockaccess/vipmembership/target/ink/metadata.json');

async function main () {

	try {

		// construct
		const wsProvider = new WsProvider('wss://ws.test.azero.dev');
		const api = await ApiPromise.create({ provider: wsProvider });
		const ADDRESS = '5CfCiRQtn2Cve6xkHzUsDTsndPqntVy2JsubDFkBwtuquZRs';
		const CONTRACT = '5Ci4xuG3bxhYmT7AmBDrGpFeB1JRwqaAsg1AxjhQikUBAFdR';

		// construct contract object
		const contract = new ContractPromise(api, metadata, CONTRACT);

		// do something
		console.log(api.genesisHash.toHex());

		// the amount required to create a new account
		console.log(api.consts.balances.existentialDeposit.toNumber());

		// get last timestamp on testnet
		const now = await api.query.timestamp.now();

		// get account balance and nonce
		const { nonce, data: balance } = await api.query.system.account(ADDRESS);

		console.log(`${now}: balance of ${balance.free} and a nonce of ${nonce}`);

		const chain = await api.rpc.system.chain();
		let count = 0;

		// Subscribe to the new headers
		/*const unsubHeads = await api.rpc.chain.subscribeNewHeads((lastHeader) => {
			console.log(`${chain}: last block #${lastHeader.number} has hash ${lastHeader.hash}`);

  			if (++count === 10) {
    				unsubHeads();
  			}
		});*/
		// Subscribe to balance changes for our account
		const unsub = await api.query.system.account(ADDRESS, ({ nonce, data: balance }) => {
  			console.log(
			`free balance is ${balance.free} with ${balance.reserved} reserved and a nonce of ${nonce}`);
		});


		// Retrieve the hash & size of the entry as stored on-chain
		const [entryHash, entrySize] = await Promise.all([
			api.query.system.account.hash(CONTRACT),
			api.query.system.account.size(CONTRACT)
		]);

		// Output the info
		console.log(`The current size is ${entrySize} bytes with a hash of ${entryHash}`);

		//const callValue = await contract.query.test();

		// print supply
		//console.log(`NFT supply is ${callValue[1]}.`);

	} catch(error) {

	console.log(error);

	}
}

main().then(() => console.log('completed'))
