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

	// get vault address
	const ownerVault = prompt("Please enter your Ethereum vault address: ");

	// get ENTITY address
	const ENTITYhash = prompt("Please enter the ENTITY hash: ");

	// find GLOBAL address
	const [pdaGLOBAL, bumpGLOBAL] = await deriveAddress(toUTF8Array(programID));
	console.log(`. GLOBAL pda:\t\t${pdaGLOBAL.toBase58()} found after ${256 - bumpGLOBAL} tries`);

	// find ENTITY address
	const [pdaENTITY, bumpENTITY] = await deriveAddress(toUTF8Array(ENTITYhash));
	console.log(`. ENTITY pda:\t\t${pdaENTITY.toBase58()} found after ${256 - bumpENTITY} tries`);

	// find USER address
	const [pdaUSER, bumpUSER] = await deriveAddress(toUTF8Array(ownerVault));
	console.log(`. USER pda:\t\t${pdaUSER.toBase58()} found after ${256 - bumpUSER} tries`);

	// set new STAKE count
	var USER = await getUSERdata(pdaUSER);
	var countSTAKE = new Uint16Array(1);
	countSTAKE[0] = USER.count + 1;
	console.log(`. This will be STAKE number ${countSTAKE[0]}.`);

	// get valence
	var valence = new Uint8Array(1);
	valence = prompt("Please enter '1' if this entity is good, or '0' if it is bad: ");

	// get STAKE address
	const pdaSTAKEseed = createSeed(pdaUSER, countSTAKE);
	const [pdaSTAKE, bumpSTAKE] = await deriveAddress(pdaSTAKEseed);
	console.log(`. New STAKE pda:\t\t${pdaSTAKE.toBase58()} found after ${256 - bumpSTAKE} tries`);

	// get final STAKE address
	var endSTAKE = new Uint16Array(1);
	endSTAKE[0] = USER.count;
	const pdaSTAKEendseed = createSeed(pdaUSER, endSTAKE);
	const [pdaSTAKEend, bumpSTAKEend] = await deriveAddress(pdaSTAKEendseed);
	console.log(`. New STAKE pda:\t\t${pdaSTAKE.toBase58()} found after ${256 - bumpSTAKE} tries`);

	// get fill amount
	const amount = prompt("Please enter the amount you wish to stake: ");
	
	// setup instruction data
	const ixDATA = [6]
		.concat(toUTF8Array(ENTITYhash));

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

