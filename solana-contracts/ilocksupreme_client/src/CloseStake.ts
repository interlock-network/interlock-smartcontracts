/****************************************************************
 * ILOCKsupreme client CloseStake				*	
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
	getUSERdata,
	newURLhash,
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

const CloseStake = async () => {
	
	try {
	
	// setup
	await establishConnection();
	await establishOperator();
	await checkProgram();
	
	// get operator ID
	const programID = "InterlockSupremeAccount";

	// get ENTITY address
	const ENTITYurl = prompt("Please enter the ENTITY URL: ");
	const ENTITYhash = newURLhash(ENTITYurl);

	// find GLOBAL address
	const [pdaGLOBAL, bumpGLOBAL] = await deriveAddress(toUTF8Array(programID));
	console.log(`. GLOBAL pda:\t\t${pdaGLOBAL.toBase58()} found after ${256 - bumpGLOBAL} tries`);

	// find ENTITY address
	const [pdaENTITY, bumpENTITY] = await deriveAddress(toUTF8Array(ENTITYhash.toString()).slice(0,32));
	console.log(`. ENTITY pda:\t\t${pdaENTITY.toBase58()} found after ${256 - bumpENTITY} tries`);

	// find USER address
	var count = new Uint16Array(1);
	count[0] = 1;	// in production, this is always 0
	const pdaUSERseed = createSeed(ownerKEY.publicKey, count);
	const [pdaUSER, bumpUSER] = await deriveAddress(pdaUSERseed);
	console.log(`. USER pda:\t\t${pdaUSER.toBase58()} found after ${256 - bumpUSER} tries`);

	// get USER data
	const USER = await getUSERdata(pdaUSER);

	// set new STAKE count
	var STAKEnumber = new Uint16Array(1);
	STAKEnumber[0] = parseInt(prompt(`From the STAKE list, please enter STAKE # that you wish to resolve: `));

	// get STAKE address
	const pdaSTAKEseed = createSeed(pdaUSER, STAKEnumber);
	const [pdaSTAKE, bumpSTAKE] = await deriveAddress(pdaSTAKEseed);
	console.log(`. STAKE pda:\t\t${pdaSTAKE.toBase58()} found after ${256 - bumpSTAKE} tries`);

	// get final STAKE address
	var endSTAKE = new Uint16Array(1);
	endSTAKE[0] = USER.count;
	const pdaSTAKEendseed = createSeed(pdaUSER, endSTAKE);
	const [pdaSTAKEend, bumpSTAKEend] = await deriveAddress(pdaSTAKEendseed);
	console.log(`. End STAKE pda:\t\t${pdaSTAKE.toBase58()} found after ${256 - bumpSTAKE} tries`);

	// setup instruction data
	const ixDATA = [6]
		.concat(pdaSTAKEseed);

	// prepare transaction
	const CloseSTAKEtx = new Transaction().add(
		new TransactionInstruction({
			keys: [
				{ pubkey: ownerKEY.publicKey, isSigner: true, isWritable: true, },
				{ pubkey: pdaGLOBAL, isSigner: false, isWritable: true, },
				{ pubkey: pdaUSER, isSigner: false, isWritable: true, },
				{ pubkey: pdaSTAKE, isSigner: false, isWritable: true, },
				{ pubkey: pdaSTAKEend, isSigner: false, isWritable: true, },
				{ pubkey: pdaENTITY, isSigner: false, isWritable: true, },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
			],
			data: Buffer.from(new Uint8Array(ixDATA)),
			programId: ilocksupremeID,
		})
	);
		
	// send transaction
	console.log(`txhash: ${await sendAndConfirmTransaction(connection, CloseSTAKEtx, [ownerKEY], )}`);
	
	// confirmation
	console.log(`\n* Successfully closed STAKE account for '${pdaSTAKE.toBase58()}'!\n`);

	} catch {

	console.log(Error);

	}
};

CloseStake();

