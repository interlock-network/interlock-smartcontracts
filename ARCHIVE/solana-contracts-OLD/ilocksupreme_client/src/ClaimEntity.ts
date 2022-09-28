/****************************************************************
 * ILOCKsupreme client CreateStake				*	
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
	PublicKey,

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

const CreateStake = async () => {
	
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

	// get fill amount
	const amount = prompt("Please enter the amount you wish to stake: ");
	
	// setup instruction data
	const ixDATA = [11, bumpSTAKE]
		.concat(pdaSTAKEseed)
		.concat(new BN(amount).toArray("le", 16))
		.concat([valence[0]]);

	// prepare transaction
	const CreateUSERtx = new Transaction().add(
		new TransactionInstruction({
			keys: [
				{ pubkey: ownerKEY.publicKey, isSigner: true, isWritable: true, },
				{ pubkey: pdaGLOBAL, isSigner: false, isWritable: true, },
				{ pubkey: pdaUSER, isSigner: false, isWritable: true, },
				{ pubkey: pdaSTAKE, isSigner: false, isWritable: true, },
				{ pubkey: pdaENTITY, isSigner: false, isWritable: true, },
				{ pubkey: new PublicKey(ENTITYhash), isSigner: false, isWritable: false, },
				{ pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false, },
				{ pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false, },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
			],
			data: Buffer.from(new Uint8Array(ixDATA)),
			programId: ilocksupremeID,
		})
	);
		
	// send transaction
	console.log(`txhash: ${await sendAndConfirmTransaction(connection, CreateUSERtx, [ownerKEY], )}`);
	
	// confirmation
	console.log(`\n* Successfully created new STAKE account '${pdaSTAKE.toBase58()}'!\n`);

	} catch(error: any) {

	console.log(error);

	}
};

CreateStake();

