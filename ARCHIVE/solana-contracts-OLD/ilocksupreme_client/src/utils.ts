/****************************************************************
 * ILOCKsupreme client utility blob				*	
 ****************************************************************/

/****************************************************************
 * imports							*
 ****************************************************************/

import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

import * as os from "os";
import * as fs from "mz/fs";
import * as path from "path";
import * as yaml from "yaml";
const BufferLayout = require("buffer-layout");
const BigNumber = require("bignumber.js");
const BN = require("bn.js");
const bs58 = require("bs58");
const lodash = require("lodash");
const crypto = require('crypto-js');


/****************************************************************
 * declare constants						*
 ****************************************************************/

export const U16_SIZE = 2;
export const U32_SIZE = 4;
export const U64_SIZE = 8;
export const U128_SIZE = 16;
export const PUBKEY_SIZE = 32;
export const VALUE_NUMBER = 64;
export const VALUES_SIZE = U32_SIZE * VALUE_NUMBER;


export const GLOBAL_SIZE = U128_SIZE +	// pool
			U32_SIZE +	// flags
			PUBKEY_SIZE +	// owner
		  	VALUES_SIZE;	// values
			// = 292

export const USER_SIZE = U16_SIZE +	// flags
		  	U16_SIZE +	// count
		  	U16_SIZE +	// success
		  	U16_SIZE +	// fail
			PUBKEY_SIZE +	// owner
		  	PUBKEY_SIZE +	// vault
		  	U128_SIZE +	// balance
		  	U128_SIZE;	// rewards
			// = 104

export const STAKE_SIZE = U16_SIZE +	// flags
		  	U64_SIZE +	// timestamp
			PUBKEY_SIZE +	// entity
			U128_SIZE;	// amount
			// = 58

export const ENTITY_SIZE = U16_SIZE +	// flags
		  	PUBKEY_SIZE +	// hunter
			U128_SIZE +	// stakepos
			U128_SIZE +	// stakeneg
			U16_SIZE +	// stakers
		  	U64_SIZE;	// timestamp
			// = 76

export let connection: Connection;
export let ownerKEY: Keypair;
export let ilocksupremeID: PublicKey;
export const PROGRAM_KEYFILE = "ILOCKsupreme-keypair.json";
export const PROGRAM_PATH = path.resolve(
	"../../ilocksupreme/target/deploy"
);
export const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, PROGRAM_KEYFILE);

/****************************************************************
 * general functions						*
 ****************************************************************/

/**
* get GLOBAL account data
**/
export async function getGLOBALdata(pdaGLOBAL: PublicKey) {
	// get GLOBAL account data
	const GLOBALaccount = await connection.getAccountInfo(pdaGLOBAL);
	if (GLOBALaccount === null || GLOBALaccount.data.length === 0) {
		console.log(`! GLOBAL account for this owner ID has not been created.`);
		process.exit(1);
	}

	// build GLOBAL struct
	const encodedGLOBALstate = GLOBALaccount.data;
	const decodedGLOBALstate = GLOBAL_DATA_LAYOUT.decode(encodedGLOBALstate) as GLOBALlayout;
	return {
		pool: new BigNumber("0x" + decodedGLOBALstate.pool.toString("hex")),
		flags1: decodedGLOBALstate.flags1,
		flags2: decodedGLOBALstate.flags2,
		owner: new PublicKey(decodedGLOBALstate.owner),
		value0: decodedGLOBALstate.value0,
		value1: decodedGLOBALstate.value1,
		value2: decodedGLOBALstate.value2,
		value3: decodedGLOBALstate.value3,
		value4: decodedGLOBALstate.value4,
		value5: decodedGLOBALstate.value5,
		value6: decodedGLOBALstate.value6,
		value7: decodedGLOBALstate.value7,
		value8: decodedGLOBALstate.value8,
		value9: decodedGLOBALstate.value9,
		value10: decodedGLOBALstate.value10,
		value11: decodedGLOBALstate.value11,
		value12: decodedGLOBALstate.value12,
		value13: decodedGLOBALstate.value13,
		value14: decodedGLOBALstate.value14,
		value15: decodedGLOBALstate.value15,
		value16: decodedGLOBALstate.value16,
		value17: decodedGLOBALstate.value17,
		value18: decodedGLOBALstate.value18,
		value19: decodedGLOBALstate.value19,
		value20: decodedGLOBALstate.value20,
		value21: decodedGLOBALstate.value21,
		value22: decodedGLOBALstate.value22,
		value23: decodedGLOBALstate.value23,
		value24: decodedGLOBALstate.value24,
		value25: decodedGLOBALstate.value25,
		value26: decodedGLOBALstate.value26,
		value27: decodedGLOBALstate.value27,
		value28: decodedGLOBALstate.value28,
		value29: decodedGLOBALstate.value29,
		value30: decodedGLOBALstate.value30,
		value31: decodedGLOBALstate.value31,
		value32: decodedGLOBALstate.value32,
		value33: decodedGLOBALstate.value33,
		value34: decodedGLOBALstate.value34,
		value35: decodedGLOBALstate.value35,
		value36: decodedGLOBALstate.value36,
		value37: decodedGLOBALstate.value37,
		value38: decodedGLOBALstate.value38,
		value39: decodedGLOBALstate.value39,
		value40: decodedGLOBALstate.value40,
		value41: decodedGLOBALstate.value41,
		value42: decodedGLOBALstate.value42,
		value43: decodedGLOBALstate.value43,
		value44: decodedGLOBALstate.value44,
		value45: decodedGLOBALstate.value45,
		value46: decodedGLOBALstate.value46,
		value47: decodedGLOBALstate.value47,
		value48: decodedGLOBALstate.value48,
		value49: decodedGLOBALstate.value49,
		value50: decodedGLOBALstate.value50,
		value51: decodedGLOBALstate.value51,
		value52: decodedGLOBALstate.value52,
		value53: decodedGLOBALstate.value53,
		value54: decodedGLOBALstate.value54,
		value55: decodedGLOBALstate.value55,
		value56: decodedGLOBALstate.value56,
		value57: decodedGLOBALstate.value57,
		value58: decodedGLOBALstate.value58,
		value59: decodedGLOBALstate.value59,
		value60: decodedGLOBALstate.value60,
		value61: decodedGLOBALstate.value61,
		value62: decodedGLOBALstate.value62,
		value63: decodedGLOBALstate.value63,
	}
}

