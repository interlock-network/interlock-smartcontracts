/****************************************************************
 * Fracpay client ListEntities
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
	toUTF8Array,
	getENTITYs,
} from "./utils";

// utility constants
import {
	connection,
	ownerKEY,
	ilocksupremeID,
} from "./utils";

/****************************************************************
 * main								
 ****************************************************************/

const ListEntities = async () => {
	
	try {
	
	// setup
	await establishConnection();
	await establishOperator();
	await checkProgram();

	// state intention
	console.log(`\nENTITYs:\n`);
	
	// get PIECE accounts with operator Key in operator field
	const ENTITYs = await getENTITYs();

	// state intention
	console.log(`ENTITYs:\n`);
	

	// cycle through all pieces
	for (var countENTITY = 0; countENTITY <= ENTITYs.length; countENTITY++) {

		// get STAKE data
		var ENTITY = await getENTITYdata(ENTITYs[countENTITY].pubkey);

		// get flags
		var flags = unpackFlags(ENTITY.flags);

		// print STAKE data
		console.log(`\t| ADDRESS: ----- ${ENTITYs[countENTITY].pubkey}`);
		console.log(`\t| HUNTER: ------ ${ENTITY.hunter}`);
		console.log(`\t| STAKEPOS: ---- ${ENTITY.stakepos}`);
		console.log(`\t| STAKENEG: ---- ${ENTITY.stakeneg}`);
		console.log(`\t| STAKERS: ----- ${ENTITY.stakers}`);
		console.log(`\t| TIMESTAMP: --- ${ENTITY.timestamp}`);
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

ListEntities();



