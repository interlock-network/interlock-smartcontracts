/****************************************************************
 * ILOCKsupreme client FillUser 				*	
 ****************************************************************/

/****************************************************************
 * imports							*
 ****************************************************************/

// misc packages
const prompt = require("prompt-sync")({sigint: true});

// misc solana
import {
  	sendAndConfirmTransaction,
	Transaction,
	TransactionInstruction,
	SYSVAR_RENT_PUBKEY,
	SystemProgram,

} from "@solana/web3.js";

// utility functions
import {
	deriveAddress,
	establishConnection,
	establishOperator,
	checkProgram,
	toUTF8Array,
	createSeed,
} from "./utils";

// utility constants
import {
	connection,
	ownerKEY,
	ilocksupremeID,
} from "./utils";

const BN = require("bn.js");


/****************************************************************
 * main								*
 ****************************************************************/

const FillUser = async () => {
	
	try {
	
	// setup
	await establishConnection();
	await establishOperator();
	await checkProgram();

	// find USER address
	var count = new Uint16Array(1);
	count[0] = 1;	// in production, this is always 0
	const pdaUSERseed = createSeed(ownerKEY.publicKey, count);
	const [pdaUSER, bumpUSER] = await deriveAddress(pdaUSERseed);
	console.log(`. USER pda:\t\t${pdaUSER.toBase58()} found after ${256 - bumpUSER} tries`);
	
	// get fill amount
	const amount = prompt("Please enter the amount you wish to fill: ");

	// setup instruction data
	const ixDATA = [3].concat(new BN(amount).toArray("le", 16));

	// prepare transaction
	const FillUSERtx = new Transaction().add(
		new TransactionInstruction({
			keys: [
				{ pubkey: ownerKEY.publicKey, isSigner: true, isWritable: true, },
				{ pubkey: pdaUSER, isSigner: false, isWritable: true, },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
			],
			data: Buffer.from(new Uint8Array(ixDATA)),
			programId: ilocksupremeID,
		})
	);
		
	// send transaction
	console.log(`txhash: ${await sendAndConfirmTransaction(connection, FillUSERtx, [ownerKEY], )}`);
	
	// confirmation
	console.log(`\n* Successfully filled USER account for '${amount}'!\n`);

	} catch(error: any) {

	console.log(error);

	}
};

FillUser();

