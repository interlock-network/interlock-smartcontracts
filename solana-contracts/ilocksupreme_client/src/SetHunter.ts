/****************************************************************
 * ILOCKsupreme client SetHunter				*	
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
	SYSVAR_CLOCK_PUBKEY,
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
	getUSERdata,
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

const SetHunter = async () => {
	
	try {
	
	// setup
	await establishConnection();
	await establishOperator();
	await checkProgram();
	
	// get operator ID
	const programID = "InterlockSupremeAccount";

	// find GLOBAL address
	const [pdaGLOBAL, bumpGLOBAL] = await deriveAddress(toUTF8Array(programID));
	console.log(`. GLOBAL pda:\t\t${pdaGLOBAL.toBase58()} found after ${256 - bumpGLOBAL} tries`);

	// find USER address
	var count = new Uint16Array(1);
	count[0] = 1;	// in production, this is always 0
	const pdaUSERseed = createSeed(ownerKEY.publicKey, count);
	const [pdaUSER, bumpUSER] = await deriveAddress(pdaUSERseed);
	console.log(`. USER pda:\t\t${pdaUSER.toBase58()} found after ${256 - bumpUSER} tries`);

	// get valence
	var status = new Uint8Array(1);
	status = prompt("Please enter '1' to give USER hunter privileges, or '0' to revoke: ");

	// setup instruction data
	const ixDATA = [8, status[0]];

	// prepare transaction
	const SetHuntertx = new Transaction().add(
		new TransactionInstruction({
			keys: [
				{ pubkey: ownerKEY.publicKey, isSigner: true, isWritable: true, },
				{ pubkey: pdaGLOBAL, isSigner: false, isWritable: true, },
				{ pubkey: pdaUSER, isSigner: false, isWritable: true, },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
			],
			data: Buffer.from(new Uint8Array(ixDATA)),
			programId: ilocksupremeID,
		})
	);
		
	// send transaction
	console.log(`txhash: ${await sendAndConfirmTransaction(connection, SetHuntertx, [ownerKEY], )}`);
	
	// confirmation
	console.log(`\n* Successfully set hunter status for USER '${pdaUSER.toBase58()}'!\n`);

	} catch(error: any) {

	console.log(error);

	}
};

SetHunter();

