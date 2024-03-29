"use strict";
/****************************************************************
 * ILOCKsupreme client utility blob				*
 ****************************************************************/
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
exports.__esModule = true;
exports.newURLhash = exports.newKeyhash = exports.toUTF8Array = exports.fromUTF8Array = exports.getProgramID = exports.getKeypair = exports.establishOperator = exports.establishConnection = exports.checkProgram = exports.getGLOBAL = exports.getUSERs = exports.getENTITYs = exports.getSTAKEs = exports.deriveAddress = exports.u32toBytes = exports.createSeed = exports.packFlags32 = exports.unpackFlags32 = exports.unpackFlags = exports.templateFlagCheck = exports.ENTITY_DATA_LAYOUT = exports.STAKE_DATA_LAYOUT = exports.USER_DATA_LAYOUT = exports.GLOBAL_DATA_LAYOUT = exports.getENTITYdata = exports.getSTAKEdata = exports.getUSERdata = exports.getGLOBALdata = exports.PROGRAM_KEYPAIR_PATH = exports.PROGRAM_PATH = exports.PROGRAM_KEYFILE = exports.ilocksupremeID = exports.ownerKEY = exports.connection = exports.ENTITY_SIZE = exports.STAKE_SIZE = exports.USER_SIZE = exports.GLOBAL_SIZE = exports.VALUES_SIZE = exports.VALUE_NUMBER = exports.PUBKEY_SIZE = exports.U128_SIZE = exports.U64_SIZE = exports.U32_SIZE = exports.U16_SIZE = void 0;
/****************************************************************
 * imports							*
 ****************************************************************/
var web3_js_1 = require("@solana/web3.js");
var os = require("os");
var fs = require("mz/fs");
var path = require("path");
var yaml = require("yaml");
var BufferLayout = require("buffer-layout");
var BigNumber = require("bignumber.js");
var BN = require("bn.js");
var bs58 = require("bs58");
var lodash = require("lodash");
var crypto = require('crypto-js');
/****************************************************************
 * declare constants						*
 ****************************************************************/
exports.U16_SIZE = 2;
exports.U32_SIZE = 4;
exports.U64_SIZE = 8;
exports.U128_SIZE = 16;
exports.PUBKEY_SIZE = 32;
exports.VALUE_NUMBER = 64;
exports.VALUES_SIZE = exports.U32_SIZE * exports.VALUE_NUMBER;
exports.GLOBAL_SIZE = exports.U128_SIZE + // pool
    exports.U32_SIZE + // flags
    exports.PUBKEY_SIZE + // owner
    exports.VALUES_SIZE; // values
// = 292
exports.USER_SIZE = exports.U16_SIZE + // flags
    exports.U16_SIZE + // count
    exports.U16_SIZE + // success
    exports.U16_SIZE + // fail
    exports.PUBKEY_SIZE + // owner
    exports.PUBKEY_SIZE + // vault
    exports.U128_SIZE + // balance
    exports.U128_SIZE; // rewards
// = 104
exports.STAKE_SIZE = exports.U16_SIZE + // flags
    exports.U64_SIZE + // timestamp
    exports.PUBKEY_SIZE + // entity
    exports.U128_SIZE; // amount
// = 58
exports.ENTITY_SIZE = exports.U16_SIZE + // flags
    exports.PUBKEY_SIZE + // hunter
    exports.U128_SIZE + // stakepos
    exports.U128_SIZE + // stakeneg
    exports.U16_SIZE + // stakers
    exports.U64_SIZE; // timestamp
exports.PROGRAM_KEYFILE = "ILOCKsupreme-keypair.json";
exports.PROGRAM_PATH = path.resolve("../../ilocksupreme/target/deploy");
exports.PROGRAM_KEYPAIR_PATH = path.join(exports.PROGRAM_PATH, exports.PROGRAM_KEYFILE);
/****************************************************************
 * general functions						*
 ****************************************************************/
