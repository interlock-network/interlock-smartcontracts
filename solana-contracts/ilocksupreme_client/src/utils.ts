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
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

import * as os from "os";
import * as fs from "mz/fs";
import * as path from "path";
import * as yaml from "yaml";
import * as BufferLayout from "buffer-layout";
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
export let operatorKEY: Keypair;
export const PROGRAM_KEYFILE = "fracpay_server-keypair.json";
export const PROGRAM_PATH = path.resolve(
	"/Users/blairmunroakusa/_ROOT/___LEAF/fracpay/fracpay_server/target/deploy"
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
		flags: decodedGLOBALstate.flags,
		owner: new PublicKey(decodedGLOBALstate.owner),
		values: decodedGLOBALstate.values,
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
		hunter: new PublicKey(decodedSTAKEstate.hunter),
		stakepos: new BigNumber("0x" + decodedSTAKEstate.stakepos.toString("hex")),
		stakeneg: new BigNumber("0x" + decodedSTAKEstate.stakeneg.toString("hex")),
		stakers: decodedENTITYstate.stakers,
		timestamp: new BigNumber("0x" + decodedSTAKEstate.timestamp.toString("hex")),

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
 * values layout
 **/
const values = (property = "values") => {
	return BufferLayout.blob(256, property);
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
	BufferLayout.u16("flags1"),
	BufferLayout.u16("flags2"),
	publicKey("owner"),
	values("values"),
]);	
export interface GLOBALlayout {
	pool: Buffer;
	flags1: number;
	flags2: number;
	owner: Uint8Array;
	values: Buffer;
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
	return flagarray[<<flagnumberhere>>] === 1;
}











/**
* general link transaction
**/

export function linkTX(
	pdaMAIN: PublicKey,
	pdaPIECE: PublicKey,
	pdaREF: PublicKey,
	ixDATA: any[]) {

	// setup transaction
	return new Transaction().add(
		new TransactionInstruction({
			keys: [
				{ pubkey: operatorKEY.publicKey, isSigner: true, isWritable: true, },
				{ pubkey: pdaMAIN, isSigner: false, isWritable: true, },
				{ pubkey: pdaPIECE, isSigner: false, isWritable: true, },
				{ pubkey: pdaREF, isSigner: false, isWritable: true, },
				{ pubkey: inviteKEY, isSigner: false, isWritable: false, },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
			],
			data: Buffer.from(new Uint8Array(ixDATA)),
			programId: fracpayID,
		})
	);
}

/**
* general init transaction
**/

export function initTX(
	pdaMAIN: PublicKey,
	pdaPIECE: PublicKey,
	pdaREF: PublicKey,
	pdaselfREF: PublicKey,
	invitarget: PublicKey,
	ixDATA: any[]) {

	// setup transaction
	return new Transaction().add(
		new TransactionInstruction({
			keys: [
				{ pubkey: operatorKEY.publicKey, isSigner: true, isWritable: true, },
				{ pubkey: invitarget, isSigner: false, isWritable: true, },
				{ pubkey: pdaMAIN, isSigner: false, isWritable: true, },
				{ pubkey: pdaPIECE, isSigner: false, isWritable: true, },
				{ pubkey: pdaREF, isSigner: false, isWritable: true, },
				{ pubkey: pdaselfREF, isSigner: false, isWritable: true, },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
			],
			data: Buffer.from(new Uint8Array(ixDATA)),
			programId: fracpayID,
		})
	);
}

/**
* pay transaction
**/

export function payTX(
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
				{ pubkey: operatorKEY.publicKey, isSigner: true, isWritable: true, },
				{ pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false, },
				{ pubkey: pdaselfTARGET, isSigner: false, isWritable: true, },
				{ pubkey: pdaTARGET, isSigner: false, isWritable: true, },
				{ pubkey: pdaPIECE, isSigner: false, isWritable: true, },
				{ pubkey: pdaselfREF, isSigner: false, isWritable: true, },
				{ pubkey: pdaREF, isSigner: false, isWritable: true, },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
			],
			data: Buffer.from(new Uint8Array(ixDATA)),
			programId: fracpayID,
		})
	);
}

/**
* general create transaction
**/

