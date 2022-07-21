/****************************************************************
 * Fracpay client ListUsers
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
	getENTITYdata,
	getSTAKEdata,
	getUSERdata,
	toUTF8Array,
	getUSERs,
} from "./utils";

/****************************************************************
 * main								
 ****************************************************************/

const ListUsers = async () => {
	
	try {
	
	// setup
	await establishConnection();
	await establishOperator();
	await checkProgram();

	// get USER flags
	var USERflags = unpackFlags(USER.flags);

	// get PIECE accounts with operator Key in operator field
	const USERs = await getUSERs();

	// state intention
	console.log(`USERs:\n`);
	

	// cycle through all pieces
	for (var countUSER = 0; countUSER <= USERs.length; countUSER++) {

		// get STAKE data
		var USER = await getUSERdata(USERs[countUSER].pubkey);

		// get flags
		var flags = unpackFlags(USER.flags);

		// print STAKE data
		console.log(`| ADDRESS: ----- ${USERs[countUSER].pubkey}`);
		console.log(`| SUCCESS: ----- ${USER.success}`);
		console.log(`| FAIL: -------- ${USER.fail}`);
		console.log(`| OWNER: ------- ${USER.owner}`);
		console.log(`| VAULT: ------- ${USER.vault}`);
		console.log(`| BALANCE: ----- ${USER.balance}`);
		console.log(`| REWARDS: ----- ${USER.rewards}`);
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

ListUsers();



