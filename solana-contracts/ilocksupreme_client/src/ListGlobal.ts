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
	var GLOBALflags1 = unpackFlags(GLOBAL.flags1);
	var GLOBALflags2 = unpackFlags(GLOBAL.flags2);

	// print STAKE data
	console.log(`\t| ADDRESS: ----- ${GLOBE[0].pubkey}`);
	console.log(`\t| POOL: -------- ${GLOBAL.pool}`);
	console.log(`\t| OWNER: ------- ${GLOBAL.owner}`);
	console.log('')
	console.log(`\t| VALUE0: ------ ${GLOBAL.value0}`);
	console.log(`\t| VALUE1: ------ ${GLOBAL.value1}`);
	console.log(`\t| VALUE2: ------ ${GLOBAL.value2}`);
	console.log(`\t| VALUE3: ------ ${GLOBAL.value3}`);
	console.log(`\t| VALUE4: ------ ${GLOBAL.value4}`);
	console.log(`\t| VALUE5: ------ ${GLOBAL.value5}`);
	console.log(`\t| VALUE6: ------ ${GLOBAL.value6}`);
	console.log(`\t| VALUE7: ------ ${GLOBAL.value7}`);
	console.log(`\t| VALUE8: ------ ${GLOBAL.value8}`);
	console.log(`\t| VALUE9: ------ ${GLOBAL.value9}`);
	console.log('')
	console.log(`\t| VALUE10: ----- ${GLOBAL.value10}`);
	console.log(`\t| VALUE11: ----- ${GLOBAL.value11}`);
	console.log(`\t| VALUE12: ----- ${GLOBAL.value12}`);
	console.log(`\t| VALUE13: ----- ${GLOBAL.value13}`);
	console.log(`\t| VALUE14: ----- ${GLOBAL.value14}`);
	console.log(`\t| VALUE15: ----- ${GLOBAL.value15}`);
	console.log(`\t| VALUE16: ----- ${GLOBAL.value16}`);
	console.log(`\t| VALUE17: ----- ${GLOBAL.value17}`);
	console.log(`\t| VALUE18: ----- ${GLOBAL.value18}`);
	console.log(`\t| VALUE19: ----- ${GLOBAL.value19}`);
	console.log('')
	console.log(`\t| VALUE20: ----- ${GLOBAL.value20}`);
	console.log(`\t| VALUE21: ----- ${GLOBAL.value21}`);
	console.log(`\t| VALUE22: ----- ${GLOBAL.value22}`);
	console.log(`\t| VALUE23: ----- ${GLOBAL.value23}`);
	console.log(`\t| VALUE24: ----- ${GLOBAL.value24}`);
	console.log(`\t| VALUE25: ----- ${GLOBAL.value25}`);
	console.log(`\t| VALUE26: ----- ${GLOBAL.value26}`);
	console.log(`\t| VALUE27: ----- ${GLOBAL.value27}`);
	console.log(`\t| VALUE28: ----- ${GLOBAL.value28}`);
	console.log(`\t| VALUE29: ----- ${GLOBAL.value29}`);
	console.log('')
	console.log(`\t| VALUE30: ----- ${GLOBAL.value30}`);
	console.log(`\t| VALUE31: ----- ${GLOBAL.value31}`);
	console.log(`\t| VALUE32: ----- ${GLOBAL.value32}`);
	console.log(`\t| VALUE33: ----- ${GLOBAL.value33}`);
	console.log(`\t| VALUE34: ----- ${GLOBAL.value34}`);
	console.log(`\t| VALUE35: ----- ${GLOBAL.value35}`);
	console.log(`\t| VALUE36: ----- ${GLOBAL.value36}`);
	console.log(`\t| VALUE37: ----- ${GLOBAL.value37}`);
	console.log(`\t| VALUE38: ----- ${GLOBAL.value38}`);
	console.log(`\t| VALUE39: ----- ${GLOBAL.value39}`);
	console.log('')
	console.log(`\t| VALUE40: ----- ${GLOBAL.value40}`);
	console.log(`\t| VALUE41: ----- ${GLOBAL.value41}`);
	console.log(`\t| VALUE42: ----- ${GLOBAL.value42}`);
	console.log(`\t| VALUE43: ----- ${GLOBAL.value43}`);
	console.log(`\t| VALUE44: ----- ${GLOBAL.value44}`);
	console.log(`\t| VALUE45: ----- ${GLOBAL.value45}`);
	console.log(`\t| VALUE46: ----- ${GLOBAL.value46}`);
	console.log(`\t| VALUE47: ----- ${GLOBAL.value47}`);
	console.log(`\t| VALUE48: ----- ${GLOBAL.value48}`);
	console.log(`\t| VALUE49: ----- ${GLOBAL.value49}`);
	console.log('')
	console.log(`\t| VALUE50: ----- ${GLOBAL.value50}`);
	console.log(`\t| VALUE51: ----- ${GLOBAL.value51}`);
	console.log(`\t| VALUE52: ----- ${GLOBAL.value52}`);
	console.log(`\t| VALUE53: ----- ${GLOBAL.value53}`);
	console.log(`\t| VALUE54: ----- ${GLOBAL.value54}`);
	console.log(`\t| VALUE55: ----- ${GLOBAL.value55}`);
	console.log(`\t| VALUE56: ----- ${GLOBAL.value56}`);
	console.log(`\t| VALUE57: ----- ${GLOBAL.value57}`);
	console.log(`\t| VALUE58: ----- ${GLOBAL.value58}`);
	console.log(`\t| VALUE59: ----- ${GLOBAL.value59}`);
	console.log('')
	console.log(`\t| VALUE60: ----- ${GLOBAL.value60}`);
	console.log(`\t| VALUE61: ----- ${GLOBAL.value61}`);
	console.log(`\t| VALUE62: ----- ${GLOBAL.value62}`);
	console.log(`\t| VALUE63: ----- ${GLOBAL.value63}`);

	process.stdout.write(`\n\t| FLAGS1: ------- `);
	for (var index = 0; index < 16; index++) {
		process.stdout.write(`${GLOBALflags1[index]}  `);
	}
	console.log("\n\t                  0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15") 
	process.stdout.write(`\n`);

	process.stdout.write(`\t| FLAGS2: ------- `);
	for (var index = 0; index < 16; index++) {
		process.stdout.write(`${GLOBALflags2[index]}  `);
	}
	console.log("\n\t                  0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15") 
	process.stdout.write(`\n\n`);


	} catch {
		console.log(Error);
	}
};

ListGlobal();