/**
* get GLOBAL account data
**/
function getGLOBALdata(pdaGLOBAL) {
    return __awaiter(this, void 0, void 0, function () {
        var GLOBALaccount, encodedGLOBALstate, decodedGLOBALstate;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, exports.connection.getAccountInfo(pdaGLOBAL)];
                case 1:
                    GLOBALaccount = _a.sent();
                    if (GLOBALaccount === null || GLOBALaccount.data.length === 0) {
                        console.log("! GLOBAL account for this owner ID has not been created.");
                        process.exit(1);
                    }
                    encodedGLOBALstate = GLOBALaccount.data;
                    decodedGLOBALstate = exports.GLOBAL_DATA_LAYOUT.decode(encodedGLOBALstate);
                    return [2 /*return*/, {
                            pool: new BigNumber("0x" + decodedGLOBALstate.pool.toString("hex")),
                            flags1: decodedGLOBALstate.flags1,
                            flags2: decodedGLOBALstate.flags2,
                            owner: new web3_js_1.PublicKey(decodedGLOBALstate.owner),
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
                            value63: decodedGLOBALstate.value63
                        }];
            }
        });
    });
}
exports.getGLOBALdata = getGLOBALdata;
/**
* get USER account data
**/
function getUSERdata(pdaUSER) {
    return __awaiter(this, void 0, void 0, function () {
        var USERaccount, encodedUSERstate, decodedUSERstate;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, exports.connection.getAccountInfo(pdaUSER)];
                case 1:
                    USERaccount = _a.sent();
                    if (USERaccount === null || USERaccount.data.length === 0) {
                        console.log("! This USER account has not been created.");
                        process.exit(1);
                    }
                    encodedUSERstate = USERaccount.data;
                    decodedUSERstate = exports.USER_DATA_LAYOUT.decode(encodedUSERstate);
                    return [2 /*return*/, {
                            flags: decodedUSERstate.flags,
                            count: decodedUSERstate.count,
                            success: decodedUSERstate.success,
                            fail: decodedUSERstate.fail,
                            owner: new web3_js_1.PublicKey(decodedUSERstate.owner),
                            vault: new web3_js_1.PublicKey(decodedUSERstate.vault),
                            balance: new BigNumber("0x" + decodedUSERstate.balance.toString("hex")),
                            rewards: new BigNumber("0x" + decodedUSERstate.rewards.toString("hex"))
                        }];
            }
        });
    });
}
exports.getUSERdata = getUSERdata;
/**
* get STAKE account data
**/
function getSTAKEdata(pdaSTAKE) {
    return __awaiter(this, void 0, void 0, function () {
        var STAKEaccount, encodedSTAKEstate, decodedSTAKEstate;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, exports.connection.getAccountInfo(pdaSTAKE)];
                case 1:
                    STAKEaccount = _a.sent();
                    if (STAKEaccount === null || STAKEaccount.data.length === 0) {
                        console.log("! This STAKE account has not been created.");
                        process.exit(1);
                    }
                    encodedSTAKEstate = STAKEaccount.data;
                    decodedSTAKEstate = exports.STAKE_DATA_LAYOUT.decode(encodedSTAKEstate);
                    return [2 /*return*/, {
                            flags: decodedSTAKEstate.flags,
                            timestamp: new BigNumber("0x" + decodedSTAKEstate.timestamp.toString("hex")),
                            entity: new web3_js_1.PublicKey(decodedSTAKEstate.entity),
                            amount: new BigNumber("0x" + decodedSTAKEstate.amount.toString("hex"))
                        }];
            }
        });
    });
}
exports.getSTAKEdata = getSTAKEdata;
/**
* get ENTITY account data
**/
function getENTITYdata(pdaENTITY) {
    return __awaiter(this, void 0, void 0, function () {
        var ENTITYaccount, encodedENTITYstate, decodedENTITYstate;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, exports.connection.getAccountInfo(pdaENTITY)];
                case 1:
                    ENTITYaccount = _a.sent();
                    if (ENTITYaccount === null || ENTITYaccount.data.length === 0) {
                        console.log("! This ENTITY account has not been created.");
                        process.exit(1);
                    }
                    encodedENTITYstate = ENTITYaccount.data;
                    decodedENTITYstate = exports.ENTITY_DATA_LAYOUT.decode(encodedENTITYstate);
                    return [2 /*return*/, {
                            flags: decodedENTITYstate.flags,
                            hunter: new web3_js_1.PublicKey(decodedENTITYstate.hunter),
                            stakepos: new BigNumber("0x" + decodedENTITYstate.stakepos.toString("hex")),
                            stakeneg: new BigNumber("0x" + decodedENTITYstate.stakeneg.toString("hex")),
                            stakers: decodedENTITYstate.stakers,
                            timestamp: new BigNumber("0x" + decodedENTITYstate.timestamp.toString("hex"))
                        }];
            }
        });
    });
}
exports.getENTITYdata = getENTITYdata;
/****************************************************************
 * setup layouts and interfaces					*
 ****************************************************************/
