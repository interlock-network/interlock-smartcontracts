// import
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main () {

	try {

	// construct
	const wsProvider = new WsProvider('wss://ws.test.azero.dev');
	const api = await ApiPromise.create({ provider: wsProvider });
	const ADDRESS = '5CfCiRQtn2Cve6xkHzUsDTsndPqntVy2JsubDFkBwtuquZRs';
	const CONTRACT = '5Ci4xuG3bxhYmT7AmBDrGpFeB1JRwqaAsg1AxjhQikUBAFdR';

	// do something
	console.log(api.genesisHash.toHex());

	// the amount required to create a new account
	console.log(api.consts.balances.existentialDeposit.toNumber());

	// get last timestamp on testnet
	const now = await api.query.timestamp.now();

	// get account balance and nonce
	const { nonce, data: balance } = await api.query.system.account(ADDRESS);

	console.log(`${now}: balance of ${balance.free} and a nonce of ${nonce}`);

	} catch(error) {

	console.log(error);

	}
}

main().then(() => console.log('completed'))
