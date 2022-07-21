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
	const ENTITYhash = prompt("Please enter the ENTITY hash: ");

	// find ENTITY address
	const [pdaENTITY, bumpENTITY] = await deriveAddress(toUTF8Array(ENTITYhash));
	console.log(`. Operator MAIN pda:\t${pdaENTITY.toBase58()} found after ${256 - bumpENTITY} tries`);

	// get ENTITY data
	const ENTITY = await getENTITYdata(pdaENTITY);
	
	// state intention
	console.log(`\nENTITY STAKEs:\n`);
	
	// get USER flags
	var ENTITYflags = unpackFlags(ENTITY.flags);

	// print USER data
	console.log(`| USER`)
	console.log(`| ADDRESS: ----- ${pdaENTITY.toBase58()}`);
	console.log(`| HUNTER: ------ ${ENTITY.hunter}`);
	console.log(`| STAKEPOS: ---- ${ENTITY.stakepos}`);
	console.log(`| STAKENEG: ---- ${ENTITY.stakeneg}`);
	console.log(`| STAKERS: ----- ${ENTITY.stakers}`);
	console.log(`| TIMESTAMP: --- ${ENTITY.timestamp}`);
	process.stdout.write(`| FLAGS: ------- `);
	process.stdout.write(`[ `);
	for (var index = 0; index < 4; index++) {
		process.stdout.write(`${ENTITYflags[index]} `);
	}
	process.stdout.write(`| `);
	for (var index = 4; index < 8; index++) {
		process.stdout.write(`${ENTITYflags[index]} `);
	}
	process.stdout.write(`| `);
	for (var index = 8; index < 12; index++) {
		process.stdout.write(`${ENTITYflags[index]} `);
	}
	process.stdout.write(`| `);
	for (var index = 12; index < 16; index++) {
		process.stdout.write(`${ENTITYflags[index]} `);
	}
	process.stdout.write(`]`);
		process.stdout.write(`\n\n`);

	// get PIECE accounts with operator Key in operator field
	const STAKEs = await getSTAKEs(ENTITYhash);

	// state intention
	console.log(`STAKEs:\n`);
	

	// cycle through all pieces
	for (var countSTAKE = 1; countSTAKE <= STAKEs.length; countSTAKE++) {

		// get STAKE data
		var STAKE = await getSTAKEdata(STAKEs[countSTAKE].pubkey);

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

ListEntityStakes();



