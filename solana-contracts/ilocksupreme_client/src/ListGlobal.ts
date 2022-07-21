/****************************************************************
 * Fracpay client ListGlobal
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
	getGLOBALdata,
	getSTAKEdata,
	getUSERdata,
	toUTF8Array,
	getGLOBAL,
} from "./utils";

/****************************************************************
 * main								
 ****************************************************************/

const ListGlobal = async () => {
	
	try {
	
	// setup
	await establishConnection();
	await establishOperator();
	await checkProgram();

	// get PIECE accounts with operator Key in operator field
	const GLOBE = await getGLOBAL();

	// state intention
	console.log(`GLOBAL:\n`);
	

	var GLOBAL = await getGLOBALdata(GLOBE[0].pubkey);

	// get flags
	var flags1 = unpackFlags(GLOBAL.flags1);
	var flags2 = unpackFlags(GLOBAL.flags2);

	// print STAKE data
	console.log(`| ADDRESS: ----- ${GLOBE[0].pubkey}`);
	console.log(`| POOL: -------- ${GLOBAL.pool}`);
	console.log(`| FLAGS1: ------ ${GLOBAL.flags1}`);
	console.log(`| FLAGS2: ------ ${GLOBAL.flags2}`);
	console.log(`| OWNER: ------- ${GLOBAL.owner}`);

	console.log(`| VALUE0: ------ ${GLOBAL.value0}`);
	console.log(`| VALUE1: ------ ${GLOBAL.value1}`);
	console.log(`| VALUE2: ------ ${GLOBAL.value2}`);
	console.log(`| VALUE3: ------ ${GLOBAL.value3}`);
	console.log(`| VALUE4: ------ ${GLOBAL.value4}`);
	console.log(`| VALUE5: ------ ${GLOBAL.value5}`);
	console.log(`| VALUE6: ------ ${GLOBAL.value6}`);
	console.log(`| VALUE7: ------ ${GLOBAL.value7}`);
	console.log(`| VALUE8: ------ ${GLOBAL.value8}`);
	console.log(`| VALUE9: ------ ${GLOBAL.value9}`);

	console.log(`| VALUE10: ----- ${GLOBAL.value10}`);
	console.log(`| VALUE11: ----- ${GLOBAL.value11}`);
	console.log(`| VALUE12: ----- ${GLOBAL.value12}`);
	console.log(`| VALUE13: ----- ${GLOBAL.value13}`);
	console.log(`| VALUE14: ----- ${GLOBAL.value14}`);
	console.log(`| VALUE15: ----- ${GLOBAL.value15}`);
	console.log(`| VALUE16: ----- ${GLOBAL.value16}`);
	console.log(`| VALUE17: ----- ${GLOBAL.value17}`);
	console.log(`| VALUE18: ----- ${GLOBAL.value18}`);
	console.log(`| VALUE19: ----- ${GLOBAL.value19}`);

	console.log(`| VALUE20: ----- ${GLOBAL.value20}`);
	console.log(`| VALUE21: ----- ${GLOBAL.value21}`);
	console.log(`| VALUE22: ----- ${GLOBAL.value22}`);
	console.log(`| VALUE23: ----- ${GLOBAL.value23}`);
	console.log(`| VALUE24: ----- ${GLOBAL.value24}`);
	console.log(`| VALUE25: ----- ${GLOBAL.value25}`);
	console.log(`| VALUE26: ----- ${GLOBAL.value26}`);
	console.log(`| VALUE27: ----- ${GLOBAL.value27}`);
	console.log(`| VALUE28: ----- ${GLOBAL.value28}`);
	console.log(`| VALUE29: ----- ${GLOBAL.value29}`);

	console.log(`| VALUE30: ----- ${GLOBAL.value30}`);
	console.log(`| VALUE31: ----- ${GLOBAL.value31}`);
	console.log(`| VALUE32: ----- ${GLOBAL.value32}`);
	console.log(`| VALUE33: ----- ${GLOBAL.value33}`);
	console.log(`| VALUE34: ----- ${GLOBAL.value34}`);
	console.log(`| VALUE35: ----- ${GLOBAL.value35}`);
	console.log(`| VALUE36: ----- ${GLOBAL.value36}`);
	console.log(`| VALUE37: ----- ${GLOBAL.value37}`);
	console.log(`| VALUE38: ----- ${GLOBAL.value38}`);
	console.log(`| VALUE39: ----- ${GLOBAL.value39}`);

	console.log(`| VALUE40: ----- ${GLOBAL.value40}`);
	console.log(`| VALUE41: ----- ${GLOBAL.value41}`);
	console.log(`| VALUE42: ----- ${GLOBAL.value42}`);
	console.log(`| VALUE43: ----- ${GLOBAL.value43}`);
	console.log(`| VALUE44: ----- ${GLOBAL.value44}`);
	console.log(`| VALUE45: ----- ${GLOBAL.value45}`);
	console.log(`| VALUE46: ----- ${GLOBAL.value46}`);
	console.log(`| VALUE47: ----- ${GLOBAL.value47}`);
	console.log(`| VALUE48: ----- ${GLOBAL.value48}`);
	console.log(`| VALUE49: ----- ${GLOBAL.value49}`);

	console.log(`| VALUE50: ----- ${GLOBAL.value50}`);
	console.log(`| VALUE51: ----- ${GLOBAL.value51}`);
	console.log(`| VALUE52: ----- ${GLOBAL.value52}`);
	console.log(`| VALUE53: ----- ${GLOBAL.value53}`);
	console.log(`| VALUE54: ----- ${GLOBAL.value54}`);
	console.log(`| VALUE55: ----- ${GLOBAL.value55}`);
	console.log(`| VALUE56: ----- ${GLOBAL.value56}`);
	console.log(`| VALUE57: ----- ${GLOBAL.value57}`);
	console.log(`| VALUE58: ----- ${GLOBAL.value58}`);
	console.log(`| VALUE59: ----- ${GLOBAL.value59}`);

	console.log(`| VALUE60: ----- ${GLOBAL.value60}`);
	console.log(`| VALUE61: ----- ${GLOBAL.value61}`);
	console.log(`| VALUE62: ----- ${GLOBAL.value62}`);
	console.log(`| VALUE63: ----- ${GLOBAL.value63}`);

	process.stdout.write(`\t| FLAGS1: ------ `);
	process.stdout.write(`[ `);
	for (var index = 0; index < 4; index++) {
		process.stdout.write(`${flags1[index]} `);
	}
	process.stdout.write(`| `);
	for (var index = 4; index < 8; index++) {
		process.stdout.write(`${flags1[index]} `);
	}
	process.stdout.write(`| `);
	for (var index = 8; index < 12; index++) {
		process.stdout.write(`${flags1[index]} `);
	}
	process.stdout.write(`| `);
	for (var index = 12; index < 16; index++) {
		process.stdout.write(`${flags1[index]} `);
	}
	process.stdout.write(`]`);
	process.stdout.write(`\n\n`);

	process.stdout.write(`\t| FLAGS2: ------ `);
	process.stdout.write(`[ `);
	for (var index = 0; index < 4; index++) {
		process.stdout.write(`${flags2[index]} `);
	}
	process.stdout.write(`| `);
	for (var index = 4; index < 8; index++) {
		process.stdout.write(`${flags2[index]} `);
	}
	process.stdout.write(`| `);
	for (var index = 8; index < 12; index++) {
		process.stdout.write(`${flags2[index]} `);
	}
	process.stdout.write(`| `);
	for (var index = 12; index < 16; index++) {
		process.stdout.write(`${flags2[index]} `);
	}
	process.stdout.write(`]`);
	process.stdout.write(`\n\n`);

	} catch {
		console.log(Error);
	}
};

ListGlobal();