/**
* get USER account data
**/
export async function getUSERdata(pdaUSER: PublicKey) {
	// get USER account data
	const USERaccount = await connection.getAccountInfo(pdaUSER);
	if (USERaccount === null || USERaccount.data.length === 0) {
		console.log(`! This USER account has not been created.`);
		process.exit(1);
	}

	// build USER struct
	const encodedUSERstate = USERaccount.data;
	const decodedUSERstate = USER_DATA_LAYOUT.decode(encodedUSERstate) as USERlayout;
	return {
		flags: decodedUSERstate.flags,
		count: decodedUSERstate.count,
		success: decodedUSERstate.success,
		fail: decodedUSERstate.fail,
		owner: new PublicKey(decodedUSERstate.owner),
		vault: new PublicKey(decodedUSERstate.vault),
		balance: new BigNumber("0x" + decodedUSERstate.balance.toString("hex")),
		rewards: new BigNumber("0x" + decodedUSERstate.rewards.toString("hex")),
	}
}

/**
* get STAKE account data
**/
export async function getSTAKEdata(pdaSTAKE: PublicKey) {
	// get STAKE account data
	const STAKEaccount = await connection.getAccountInfo(pdaSTAKE);
	if (STAKEaccount === null || STAKEaccount.data.length === 0) {
		console.log(`! This STAKE account has not been created.`);
		process.exit(1);
	}

	// build MAIN struct
	const encodedSTAKEstate = STAKEaccount.data;
	const decodedSTAKEstate = STAKE_DATA_LAYOUT.decode(encodedSTAKEstate) as STAKElayout;
	return {
		flags: decodedSTAKEstate.flags,
		timestamp: new BigNumber("0x" + decodedSTAKEstate.timestamp.toString("hex")),
		entity: new PublicKey(decodedSTAKEstate.entity),
		amount: new BigNumber("0x" + decodedSTAKEstate.amount.toString("hex")),

	}
}

/**
* get ENTITY account data
**/
export async function getENTITYdata(pdaENTITY: PublicKey) {
	// get ENTITY account data
	const ENTITYaccount = await connection.getAccountInfo(pdaENTITY);
	if (ENTITYaccount === null || ENTITYaccount.data.length === 0) {
		console.log(`! This ENTITY account has not been created.`);
		process.exit(1);
	}

	// build MAIN struct
	const encodedENTITYstate = ENTITYaccount.data;
	const decodedENTITYstate = ENTITY_DATA_LAYOUT.decode(encodedENTITYstate) as ENTITYlayout;
	return {
		flags: decodedENTITYstate.flags,
		hunter: new PublicKey(decodedENTITYstate.hunter),
		stakepos: new BigNumber("0x" + decodedENTITYstate.stakepos.toString("hex")),
		stakeneg: new BigNumber("0x" + decodedENTITYstate.stakeneg.toString("hex")),
		stakers: decodedENTITYstate.stakers,
		timestamp: new BigNumber("0x" + decodedENTITYstate.timestamp.toString("hex")),

	}
}

