/****************************************************************
 * Fracpay client ListUserStakes
 ****************************************************************/

/****************************************************************
 * imports							
 ****************************************************************/

const prompt = require("prompt-sync")({sigint: true});

import {
	unpackFlags,
	deriveAddress,
	createSeed,
	establishConnection,
	establishOperator,
	checkProgram,
	getUSERdata,
	getSTAKEdata,
	toUTF8Array,
} from "./utils";

/****************************************************************
 * main								
 ****************************************************************/

const ListUserStakes = async () => {
	
	try {
	
	// setup
	await establishConnection();
	await establishOperator();
	await checkProgram();

	// get vault address
	const ownerVault = prompt("Please enter your Ethereum vault address: ");

	// find MAIN address
	const [pdaUSER, bumpUSER] = await deriveAddress(toUTF8Array(ownerVault));
	console.log(`. Operator MAIN pda:\t${pdaUSER.toBase58()} found after ${256 - bumpUSER} tries`);

	// get MAIN data
	const USER = await getUSERdata(pdaUSER);
	
	// state intention
	console.log(`\nUSER STAKEs:\n`);
	
	// get USER flags
	var USERflags = unpackFlags(USER.flags);

	// print USER data
	console.log(`| USER`)
	console.log(`| ADDRESS: ----- ${pdaUSER.toBase58()}`);
	console.log(`| SUCCESS: ----- ${USER.success}`);
	console.log(`| FAIL: -------- ${USER.fail}`);
	console.log(`| OWNER: ------- ${USER.owner}`);
	console.log(`| VAULT: ------- ${USER.vault}`);
	console.log(`| BALANCE: ----- ${USER.balance}`);
	console.log(`| REWARDS: ----- ${USER.rewards}`);
	process.stdout.write(`| FLAGS: ------- `);
	process.stdout.write(`[ `);
	for (var index = 0; index < 4; index++) {
		process.stdout.write(`${USERflags[index]} `);
	}
	process.stdout.write(`| `);
	for (var index = 4; index < 8; index++) {
		process.stdout.write(`${USERflags[index]} `);
	}
	process.stdout.write(`| `);
	for (var index = 8; index < 12; index++) {
		process.stdout.write(`${USERflags[index]} `);
	}
	process.stdout.write(`| `);
	for (var index = 12; index < 16; index++) {
		process.stdout.write(`${USERflags[index]} `);
	}
	process.stdout.write(`]`);
		process.stdout.write(`\n\n`);

	var countSTAKE = new Uint16Array(1);
	var pdaSTAKEseed;
	var pdaSTAKE;
	var bumpSTAKE;
	var STAKE;

	// cycle through all pieces
	for (countSTAKE[0] = 0; countSTAKE[0] <= USER.count; countSTAKE[0]++) {

		// find STAKE address
		pdaSTAKEseed = createSeed(pdaUSER, countSTAKE);
		[pdaSTAKE, bumpSTAKE] = await deriveAddress(pdaSTAKEseed);

		// get STAKE data
		STAKE = await getSTAKEdata(pdaSTAKE);

		// get flags
		var flags = unpackFlags(STAKE.flags);

		// print STAKE data
		console.log(`# ${countSTAKE[0]}\t| STAKE ID: ----> ${STAKE.entity}`);
		console.log(`\t| TIMESTAMP: --- ${STAKE.timestamp}`);
		console.log(`\t| ENTITY: ------ ${STAKE.entity}`);
		console.log(`\t| AMOUNT: ------ ${STAKE.amount}`);
		process.stdout.write(`\t| FLAGS: ------- `);
		process.stdout.write(`[ `);
		for (var index = 0; index < 4; index++) {
			process.stdout.write(`${flags[index]} `);
		}
		process.stdout.write(`| `);
		for (var index = 4; index < 8; index++) {
			process.stdout.write(`${flags[index]} `);
		}
		process.stdout.write(`| `);
		for (var index = 8; index < 12; index++) {
			process.stdout.write(`${flags[index]} `);
		}
		process.stdout.write(`| `);
		for (var index = 12; index < 16; index++) {
			process.stdout.write(`${flags[index]} `);
		}
		process.stdout.write(`]`);
		process.stdout.write(`\n\n`);

	}

	} catch {
		console.log(Error);
	}
};

ListUserStakes();



