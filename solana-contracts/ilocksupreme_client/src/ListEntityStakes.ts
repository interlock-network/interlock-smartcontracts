/****************************************************************
 * Fracpay client ListEntityStakes
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
	getSTAKEs,
	newURLhash,
} from "./utils";

/****************************************************************
 * main								
 ****************************************************************/

const ListEntityStakes = async () => {
	
	try {
	
	// setup
	await establishConnection();
	await establishOperator();
	await checkProgram();
	
	// get ENTITY address
	const ENTITYurl = prompt("Please enter the ENTITY URL: ");
	const ENTITYhash = newURLhash(ENTITYurl);

	// find ENTITY address
	const [pdaENTITY, bumpENTITY] = await deriveAddress(toUTF8Array(ENTITYhash.toString()).slice(0,32));
	console.log(`. Operator MAIN pda:\t${pdaENTITY.toBase58()} found after ${256 - bumpENTITY} tries`);

	// get ENTITY data
	const ENTITY = await getENTITYdata(pdaENTITY);
	
	// state intention
	console.log(`\nENTITY STAKEs:\n`);
	
	// get ENTITY flags
	var ENTITYflags = unpackFlags(ENTITY.flags);

	// print ENTITY data
	console.log(`| ENTITY`)
	console.log(`| ADDRESS: ----- ${pdaENTITY.toBase58()}`);
	console.log(`| HUNTER: ------ ${ENTITY.hunter}`);
	console.log(`| STAKEPOS: ---- ${ENTITY.stakepos}`);
	console.log(`| STAKENEG: ---- ${ENTITY.stakeneg}`);
	console.log(`| STAKERS: ----- ${ENTITY.stakers}`);
	console.log(`| TIMESTAMP: --- ${ENTITY.timestamp}`);
	process.stdout.write(`| FLAGS: ------- `);
	for (var index = 0; index < 16; index++) {
		process.stdout.write(`${ENTITYflags[index]}  `);
	}
	console.log("\n                 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15") 
	process.stdout.write(`\n\n`);

	// get STAKE accounts with operator Key in operator field
	const STAKEs = await getSTAKEs(ENTITYhash.toString());

	// state intention
	console.log(`STAKEs:\n`);
	
	// cycle through all pieces
	for (var countSTAKE = 0; countSTAKE < STAKEs.length; countSTAKE++) {


		// get STAKE data
		var STAKE = await getSTAKEdata(STAKEs[countSTAKE].pubkey);

		// get flags
		var STAKEflags = unpackFlags(STAKE.flags);

		// print STAKE data
		console.log(`# ${countSTAKE}\t| STAKE ID: ---- ${STAKEs[countSTAKE].pubkey.toBase58()}`);
		console.log(`\t| TIMESTAMP: --- ${STAKE.timestamp}`);
		console.log(`\t| ENTITY: ------ ${STAKE.entity}`);
		console.log(`\t| AMOUNT: ------ ${STAKE.amount}`);
		process.stdout.write(`\t| FLAGS: ------- `);
		for (var index = 0; index < 16; index++) {
			process.stdout.write(`${STAKEflags[index]}  `);
		}
		console.log("\n\t                 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15") 
		process.stdout.write(`\n\n`);


	}

	} catch {
		console.log(Error);
	}
};

ListEntityStakes();