/****************************************************************
 * setup layouts and interfaces					*
 ****************************************************************/

/**
 * flags layout
 **/
const flags = (property = "flags") => {
	return BufferLayout.blob(2, property);
};

/**
 * public key layout
 **/
const publicKey = (property = "publicKey") => {
	return BufferLayout.blob(32, property);
};

/**
 * u64 layout
 **/
const uint64 = (property = "uint64") => {
  return BufferLayout.blob(8, property);
};

/**
 * u128 layout
 **/
const uint128 = (property = "uint128") => {
  return BufferLayout.blob(16, property);
};

/**
 * account struct GLOBAL
 **/
export const GLOBAL_DATA_LAYOUT = BufferLayout.struct([
	uint128("pool"),
	BufferLayout.u16("flags2"),
	BufferLayout.u16("flags1"),
	publicKey("owner"),
	BufferLayout.u32("value0"),
	BufferLayout.u32("value1"),
	BufferLayout.u32("value2"),
	BufferLayout.u32("value3"),
	BufferLayout.u32("value4"),
	BufferLayout.u32("value5"),
	BufferLayout.u32("value6"),
	BufferLayout.u32("value7"),
	BufferLayout.u32("value8"),
	BufferLayout.u32("value9"),
	BufferLayout.u32("value10"),
	BufferLayout.u32("value11"),
	BufferLayout.u32("value12"),
	BufferLayout.u32("value13"),
	BufferLayout.u32("value14"),
	BufferLayout.u32("value15"),
	BufferLayout.u32("value16"),
	BufferLayout.u32("value17"),
	BufferLayout.u32("value18"),
	BufferLayout.u32("value19"),
	BufferLayout.u32("value20"),
	BufferLayout.u32("value21"),
	BufferLayout.u32("value22"),
	BufferLayout.u32("value23"),
	BufferLayout.u32("value24"),
	BufferLayout.u32("value25"),
	BufferLayout.u32("value26"),
	BufferLayout.u32("value27"),
	BufferLayout.u32("value28"),
	BufferLayout.u32("value29"),
	BufferLayout.u32("value30"),
	BufferLayout.u32("value31"),
	BufferLayout.u32("value32"),
	BufferLayout.u32("value33"),
	BufferLayout.u32("value34"),
	BufferLayout.u32("value35"),
	BufferLayout.u32("value36"),
	BufferLayout.u32("value37"),
	BufferLayout.u32("value38"),
	BufferLayout.u32("value39"),
	BufferLayout.u32("value40"),
	BufferLayout.u32("value41"),
	BufferLayout.u32("value42"),
	BufferLayout.u32("value43"),
	BufferLayout.u32("value44"),
	BufferLayout.u32("value45"),
	BufferLayout.u32("value46"),
	BufferLayout.u32("value47"),
	BufferLayout.u32("value48"),
	BufferLayout.u32("value49"),
	BufferLayout.u32("value50"),
	BufferLayout.u32("value51"),
	BufferLayout.u32("value52"),
	BufferLayout.u32("value53"),
	BufferLayout.u32("value54"),
	BufferLayout.u32("value55"),
	BufferLayout.u32("value56"),
	BufferLayout.u32("value57"),
	BufferLayout.u32("value58"),
	BufferLayout.u32("value59"),
	BufferLayout.u32("value60"),
	BufferLayout.u32("value61"),
	BufferLayout.u32("value62"),
	BufferLayout.u32("value63"),
]);	
export interface GLOBALlayout {
	pool: Buffer;
	flags2: number;
	flags1: number;
	owner: Uint8Array;
	value0: number;
	value1: number;
	value2: number;
	value3: number;
	value4: number;
	value5: number;
	value6: number;
	value7: number;
	value8: number;
	value9: number;
	value10: number;
	value11: number;
	value12: number;
	value13: number;
	value14: number;
	value15: number;
	value16: number;
	value17: number;
	value18: number;
	value19: number;
	value20: number;
	value21: number;
	value22: number;
	value23: number;
	value24: number;
	value25: number;
	value26: number;
	value27: number;
	value28: number;
	value29: number;
	value30: number;
	value31: number;
	value32: number;
	value33: number;
	value34: number;
	value35: number;
	value36: number;
	value37: number;
	value38: number;
	value39: number;
	value40: number;
	value41: number;
	value42: number;
	value43: number;
	value44: number;
	value45: number;
	value46: number;
	value47: number;
	value48: number;
	value49: number;
	value50: number;
	value51: number;
	value52: number;
	value53: number;
	value54: number;
	value55: number;
	value56: number;
	value57: number;
	value58: number;
	value59: number;
	value60: number;
	value61: number;
	value62: number;
	value63: number;
}