/**
 * flags layout
 **/
var flags = function (property) {
    if (property === void 0) { property = "flags"; }
    return BufferLayout.blob(2, property);
};
/**
 * public key layout
 **/
var publicKey = function (property) {
    if (property === void 0) { property = "publicKey"; }
    return BufferLayout.blob(32, property);
};
/**
 * u64 layout
 **/
var uint64 = function (property) {
    if (property === void 0) { property = "uint64"; }
    return BufferLayout.blob(8, property);
};
/**
 * u128 layout
 **/
var uint128 = function (property) {
    if (property === void 0) { property = "uint128"; }
    return BufferLayout.blob(16, property);
};
/**
 * account struct GLOBAL
 **/
exports.GLOBAL_DATA_LAYOUT = BufferLayout.struct([
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
/**
 * account struct USER
 **/
exports.USER_DATA_LAYOUT = BufferLayout.struct([
    BufferLayout.u16("flags"),
    BufferLayout.u16("count"),
    BufferLayout.u16("success"),
    BufferLayout.u16("fail"),
    publicKey("owner"),
    publicKey("vault"),
    uint128("balance"),
    uint128("rewards"),
]);
/**
 * account struct STAKE
 **/
exports.STAKE_DATA_LAYOUT = BufferLayout.struct([
    BufferLayout.u16("flags"),
    uint64("timestamp"),
    publicKey("entity"),
    uint128("amount"),
]);
/**
 * account struct ENTITY
 **/
exports.ENTITY_DATA_LAYOUT = BufferLayout.struct([
    BufferLayout.u16("flags"),
    publicKey("hunter"),
    uint128("stakepos"),
    uint128("stakeneg"),
    BufferLayout.u16("stakers"),
    uint64("timestamp"),
]);
/**
* check flag template
**/
function templateFlagCheck(flags) {
    var flagarray = unpackFlags(flags);
    return flagarray[0] === 1;
}
exports.templateFlagCheck = templateFlagCheck;
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
function unpackFlags(flags) {
    var highflags = flags >> 8;
    var lowflags = flags & 0xFF;
    var bitarray = new Uint8Array(16);
    for (var index = 0; index < 8; index++) {
        bitarray[index] = (highflags >> (7 - index)) & 0x01;
    }
    for (index = 0; index < 8; index++) {
        bitarray[8 + index] = (lowflags >> (7 - index)) & 0x01;
    }
    return bitarray;
}
exports.unpackFlags = unpackFlags;
/**
* unpack flags 32
**/
function unpackFlags32(flags) {
    var flags1 = (flags >> 24) & 0xFF;
    var flags2 = (flags >> 16) & 0xFF;
    var flags3 = (flags >> 8) & 0xFF;
    var flags4 = flags & 0xFF;
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
    return bitarray;
}
exports.unpackFlags32 = unpackFlags32;
/**
* pack flags 32
**/
function packFlags32(flags) {
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
    return [byte4, byte3, byte2, byte1];
}
exports.packFlags32 = packFlags32;
/**
* create pda seed
**/
function createSeed(pda, count) {
    var countLow = count[0] & 0xFF; // mask for low order count byte
    var countHigh = (count[0] >> 8) & 0xFF; // shift and mask for high order count byte
    return toUTF8Array(pda
        .toString()
        .slice(0, exports.PUBKEY_SIZE - exports.U16_SIZE))
        .concat(countHigh, countLow);
}
exports.createSeed = createSeed;
/**
* u32 to bytes
**/
function u32toBytes(number) {
    var byte1 = number & 0xFF; // mask for lowest order number byte
    var byte2 = (number >> 8) & 0xFF; // shift and mask for next lowest order number byte
    var byte3 = (number >> 16) & 0xFF; // shift and mask for high order number byte
    var byte4 = (number >> 24) & 0xFF; // shift and mask for highest order number byte
    return [byte4, byte3, byte2, byte1];
}
exports.u32toBytes = u32toBytes;
/**
* derive pda
**/
function deriveAddress(seed) {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, web3_js_1.PublicKey.findProgramAddress([new Uint8Array(seed)], exports.ilocksupremeID)];
                case 1: return [2 /*return*/, _a.sent()];
            }
        });
    });
}
exports.deriveAddress = deriveAddress;
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
function getSTAKEs(hash) {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, exports.connection.getParsedProgramAccounts(exports.ilocksupremeID, {
                        filters: [
                            {
                                dataSize: exports.STAKE_SIZE
                            },
                            {
                                memcmp: {
                                    offset: exports.U16_SIZE + exports.U64_SIZE,
                                    bytes: hash
                                }
                            },
                        ]
                    })];
                case 1: return [2 /*return*/, _a.sent()];
            }
        });
    });
}
exports.getSTAKEs = getSTAKEs;
/**
* get all ENTITYs
***/
function getENTITYs() {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, exports.connection.getParsedProgramAccounts(exports.ilocksupremeID, {
                        filters: [
                            {
                                dataSize: exports.ENTITY_SIZE
                            },
                            {
                                memcmp: {
                                    offset: 0,
                                    bytes: ""
                                }
                            },
                        ]
                    })];
                case 1: return [2 /*return*/, _a.sent()];
            }
        });
    });
}
exports.getENTITYs = getENTITYs;
/**
* get all USERs
***/
function getUSERs() {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, exports.connection.getParsedProgramAccounts(exports.ilocksupremeID, {
                        filters: [
                            {
                                dataSize: exports.USER_SIZE
                            },
                            {
                                memcmp: {
                                    offset: 0,
                                    bytes: ""
                                }
                            },
                        ]
                    })];
                case 1: return [2 /*return*/, _a.sent()];
            }
        });
    });
}
exports.getUSERs = getUSERs;
/**
* get GLOBAL
***/
function getGLOBAL() {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, exports.connection.getParsedProgramAccounts(exports.ilocksupremeID, {
                        filters: [
                            {
                                dataSize: exports.GLOBAL_SIZE
                            },
                            {
                                memcmp: {
                                    offset: 0,
                                    bytes: ""
                                }
                            },
                        ]
                    })];
                case 1: return [2 /*return*/, _a.sent()];
            }
        });
    });
}
exports.getGLOBAL = getGLOBAL;
/**
* Check if the hello world BPF program has been deployed
**/
function checkProgram() {
    return __awaiter(this, void 0, void 0, function () {
        var programKeypair, err_1, errMsg;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    _a.trys.push([0, 2, , 3]);
                    return [4 /*yield*/, createKeypairFromFile(exports.PROGRAM_KEYPAIR_PATH)];
                case 1:
                    programKeypair = _a.sent();
                    exports.ilocksupremeID = programKeypair.publicKey;
                    console.log(". ILOCKsupreme found at:\t".concat(exports.ilocksupremeID.toBase58()));
                    return [3 /*break*/, 3];
                case 2:
                    err_1 = _a.sent();
                    errMsg = err_1.message;
                    throw new Error("! Failed to read program keypair at \"".concat(exports.PROGRAM_KEYPAIR_PATH, "\" due to error: ").concat(errMsg, ".\n\n\t\t\tProgram may need to be deployed with \n\t\t\t`solana program deploy fracpay_server/target/deploy/fracpay_server.so`"));
                case 3: return [2 /*return*/];
            }
        });
    });
}
exports.checkProgram = checkProgram;
/**
 * establish connection
 **/
