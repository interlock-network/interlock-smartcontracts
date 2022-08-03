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

	// get PIECE accounts with operator Key in operator field
	const USERs = await getUSERs();

	// state intention
	console.log(`USERs:\n`);
	

	// cycle through all pieces
	for (var countUSER = 0; countUSER < USERs.length; countUSER++) {

		// get STAKE data
		var USER = await getUSERdata(USERs[countUSER].pubkey);

		// get flags
		var USERflags = unpackFlags(USER.flags);

		// print STAKE data
		console.log(`\t| ADDRESS: ----- ${USERs[countUSER].pubkey}`);
		console.log(`\t| COUNT: ------- ${USER.count}`);
		console.log(`\t| SUCCESS: ----- ${USER.success}`);
		console.log(`\t| FAIL: -------- ${USER.fail}`);
		console.log(`\t| OWNER: ------- ${USER.owner}`);
		console.log(`\t| VAULT: ------- ${USER.vault}`);
		console.log(`\t| BALANCE: ----- ${USER.balance}`);
		console.log(`\t| REWARDS: ----- ${USER.rewards}`);
		process.stdout.write(`\t| FLAGS: ------- `);
		for (var index = 0; index < 16; index++) {
			process.stdout.write(`${USERflags[index]}  `);
		}
		console.log("\n\t                 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15") 
		process.stdout.write(`\n\n`);

	}

	} catch {
		console.log(Error);
	}
};

ListUsers();