/**
 * account struct USER
 **/
export const USER_DATA_LAYOUT = BufferLayout.struct([
	BufferLayout.u16("flags"),
	BufferLayout.u16("count"),
	BufferLayout.u16("success"),
	BufferLayout.u16("fail"),
	publicKey("owner"),
	publicKey("vault"),
	uint128("balance"),
	uint128("rewards"),
]);
export interface USERlayout {
	flags: number;
	count: number;
	success: number;
	fail: number;
       	owner: Uint8Array;
       	vault: Uint8Array;
	balance: Buffer;
	rewards: Buffer;
}

/**
 * account struct STAKE
 **/
export const STAKE_DATA_LAYOUT = BufferLayout.struct([
	BufferLayout.u16("flags"),
	uint64("timestamp"),
	publicKey("entity"),
	uint128("amount"),
]);
export interface STAKElayout {
	flags: number;
	timestamp: Buffer;
       	entity: Uint8Array;
	amount: Buffer;
}

/**
 * account struct ENTITY
 **/
export const ENTITY_DATA_LAYOUT = BufferLayout.struct([
	BufferLayout.u16("flags"),
	publicKey("hunter"),
	uint128("stakepos"),
	uint128("stakeneg"),
	BufferLayout.u16("stakers"),
	uint64("timestamp"),
]);
export interface ENTITYlayout {
	flags: number;
       	hunter: Uint8Array;
	stakepos: Buffer;
	stakeneg: Buffer;
	stakers: number;
	timestamp: Buffer;
}






/**
* check flag template
**/

export function templateFlagCheck(flags: number) {
	const flagarray = unpackFlags(flags);
	return flagarray[0] === 1;
}

/**
* transaction template
**

export function templateTX(
	pdaMAIN: PublicKey,
	pdaPIECE: PublicKey,
	pdaREF: PublicKey,
	ixDATA: any[]) {

	// setup transaction
	return new Transaction().add(
		new TransactionInstruction({
			keys: [
				{ pubkey: ownerKEY.publicKey, isSigner: true, isWritable: true, },
				{ pubkey: pdaMAIN, isSigner: false, isWritable: true, },
				{ pubkey: pdaPIECE, isSigner: false, isWritable: true, },
				{ pubkey: pdaREF, isSigner: false, isWritable: true, },
				{ pubkey: inviteKEY, isSigner: false, isWritable: false, },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
			],
			data: Buffer.from(new Uint8Array(ixDATA)),
			programId: ilocksupremeID,
		})
	);
}

/**
* extra compute tx
***

export function extracomputeTX(
	pdaselfTARGET: PublicKey,
	pdaTARGET: PublicKey,
	pdaPIECE: PublicKey,
	pdaselfREF: PublicKey,
	pdaREF: PublicKey,
	ixDATA: any[]) {
	
	// raise compute budget for pda derivation max
	const data = Buffer.from(
    	Uint8Array.of(0, ...new BN(650000).toArray("le", 4))
  	);
  	const additionalComputeBudgetInstruction = new TransactionInstruction({
    		keys: [],
    		programId: new PublicKey("ComputeBudget111111111111111111111111111111"),
    		data,
  	});

	// setup transaction
	return new Transaction().add(additionalComputeBudgetInstruction)
		.add(new TransactionInstruction({
			keys: [
				{ pubkey: ownerKEY.publicKey, isSigner: true, isWritable: true, },
				{ pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false, },
				{ pubkey: pdaselfTARGET, isSigner: false, isWritable: true, },
				{ pubkey: pdaTARGET, isSigner: false, isWritable: true, },
				{ pubkey: pdaPIECE, isSigner: false, isWritable: true, },
				{ pubkey: pdaselfREF, isSigner: false, isWritable: true, },
				{ pubkey: pdaREF, isSigner: false, isWritable: true, },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
			],
			data: Buffer.from(new Uint8Array(ixDATA)),
			programId: ilocksupremeID,
		})
	);
}

*/

/**
* unpack flags
**/
export function unpackFlags(flags: number) {
	const highflags = flags >> 8;
	const lowflags = flags & 0xFF;
	var bitarray = new Uint8Array(16);
	for (var index = 0; index < 8; index++) {
		bitarray[index] = (highflags >> (7 - index)) & 0x01;
	}
	for (index = 0; index < 8; index++) {
		bitarray[8 + index] = (lowflags >> (7 - index)) & 0x01;
	}
	return bitarray
}