function establishConnection() {
    return __awaiter(this, void 0, void 0, function () {
        var rpcUrl, version;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, getRpcUrl()];
                case 1:
                    rpcUrl = _a.sent();
                    exports.connection = new web3_js_1.Connection(rpcUrl, "confirmed");
                    return [4 /*yield*/, exports.connection.getVersion()];
                case 2:
                    version = _a.sent();
                    console.log(". Connection to cluster established:", rpcUrl, version);
                    return [2 /*return*/];
            }
        });
    });
}
exports.establishConnection = establishConnection;
function getRpcUrl() {
    return __awaiter(this, void 0, void 0, function () {
        var config, err_2;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    _a.trys.push([0, 2, , 3]);
                    return [4 /*yield*/, getConfig()];
                case 1:
                    config = _a.sent();
                    if (!config.json_rpc_url)
                        throw new Error("Missing RPC URL");
                    return [2 /*return*/, config.json_rpc_url];
                case 2:
                    err_2 = _a.sent();
                    console.warn("! Failed to read RPC url from CLI config file, falling back to localhost");
                    return [2 /*return*/, "http://localhost:8899"];
                case 3: return [2 /*return*/];
            }
        });
    });
}
/**
 * get owner's local solana config
 **/
function getConfig() {
    return __awaiter(this, void 0, void 0, function () {
        var CONFIG_FILE_PATH, configYml;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    CONFIG_FILE_PATH = path.resolve(os.homedir(), ".config", "solana", "cli", "config.yml");
                    return [4 /*yield*/, fs.readFile(CONFIG_FILE_PATH, { encoding: "utf8" })];
                case 1:
                    configYml = _a.sent();
                    return [2 /*return*/, yaml.parse(configYml)];
            }
        });
    });
}
/**
 * establish owner
 **/
