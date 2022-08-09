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
	getSTAKEdata,
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

	// find GLOBAL address
	const [pdaGLOBAL, bumpGLOBAL] = await deriveAddress(toUTF8Array(programID));
	console.log(`. GLOBAL pda:\t\t${pdaGLOBAL.toBase58()} found after ${256 - bumpGLOBAL} tries`);

	// find USER address
	var count = new Uint16Array(1);
	count[0] = 1;	// in production, this is always 0
	const pdaUSERseed = createSeed(ownerKEY.publicKey, count);
	const [pdaUSER, bumpUSER] = await deriveAddress(pdaUSERseed);
	console.log(`. USER pda:\t\t${pdaUSER.toBase58()} found after ${256 - bumpUSER} tries`);

	// set new STAKE count
	var STAKEnumber = new Uint16Array(1);
	STAKEnumber[0] = parseInt(prompt(`From the STAKE list, please enter STAKE # that you wish to resolve: `));

	// get STAKE address
	const pdaSTAKEseed = createSeed(pdaUSER, STAKEnumber);
	const [pdaSTAKE, bumpSTAKE] = await deriveAddress(pdaSTAKEseed);
	console.log(`. STAKE pda:\t\t${pdaSTAKE.toBase58()} found after ${256 - bumpSTAKE} tries`);

	// find ENTITYhash
	var STAKE = await getSTAKEdata(pdaSTAKE);
	const ENTITYhash = STAKE.entity.toString();

	// find ENTITY address
	const [pdaENTITY, bumpENTITY] = await deriveAddress(toUTF8Array(ENTITYhash.toString()).slice(0,32));
	console.log(`. ENTITY pda:\t\t${pdaENTITY.toBase58()} found after ${256 - bumpENTITY} tries`);

	// setup instruction data
	const ixDATA = [12]
		.concat(pdaSTAKEseed);

	// prepare transaction
	const ResolveSTAKEtx = new Transaction().add(
		new TransactionInstruction({
			keys: [
				{ pubkey: ownerKEY.publicKey, isSigner: true, isWritable: true, },
				{ pubkey: pdaGLOBAL, isSigner: false, isWritable: true, },
				{ pubkey: pdaUSER, isSigner: false, isWritable: true, },
				{ pubkey: pdaSTAKE, isSigner: false, isWritable: true, },
				{ pubkey: pdaENTITY, isSigner: false, isWritable: true, },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
			],
			data: Buffer.from(new Uint8Array(ixDATA)),
			programId: ilocksupremeID,
		})
	);
		
	// send transaction
	console.log(`txhash: ${await sendAndConfirmTransaction(connection, ResolveSTAKEtx, [ownerKEY], )}`);
	
	// confirmation
	console.log(`\n* Successfully resolved STAKE account '${pdaSTAKE.toBase58()}'!\n`);

	} catch(error: any) {

	console.log(error);

	}
};

CreateStake();