/**
* unpack flags 32
**/
export function unpackFlags32(flags: number) {
	const flags1 = (flags >> 24) & 0xFF;
	const flags2 = (flags >> 16) & 0xFF;
	const flags3 = (flags >> 8) & 0xFF;
	const flags4 = flags & 0xFF;
	var bitarray = new Uint8Array(32);
	for (var index = 0; index < 8; index++) {
		bitarray[index] = (flags1 >> (7 - index)) & 0x01;
	}
	for (index = 0; index < 8; index++) {
		bitarray[8 + index] = (flags2 >> (7 - index)) & 0x01;
	}
	for (index = 0; index < 8; index++) {
		bitarray[16 + index] = (flags3 >> (7 - index)) & 0x01;
	}
	for (index = 0; index < 8; index++) {
		bitarray[24 + index] = (flags4 >> (7 - index)) & 0x01;
	}
	return bitarray
}

/**
* pack flags 32
**/
export function packFlags32(flags: Uint8Array) {
	
	var byte1 = flags[0] << 7 |
		flags[1] << 6 |
		flags[2] << 5 |
		flags[3] << 4 |
		flags[4] << 3 |
		flags[5] << 2 |
		flags[6] << 1 |
		flags[7];
	var byte2 = flags[8] << 7 |
		flags[9] << 6 |
		flags[10] << 5 |
		flags[11] << 4 |
		flags[12] << 3 |
		flags[13] << 2 |
		flags[14] << 1 |
		flags[15];
	var byte3 = flags[16] << 7 |
		flags[17] << 6 |
		flags[18] << 5 |
		flags[19] << 4 |
		flags[20] << 3 |
		flags[21] << 2 |
		flags[22] << 1 |
		flags[23];

	var byte4 = flags[24] << 7 |
		flags[25] << 6 |
		flags[26] << 5 |
		flags[27] << 4 |
		flags[28] << 3 |
		flags[29] << 2 |
		flags[30] << 1 |
		flags[31];

	return [byte4, byte3, byte2, byte1]
}


/**
* create pda seed
**/
export function createSeed(pda: PublicKey, count: Uint16Array) {
	let countLow = count[0] & 0xFF; 		// mask for low order count byte
	let countHigh = (count[0] >> 8) & 0xFF; 	// shift and mask for high order count byte
	return toUTF8Array(pda
			   .toString()
			   .slice(0,PUBKEY_SIZE - U16_SIZE))
			   .concat(countHigh, countLow);
}

/**
* u32 to bytes
**/
export function u32toBytes(number: number) {
	let byte1 = number & 0xFF; 		// mask for lowest order number byte
	let byte2 = (number >> 8) & 0xFF; 	// shift and mask for next lowest order number byte
	let byte3 = (number >> 16) & 0xFF; 	// shift and mask for high order number byte
	let byte4 = (number >> 24) & 0xFF; 	// shift and mask for highest order number byte
	return [byte4, byte3, byte2, byte1];
}





/**
* derive pda
**/
export async function deriveAddress(seed: any[]) {
	return await PublicKey.findProgramAddress(
		[new Uint8Array(seed)], ilocksupremeID);
}

/**
* find invitation hash
**
export async function findHash(inviteHASH: string) {
	return  await connection.getParsedProgramAccounts(
		ilocksupremeID,
		{
			filters: [
				{
					dataSize: REF_SIZE,
				},
				{
					memcmp: {
						offset: FLAGS_SIZE,
						bytes: inviteHASH,
					},
				},
			],
		},
	);
}

/**
* check to make sure owner ID isn't already taken
**
export async function availableIDcheck(ownerID: string): Promise<void> {
	const ownerIDaccount = await connection.getParsedProgramAccounts(
		ilocksupremeID,
		{
			filters: [
				{
					dataSize: PIECE_SIZE,
				},
				{
					memcmp: {
						offset: PIECE_SIZE - PIECESLUG_SIZE,
						bytes: bs58.encode(toUTF8Array(ownerID)),
					},
				},
			],
		},
	);
	if (!lodash.isEqual(ownerIDaccount, [])) {
		console.log(`! The owner ID '${ownerID}' already has a MAIN account associated with it.\n`,
			    ` Choose a different ID for your owner MAIN account.`,
		);
		process.exit(1);
	}
}
*/