function establishOperator() {
    return __awaiter(this, void 0, void 0, function () {
        var fees, feeCalculator, _a, lamports;
        return __generator(this, function (_b) {
            switch (_b.label) {
                case 0:
                    fees = 0;
                    if (!!exports.ownerKEY) return [3 /*break*/, 4];
                    return [4 /*yield*/, exports.connection.getRecentBlockhash()];
                case 1:
                    feeCalculator = (_b.sent()).feeCalculator;
                    // Calculate the cost to fund the greeter account
                    _a = fees;
                    return [4 /*yield*/, exports.connection.getMinimumBalanceForRentExemption(exports.GLOBAL_SIZE)];
                case 2:
                    // Calculate the cost to fund the greeter account
                    fees = _a + _b.sent();
                    // Calculate the cost of sending transactions
                    fees += feeCalculator.lamportsPerSignature * 100; // wag
                    return [4 /*yield*/, getOperator()];
                case 3:
                    exports.ownerKEY = _b.sent();
                    _b.label = 4;
                case 4: return [4 /*yield*/, exports.connection.getBalance(exports.ownerKEY.publicKey)];
                case 5:
                    lamports = _b.sent();
                    if (lamports < fees) {
                        // If current balance is not enough to pay for fees, request an airdrop
                        console.log("! Unfortunately you do not have enough SOL to initialize an account.\n", "  You need ".concat(fees / web3_js_1.LAMPORTS_PER_SOL, " SOL to initialize account."));
                    }
                    console.log(". Operator account is:\t", exports.ownerKEY.publicKey.toBase58(), "containing", lamports / web3_js_1.LAMPORTS_PER_SOL, "SOL to pay for fees");
                    return [2 /*return*/];
            }
        });
    });
}
exports.establishOperator = establishOperator;
/**
 * setup ownerKEY as Keypair
 **/
function getOperator() {
    return __awaiter(this, void 0, void 0, function () {
        var config, err_3;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    _a.trys.push([0, 3, , 4]);
                    return [4 /*yield*/, getConfig()];
                case 1:
                    config = _a.sent();
                    if (!config.keypair_path)
                        throw new Error("Missing keypair path");
                    return [4 /*yield*/, createKeypairFromFile(config.keypair_path)];
                case 2: return [2 /*return*/, _a.sent()];
                case 3:
                    err_3 = _a.sent();
                    console.warn("! Failed to create keypair from CLI config file, falling back to new random keypair");
                    return [2 /*return*/, web3_js_1.Keypair.generate()];
                case 4: return [2 /*return*/];
            }
        });
    });
}
/**
 * read secret key from file and return Keypair object
 **/
