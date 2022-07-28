/****************************************************************
 * ILOCKsupreme client UpdateGlobal				*	
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
	unpackFlags32,
	packFlags32,
	u32toBytes,
} from "./utils";

// utility constants
import {
	connection,
	ownerKEY,
	ilocksupremeID,
} from "./utils";

/****************************************************************
 * main								*
 ****************************************************************/

const UpdateGlobal = async () => {
	
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

	// value flags...high updates value
	const updateFlagsHigh = unpackFlags32(0);
	const updateFlagsLow = unpackFlags32(0);
	const updateValues = new Uint32Array(64);

	// values and their flags, 64

	// 0: entity total stake threshold
	updateFlagsHigh[0] = 1;
	updateValues[0] = 555;

	// 1: bounty hunter reward threshold percentage for entity settlement
	updateFlagsHigh[1] = 0;
	updateValues[1] = 0;

	// 2: entity total time threshold
	updateFlagsHigh[2] = 0;
	updateValues[2] = 0;
	
	// 3: security stake yield compounding rate
	updateFlagsHigh[3] = 0;
	updateValues[3] = 0;

	// 4: entity total time elapsed threshold
	updateFlagsHigh[4] = 0;
	updateValues[4] = 0;

	// 5: minimum stake
	updateFlagsHigh[5] = 0;
	updateValues[5] = 0;

	// 6: percent of security stake rewarded to staker (value in 100_000)
	updateFlagsHigh[6] = 0;
	updateValues[6] = 0;

	updateFlagsHigh[7] = 0;
	updateValues[7] = 0;

	updateFlagsHigh[8] = 0;
	updateValues[8] = 0;

	updateFlagsHigh[9] = 0;
	updateValues[9] = 0;

	updateFlagsHigh[10] = 1;
	updateValues[10] = 0;

	updateFlagsHigh[11] = 0;
	updateValues[11] = 0;

	updateFlagsHigh[12] = 0;
	updateValues[12] = 0;

	updateFlagsHigh[13] = 0;
	updateValues[13] = 0;

	updateFlagsHigh[14] = 0;
	updateValues[14] = 0;

	updateFlagsHigh[15] = 0;
	updateValues[15] = 0;

	updateFlagsHigh[16] = 0;
	updateValues[16] = 0;

	updateFlagsHigh[17] = 0;
	updateValues[17] = 0;

	updateFlagsHigh[18] = 0;
	updateValues[18] = 0;

	updateFlagsHigh[19] = 0;
	updateValues[19] = 0;

	updateFlagsHigh[20] = 0;
	updateValues[20] = 0;

	updateFlagsHigh[21] = 0;
	updateValues[21] = 0;

	updateFlagsHigh[22] = 1;
	updateValues[22] = 0;

	updateFlagsHigh[23] = 0;
	updateValues[23] = 0;

	updateFlagsHigh[24] = 0;
	updateValues[24] = 0;

	updateFlagsHigh[25] = 0;
	updateValues[25] = 0;

	updateFlagsHigh[26] = 0;
	updateValues[26] = 0;

	updateFlagsHigh[27] = 0;
	updateValues[27] = 0;

	updateFlagsHigh[28] = 0;
	updateValues[28] = 0;

	updateFlagsHigh[29] = 1;
	updateValues[29] = 0;

	updateFlagsHigh[30] = 0;
	updateValues[30] = 0;

	updateFlagsHigh[31] = 0;
	updateValues[31] = 0;

	updateFlagsLow[0] = 0;
	updateValues[32] = 0;

	updateFlagsLow[1] = 0;
	updateValues[33] = 0;

	updateFlagsLow[2] = 0;
	updateValues[34] = 0;

	updateFlagsLow[3] = 0;
	updateValues[35] = 0;

	updateFlagsLow[4] = 0;
	updateValues[36] = 0;

	updateFlagsLow[5] = 0;
	updateValues[37] = 0;

	updateFlagsLow[6] = 0;
	updateValues[38] = 0;

	updateFlagsLow[7] = 0;
	updateValues[39] = 0;

	updateFlagsLow[8] = 0;
	updateValues[40] = 0;

	updateFlagsLow[9] = 0;
	updateValues[41] = 0;

	updateFlagsLow[10] = 0;
	updateValues[42] = 0;

	updateFlagsLow[11] = 0;
	updateValues[43] = 0;

	updateFlagsLow[12] = 0;
	updateValues[44] = 0;

	updateFlagsLow[13] = 0;
	updateValues[45] = 0;

	updateFlagsLow[14] = 0;
	updateValues[46] = 0;

	updateFlagsLow[15] = 0;
	updateValues[47] = 0;

	updateFlagsLow[16] = 0;
	updateValues[48] = 0;

	updateFlagsLow[17] = 0;
	updateValues[49] = 0;

	updateFlagsLow[18] = 0;
	updateValues[50] = 0;

	updateFlagsLow[19] = 0;
	updateValues[51] = 0;

	updateFlagsLow[20] = 0;
	updateValues[52] = 0;

	updateFlagsLow[21] = 0;
	updateValues[53] = 0;

	updateFlagsLow[22] = 0;
	updateValues[54] = 0;

	updateFlagsLow[23] = 0;
	updateValues[55] = 0;

	updateFlagsLow[24] = 0;
	updateValues[56] = 0;

	updateFlagsLow[25] = 0;
	updateValues[57] = 0;

	updateFlagsLow[26] = 0;
	updateValues[58] = 0;

	updateFlagsLow[27] = 0;
	updateValues[59] = 0;

	updateFlagsLow[28] = 0;
	updateValues[60] = 0;

	updateFlagsLow[29] = 0;
	updateValues[61] = 0;

	updateFlagsLow[30] = 0;
	updateValues[62] = 0;

	updateFlagsLow[31] = 0;
	updateValues[63] = 0;



	// setup instruction data
	var ixDATA = [1]
		.concat(packFlags32(updateFlagsHigh))
		.concat(packFlags32(updateFlagsLow));

	for (var i = 0; i < 64; i++) {
		ixDATA = ixDATA.concat(u32toBytes(updateValues[i]));
	}

	// prepare transaction
	const UpdateGLOBALtx = new Transaction().add(
		new TransactionInstruction({
			keys: [
				{ pubkey: ownerKEY.publicKey, isSigner: true, isWritable: true, },
				{ pubkey: pdaGLOBAL, isSigner: false, isWritable: true, },
					// new owner below, if needed
				{ pubkey: ownerKEY.publicKey, isSigner: false, isWritable: true, },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
			],
			data: Buffer.from(new Uint8Array(ixDATA)),
			programId: ilocksupremeID,
		})
	);

	console.log("chirp")
		
	// send transaction
	console.log(`txhash: ${await sendAndConfirmTransaction(connection, UpdateGLOBALtx, [ownerKEY], )}`);
	
	// confirmation
	console.log(`\n* Successfully updated GLOBAL account for '${programID}'!\n`);

	} catch {

	console.log(Error);

	}
};

UpdateGlobal();