/**
* ProgramInit tx
***
export function ProgramInitTX(
	pdaGLOBAL: PublicKey,
	ixDATA: any[]) {
	
	console.log("chirp")
	return new Transaction().add(
		new TransactionInstruction({
			keys: [
				{ pubkey: ownerKEY.publicKey, isSigner: true, isWritable: true, },
				{ pubkey: pdaGLOBAL, isSigner: false, isWritable: true, },
				{ pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false, },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
			],
			data: Buffer.from(new Uint8Array(ixDATA)),
			programId: ilocksupremeID,
		})
	);
}
*/



/**
* get all STAKEs with specific ENTITY account
***/
export async function getSTAKEs(hash: string) {
	return await connection.getParsedProgramAccounts(
		ilocksupremeID,
		{
			filters: [
				{
					dataSize: STAKE_SIZE,
				},
				{
					memcmp: {
						offset: U16_SIZE + U64_SIZE,
						bytes: hash,
					},
				},
			],
		},
	);
}

/**
* get all ENTITYs
***/
export async function getENTITYs() {
	return await connection.getParsedProgramAccounts(
		ilocksupremeID,
		{
			filters: [
				{
					dataSize: ENTITY_SIZE,
				},
				{
					memcmp: {
						offset: 0,
						bytes: "",
					},
				},
			],
		},
	);
}


/**
* get all USERs
***/
export async function getUSERs() {
	return await connection.getParsedProgramAccounts(
		ilocksupremeID,
		{
			filters: [
				{
					dataSize: USER_SIZE,
				},
				{
					memcmp: {
						offset: 0,
						bytes: "",
					},
				},
			],
		},
	);
}

/**
* get GLOBAL
***/
export async function getGLOBAL() {
	return await connection.getParsedProgramAccounts(
		ilocksupremeID,
		{
			filters: [
				{
					dataSize: GLOBAL_SIZE,
				},
				{
					memcmp: {
						offset: 0,
						bytes: "",
					},
				},
			],
		},
	);
}


/**
* Check if the hello world BPF program has been deployed
**/
export async function checkProgram(): Promise<void> {
	// Read program id from keypair file
	try {
		const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
		ilocksupremeID = programKeypair.publicKey;
		console.log(`. ILOCKsupreme found at:\t${ilocksupremeID.toBase58()}`);
	} catch (err) {
		const errMsg = (err as Error).message;
		throw new Error(
			`! Failed to read program keypair at "${PROGRAM_KEYPAIR_PATH}" due to error: ${errMsg}.\n
			Program may need to be deployed with 
			\`solana program deploy fracpay_server/target/deploy/fracpay_server.so\``,
		);
	}
}

/**
 * establish connection
 **/
export async function establishConnection(): Promise<void> {
	const rpcUrl = await getRpcUrl();
  	connection = new Connection(rpcUrl, "confirmed");
  	const version = await connection.getVersion();
  	console.log(". Connection to cluster established:", rpcUrl, version);
}

async function getRpcUrl(): Promise<string> {
  	try {
    		const config = await getConfig();
    		if (!config.json_rpc_url) throw new Error("Missing RPC URL");
    		return config.json_rpc_url;
 	 } catch (err) {
    		console.warn(
      			"! Failed to read RPC url from CLI config file, falling back to localhost",
    		);
    		return "http://localhost:8899";
  	}
}

/**
 * get owner's local solana config
 **/
async function getConfig(): Promise<any> {
  	// Path to Solana CLI config file
  	const CONFIG_FILE_PATH = path.resolve(
    		os.homedir(),
    		".config",
    		"solana",
    		"cli",
    		"config.yml",
  	);
  	const configYml = await fs.readFile(CONFIG_FILE_PATH, {encoding: "utf8"});
  	return yaml.parse(configYml);
}

/**
 * establish owner
 **/
export async function establishOperator(): Promise<void> {
  	let fees = 0;
  	if (!ownerKEY) {
    		const {feeCalculator} = await connection.getRecentBlockhash();

 		// Calculate the cost to fund the greeter account
    		fees += await connection.getMinimumBalanceForRentExemption(GLOBAL_SIZE);

    		// Calculate the cost of sending transactions
    		fees += feeCalculator.lamportsPerSignature * 100; // wag

    		ownerKEY = await getOperator();
  	}

  	let lamports = await connection.getBalance(ownerKEY.publicKey);
  	if (lamports < fees) {

    		// If current balance is not enough to pay for fees, request an airdrop
    		console.log(
			`! Unfortunately you do not have enough SOL to initialize an account.\n`,
			`  You need ${fees/LAMPORTS_PER_SOL} SOL to initialize account.`,
		)
  	}

  	console.log(
    		". Operator account is:\t",
    		ownerKEY.publicKey.toBase58(),
    		"containing",
    		lamports / LAMPORTS_PER_SOL,
    		"SOL to pay for fees",
  	);
}