function createKeypairFromFile(filePath) {
    return __awaiter(this, void 0, void 0, function () {
        var secretKeyString, secretKey;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, fs.readFile(filePath, { encoding: "utf8" })];
                case 1:
                    secretKeyString = _a.sent();
                    secretKey = Uint8Array.from(JSON.parse(secretKeyString));
                    return [2 /*return*/, web3_js_1.Keypair.fromSecretKey(secretKey)];
            }
        });
    });
}
/**
 * return private key from 64 byte array in file
 **/
var getPrivateKey = function (name) {
    return Uint8Array.from(JSON.parse(fs.readFileSync("./keys/".concat(name, "_pri.json"))));
};
/**
 * return public key from base58 formatted string in file
 **/
var getPublicKey = function (name) {
    return new web3_js_1.PublicKey(JSON.parse(fs.readFileSync("./keys/".concat(name, "_pub.json"))));
};
/**
 * write a public key to file [presumably hex string, haven't checked yet]
 **/
var writePublicKey = function (publicKey, name) {
    fs.writeFileSync("./keys/".concat(name, "_pub.json"), JSON.stringify(publicKey.toString()));
};
/**
 * creates Keypair object from named pubkey prikey json files
 **/
var getKeypair = function (name) {
    return new web3_js_1.Keypair({
        publicKey: new Uint8Array(getPublicKey(name).toBytes()),
        secretKey: getPrivateKey(name)
    });
};
exports.getKeypair = getKeypair;
/**
 * read fracpay program ID from json file in keys directory
 **/
var getProgramID = function () {
    try {
        return getPublicKey("ILOCKsupreme");
    }
    catch (error) {
        console.log("Given programId is missing or incorrect");
        process.exit(1);
    }
};
exports.getProgramID = getProgramID;
/**
 * take in a UTF8 array and turn it into a string
 **/
function fromUTF8Array(data) {
    var str = "";
    var i;
    for (i = 0; i < data.length; i++) {
        var value = data[i];
        if (value < 0x80) {
            str += String.fromCharCode(value);
        }
        else if (value > 0xBF && value < 0xE0) {
            str += String.fromCharCode((value & 0x1F) << 6 | data[i + 1] & 0x3F);
            i += 1;
        }
        else if (value > 0xDF && value < 0xF0) {
            str += String.fromCharCode((value & 0x0F) << 12 | (data[i + 1] & 0x3F) << 6 | data[i + 2] & 0x3F);
            i += 2;
        }
        else {
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
exports.fromUTF8Array = fromUTF8Array;
/**
 * take in a string and turn it into a UTF8 byte array
 **/
function toUTF8Array(str) {
    var utf8 = [];
    for (var i = 0; i < str.length; i++) {
        var charcode = str.charCodeAt(i);
        if (charcode < 0x80)
            utf8.push(charcode);
        else if (charcode < 0x800) {
            utf8.push(0xc0 | (charcode >> 6), 0x80 | (charcode & 0x3f));
        }
        else if (charcode < 0xd800 || charcode >= 0xe000) {
            utf8.push(0xe0 | (charcode >> 12), 0x80 | ((charcode >> 6) & 0x3f), 0x80 | (charcode & 0x3f));
        }
        // surrogate pair
        else {
            i++;
            charcode = ((charcode & 0x3ff) << 10) | (str.charCodeAt(i) & 0x3ff);
            utf8.push(0xf0 | (charcode >> 18), 0x80 | ((charcode >> 12) & 0x3f), 0x80 | ((charcode >> 6) & 0x3f), 0x80 | (charcode & 0x3f));
        }
    }
    return utf8;
}
exports.toUTF8Array = toUTF8Array;
// MISC FUNCTION HERE
/**
* create keyhash
**/
function newKeyhash() {
    var newkey = new web3_js_1.Keypair();
    var keyhash = crypto.SHA256(newkey.publicKey.toString());
    keyhash = bs58.encode(Buffer.from(keyhash.toString(), 'hex'));
    keyhash = new web3_js_1.PublicKey(keyhash);
    return [newkey.publicKey, keyhash];
}
exports.newKeyhash = newKeyhash;
function newURLhash(URL) {
    var URLhash = crypto.SHA256(URL);
    URLhash = bs58.encode(Buffer.from(URLhash.toString(), 'hex'));
    URLhash = new web3_js_1.PublicKey(URLhash);
    return URLhash;
}
exports.newURLhash = newURLhash;
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
