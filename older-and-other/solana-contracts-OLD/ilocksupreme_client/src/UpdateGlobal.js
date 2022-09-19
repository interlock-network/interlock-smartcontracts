"use strict";
/****************************************************************
 * ILOCKsupreme client UpdateGlobal				*
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
/****************************************************************
 * imports							*
 ****************************************************************/
// misc packages
var prompt = require("prompt-sync")({ sigint: true });
// misc solana
var web3_js_1 = require("@solana/web3.js");
// utility functions
var utils_1 = require("./utils");
// utility constants
var utils_2 = require("./utils");
/****************************************************************
 * main								*
 ****************************************************************/
var UpdateGlobal = function () { return __awaiter(void 0, void 0, void 0, function () {
    var programID, _a, pdaGLOBAL, bumpGLOBAL, updateFlagsHigh, updateFlagsLow, updateValues, ixDATA, i, UpdateGLOBALtx, _b, _c, _d, _e;
    return __generator(this, function (_f) {
        switch (_f.label) {
            case 0:
                _f.trys.push([0, 6, , 7]);
                // setup
                return [4 /*yield*/, (0, utils_1.establishConnection)()];
            case 1:
                // setup
                _f.sent();
                return [4 /*yield*/, (0, utils_1.establishOperator)()];
            case 2:
                _f.sent();
                return [4 /*yield*/, (0, utils_1.checkProgram)()];
            case 3:
                _f.sent();
                programID = "InterlockSupremeAccount";
                return [4 /*yield*/, (0, utils_1.deriveAddress)((0, utils_1.toUTF8Array)(programID))];
            case 4:
                _a = _f.sent(), pdaGLOBAL = _a[0], bumpGLOBAL = _a[1];
                console.log(". GLOBAL pda:\t\t".concat(pdaGLOBAL.toBase58(), " found after ").concat(256 - bumpGLOBAL, " tries"));
                updateFlagsHigh = (0, utils_1.unpackFlags32)(0);
                updateFlagsLow = (0, utils_1.unpackFlags32)(0);
                updateValues = new Uint32Array(64);
                // values and their flags, 64
                // 0: entity total stake threshold
                updateFlagsHigh[0] = 0;
                updateValues[0] = 0;
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
                updateFlagsHigh[10] = 0;
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
                updateFlagsHigh[22] = 0;
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
                updateFlagsHigh[29] = 0;
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
                ixDATA = [1]
                    .concat((0, utils_1.packFlags32)(updateFlagsHigh))
                    .concat((0, utils_1.packFlags32)(updateFlagsLow));
                for (i = 0; i < 64; i++) {
                    //ixDATA.concat(u32toBytes(updateValues[i]));
                    ixDATA.concat([0]);
                }
                UpdateGLOBALtx = new web3_js_1.Transaction().add(new web3_js_1.TransactionInstruction({
                    keys: [
                        { pubkey: utils_2.ownerKEY.publicKey, isSigner: true, isWritable: true },
                        { pubkey: pdaGLOBAL, isSigner: false, isWritable: true },
                        // new owner below, if needed
                        { pubkey: utils_2.ownerKEY.publicKey, isSigner: false, isWritable: true },
                        { pubkey: web3_js_1.SystemProgram.programId, isSigner: false, isWritable: false },
                    ],
                    data: Buffer.from(new Uint8Array(ixDATA)),
                    programId: utils_2.ilocksupremeID
                }));
                console.log("chirp");
                // send transaction
                _c = (_b = console).log;
                _d = "txhash: ".concat;
                return [4 /*yield*/, (0, web3_js_1.sendAndConfirmTransaction)(utils_2.connection, UpdateGLOBALtx, [utils_2.ownerKEY])];
            case 5:
                // send transaction
                _c.apply(_b, [_d.apply("txhash: ", [_f.sent()])]);
                // confirmation
                console.log("\n* Successfully updated GLOBAL account for '".concat(programID, "'!\n"));
                return [3 /*break*/, 7];
            case 6:
                _e = _f.sent();
                console.log(Error);
                return [3 /*break*/, 7];
            case 7: return [2 /*return*/];
        }
    });
}); };
UpdateGlobal();