/**
 * setup ownerKEY as Keypair
 **/
async function getOperator(): Promise<Keypair> {
  	try {
    		const config = await getConfig();
    		if (!config.keypair_path) throw new Error("Missing keypair path");
    		return await createKeypairFromFile(config.keypair_path);
  	} catch (err) {
    		console.warn(
      			"! Failed to create keypair from CLI config file, falling back to new random keypair",
    		);
    		return Keypair.generate();
 	 }
}

/**
 * read secret key from file and return Keypair object
 **/
async function createKeypairFromFile(filePath: string,): Promise<Keypair> {
  	const secretKeyString = await fs.readFile(filePath, {encoding: "utf8"});
 	const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  	return Keypair.fromSecretKey(secretKey);
}

/**
 * return private key from 64 byte array in file
 **/
const getPrivateKey = (name: string) =>
	Uint8Array.from(
		JSON.parse(fs.readFileSync(`./keys/${name}_pri.json`) as unknown as string)
	);

/**
 * return public key from base58 formatted string in file
 **/
const getPublicKey = (name: string) =>
	new PublicKey(
		JSON.parse(fs.readFileSync(`./keys/${name}_pub.json`) as unknown as string)
 	);

/**
 * write a public key to file [presumably hex string, haven't checked yet]
 **/
const writePublicKey = (publicKey: PublicKey, name: string) => {
	fs.writeFileSync(
		`./keys/${name}_pub.json`,
		JSON.stringify(publicKey.toString())
	);
};

/**
 * creates Keypair object from named pubkey prikey json files
 **/
export const getKeypair = (name: string) =>
	new Keypair({
		publicKey: new Uint8Array(getPublicKey(name).toBytes()),
		secretKey: getPrivateKey(name),
	});
/**
 * read fracpay program ID from json file in keys directory
 **/
export const getProgramID = () => {
	try {
		return getPublicKey("ILOCKsupreme");
	} catch (error) {
		console.log("Given programId is missing or incorrect");
	process.exit(1);
	}
};

/**
 * take in a UTF8 array and turn it into a string
 **/
export function fromUTF8Array(data: Uint8Array) { // array of bytes
    	var str = "";
    	var i;

    	for (i = 0; i < data.length; i++) {
        	var value = data[i];

        	if (value < 0x80) {
            		str += String.fromCharCode(value);
       		} else if (value > 0xBF && value < 0xE0) {
            		str += String.fromCharCode((value & 0x1F) << 6 | data[i + 1] & 0x3F);
            		i += 1;
        	} else if (value > 0xDF && value < 0xF0) {
            		str += String.fromCharCode((value & 0x0F) << 12 | (data[i + 1] & 0x3F) << 6 | data[i + 2] & 0x3F);
            		i += 2;
        	} else {
            		// surrogate pair
            		var charCode = ((value & 0x07) << 18 | 
					(data[i + 1] & 0x3F) << 12 |
					(data[i + 2] & 0x3F) << 6 |
					 data[i + 3] & 0x3F) - 0x010000;

            		str += String.fromCharCode(charCode >> 10 | 0xD800, charCode & 0x03FF | 0xDC00);
            		i += 3;
        	}
    	}
   	 return str;
}

/**
 * take in a string and turn it into a UTF8 byte array
 **/
export function toUTF8Array(str: string) {
    	var utf8 = [];
    	for (var i=0; i < str.length; i++) {
        	var charcode = str.charCodeAt(i);
        	if (charcode < 0x80) utf8.push(charcode);
        	else if (charcode < 0x800) {
           		utf8.push(0xc0 | (charcode >> 6),
                      		  0x80 | (charcode & 0x3f));
        	}
        	else if (charcode < 0xd800 || charcode >= 0xe000) {
            		utf8.push(0xe0 | (charcode >> 12),
                      		  0x80 | ((charcode>>6) & 0x3f),
                      		  0x80 | (charcode & 0x3f));
        	}
        	// surrogate pair
        	else {
            		i++;
            		charcode = ((charcode&0x3ff)<<10)|(str.charCodeAt(i)&0x3ff)
            		utf8.push(0xf0 | (charcode >>18),
                      		  0x80 | ((charcode>>12) & 0x3f),
                      		  0x80 | ((charcode>>6) & 0x3f),
                      		  0x80 | (charcode & 0x3f));
        	}
    	}
    	return utf8;
}




















// MISC FUNCTION HERE


/**
* create keyhash
**/

export function newKeyhash() {
	const newkey = new Keypair();
	var keyhash = crypto.SHA256(newkey.publicKey.toString());
	keyhash = bs58.encode(Buffer.from(keyhash.toString(), 'hex'));
	keyhash = new PublicKey(keyhash);
	return [newkey.publicKey, keyhash];
}

