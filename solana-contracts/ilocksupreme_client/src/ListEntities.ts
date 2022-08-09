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
		var ENTITYflags = unpackFlags(ENTITY.flags);

		// print STAKE data
		console.log(`\t| ADDRESS: ----- ${ENTITYs[countENTITY].pubkey}`);
		console.log(`\t| HUNTER: ------ ${ENTITY.hunter}`);
		console.log(`\t| STAKEPOS: ---- ${ENTITY.stakepos}`);
		console.log(`\t| STAKENEG: ---- ${ENTITY.stakeneg}`);
		console.log(`\t| STAKERS: ----- ${ENTITY.stakers}`);
		console.log(`\t| TIMESTAMP: --- ${ENTITY.timestamp}`);
		process.stdout.write(`\t| FLAGS: ------- `);
		for (var index = 0; index < 16; index++) {
			process.stdout.write(`${ENTITYflags[index]}  `);
		}
		console.log("\n\t                 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15") 
		process.stdout.write(`\n\n`);
	}

	} catch(error: any) {
		console.log(error);
	}
};

ListEntities();