export function createTX(
	pdaMAIN: PublicKey,
	pdaPIECE: PublicKey,
	pdaREF: PublicKey,
	extra: PublicKey,
	ixDATA: any[]) {


	// setup transaction
	return new Transaction().add(
		new TransactionInstruction({
			keys: [
				{ pubkey: operatorKEY.publicKey, isSigner: true, isWritable: true, },
				{ pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false, },
				{ pubkey: pdaMAIN, isSigner: false, isWritable: true, },
				{ pubkey: pdaPIECE, isSigner: false, isWritable: true, },
				{ pubkey: pdaREF, isSigner: false, isWritable: true, },
				{ pubkey: extra, isSigner: false, isWritable: true, },
				{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false, },
			],
			data: Buffer.from(new Uint8Array(ixDATA)),
			programId: fracpayID,
		})
	);
}

/**
* print verbose REF list, no flags
**/
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
**/
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
**/
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
* create pda seed
**/
export function createSeed(pda: PublicKey, count: Uint16Array) {
	let countLow = count[0] & 0xFF; 		// mask for low order count byte
	let countHigh = (count[0] >> 8) & 0xFF; 	// shift and mask for high order count byte
	return toUTF8Array(pda
			   .toString()
			   .slice(0,PUBKEY_SIZE - COUNT_SIZE))
			   .concat(countHigh, countLow);
}

/**
* u32 to bytes
**/
export function u32toBytes(number: Uint32Array) {
	let byte1 = number[0] & 0xFF; 		// mask for lowest order number byte
	let byte2 = (number[0] >> 8) & 0xFF; 	// shift and mask for next lowest order number byte
	let byte3 = (number[0] >> 16) & 0xFF; 	// shift and mask for high order number byte
	let byte4 = (number[0] >> 24) & 0xFF; 	// shift and mask for highest order number byte
	return [byte4, byte3, byte2, byte1];
}



/**
* derive pda
**/
export async function deriveAddress(seed: any[]) {
	return await PublicKey.findProgramAddress(
		[new Uint8Array(seed)], fracpayID);
}

/**
* find invitation hash
**/
export async function findHash(inviteHASH: string) {
	return  await connection.getParsedProgramAccounts(
		fracpayID,
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
* check to make sure operator ID isn't already taken
**/
export async function availableIDcheck(operatorID: string): Promise<void> {
	const operatorIDaccount = await connection.getParsedProgramAccounts(
		fracpayID,
		{
			filters: [
				{
					dataSize: PIECE_SIZE,
				},
				{
					memcmp: {
						offset: PIECE_SIZE - PIECESLUG_SIZE,
						bytes: bs58.encode(toUTF8Array(operatorID)),
					},
				},
			],
		},
	);
	if (!lodash.isEqual(operatorIDaccount, [])) {
		console.log(`! The operator ID '${operatorID}' already has a MAIN account associated with it.\n`,
			    ` Choose a different ID for your operator MAIN account.`,
		);
		process.exit(1);
	}
}

/**
* get all PIECEs with specific MAIN operator account
***/
export async function getPIECEs(operator: PublicKey) {
	console.log("chirp");
	return await connection.getParsedProgramAccounts(
		fracpayID,
		{
			filters: [
				{
					dataSize: PIECE_SIZE,
				},
				{
					memcmp: {
						offset: FLAGS_SIZE,
						bytes: operator.toString(),
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
		fracpayID = programKeypair.publicKey;
		console.log(`. Fracpay found at:\t${fracpayID.toBase58()}`);
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
 * get operator's local solana config
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
 * establish operator
 **/
export async function establishOperator(): Promise<void> {
  	let fees = 0;
  	if (!operatorKEY) {
    		const {feeCalculator} = await connection.getRecentBlockhash();

 		// Calculate the cost to fund the greeter account
    		fees += await connection.getMinimumBalanceForRentExemption(MAIN_SIZE + PIECE_SIZE + REF_SIZE);

    		// Calculate the cost of sending transactions
    		fees += feeCalculator.lamportsPerSignature * 100; // wag

    		operatorKEY = await getOperator();
  	}

  	let lamports = await connection.getBalance(operatorKEY.publicKey);
  	if (lamports < fees) {

    		// If current balance is not enough to pay for fees, request an airdrop
    		console.log(
			`! Unfortunately you do not have enough SOL to initialize an account.\n`,
			`  You need ${fees/LAMPORTS_PER_SOL} SOL to initialize account.`,
		)
  	}

  	console.log(
    		". Operator account is:\t",
    		operatorKEY.publicKey.toBase58(),
    		"containing",
    		lamports / LAMPORTS_PER_SOL,
    		"SOL to pay for fees",
  	);
}

/**
 * setup operatorKEY as Keypair
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
		return getPublicKey("fracpay");
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