export function newURLhash(URL: string) {
	var URLhash = crypto.SHA256(URL);
	URLhash = bs58.encode(Buffer.from(URLhash.toString(), 'hex'));
	URLhash = new PublicKey(URLhash);
	return URLhash;
}




/**
* print verbose REF list, no flags
**
export async function verboseREFlist(pdaPIECE: PublicKey, count: number) {

	// initialize piece counter
	var countREF = new Uint16Array(1);
	countREF[0] = 0;

	// find self REF address
	var pdaREFseed = createSeed(pdaPIECE, countREF);
	var [pdaREF, bumpREF] = await deriveAddress(pdaREFseed);

	// get self PIECE data
	var REF = await getREFdata(pdaREF);

	// get flags
	var flags = unpackFlags(REF.flags);

	// print self PIECE data
	console.log(`\t. 0\t| SELF: --------> ${REF.refslug}`);
	console.log(`\t\t| ADDRESS: -----> ${pdaREF.toBase58()}`);
	console.log(`\t\t| TARGET: ------> ${REF.target.toBase58()}`);
	console.log(`\t\t| FRACTION: ----> ${REF.fract}`);
	console.log(`\t\t| NETSUM: ------> ${REF.netsum}`);
	process.stdout.write(`\t\t| FLAGS: -------> `);
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

	// cycle through all pieces
	for (countREF[0] = 1; countREF[0] <= count; countREF[0]++) {

		// find PIECE address
		pdaREFseed = createSeed(pdaPIECE, countREF);
		[pdaREF, bumpREF] = await deriveAddress(pdaREFseed);

		// get PIECE data
		REF = await getREFdata(pdaREF);

		// get flags
		flags = unpackFlags(REF.flags);

		// print PIECE data
		console.log(`\t. ${countREF[0]}\t| REF ID: ------> ${REF.refslug}`);
		console.log(`\t\t| ADDRESS: -----> ${pdaREF.toBase58()}`);
		console.log(`\t\t| TARGET: ------> ${REF.target.toBase58()}`);
		console.log(`\t\t| FRACTION: ----> ${REF.fract}`);
		console.log(`\t\t| NETSUM: ------> ${REF.netsum}`);
		process.stdout.write(`\t\t| FLAGS: -------> `);
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
}

/**
* print REF list
**
export async function printREFlist(pdaPIECE: PublicKey, count: number) {

	// initialize piece counter
	var countREF = new Uint16Array(1);
	countREF[0] = 0;

	// find self REF address
	var pdaREFseed = createSeed(pdaPIECE, countREF);
	var [pdaREF, bumpREF] = await deriveAddress(pdaREFseed);

	// get self PIECE data
	var REF = await getREFdata(pdaREF);

	// print self PIECE data
	console.log(`\t. 0\tSELF:\t${REF.refslug}`);

	// cycle through all pieces
	for (countREF[0] = 1; countREF[0] <= count; countREF[0]++) {

		// find PIECE address
		pdaREFseed = createSeed(pdaPIECE, countREF);
		[pdaREF, bumpREF] = await deriveAddress(pdaREFseed);

		// get PIECE data
		REF = await getREFdata(pdaREF);

		// print PIECE data
		console.log(`\t. ${countREF[0]}\tREF ID:\t${REF.refslug}`);
	}	
	console.log("");
}

/**
* get PIECE list
**
export async function printPIECElist(pdaMAIN: PublicKey, count: number) {

	// initialize piece counter
	var countPIECE = new Uint16Array(1);
	countPIECE[0] = 0;

	// find self PIECE address
	var pdaPIECEseed = createSeed(pdaMAIN, countPIECE);
	var [pdaPIECE, bumpPIECE] = await deriveAddress(pdaPIECEseed);

	// get self PIECE data
	var PIECE = await getPIECEdata(pdaPIECE);

	// print self PIECE data
	console.log(`# 0\tOPERATOR:\t${PIECE.pieceslug}`);

	// cycle through all pieces
	for (countPIECE[0] = 1; countPIECE[0] <= count; countPIECE[0]++) {

		// find PIECE address
		pdaPIECEseed = createSeed(pdaMAIN, countPIECE);
		[pdaPIECE, bumpPIECE] = await deriveAddress(pdaPIECEseed);

		// get PIECE data
		PIECE = await getPIECEdata(pdaPIECE);

		// print PIECE data
		console.log(`# ${countPIECE[0]}\tPIECE ID:\t${PIECE.pieceslug}`);
	}	
}
*/

