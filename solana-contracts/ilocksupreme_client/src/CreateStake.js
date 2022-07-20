"use strict";
/****************************************************************
 * ILOCKsupreme client ProgramInit				*
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
var BN = require("bn.js");
/****************************************************************
 * main								*
 ****************************************************************/
var CreateUser = function () { return __awaiter(void 0, void 0, void 0, function () {
    var programID, ownerVault, _a, pdaGLOBAL, bumpGLOBAL, _b, pdaUSER, bumpUSER, USER, countSTAKE, valence, pdaSTAKEseed, _c, pdaSTAKE, bumpSTAKE, amount, ixDATA, CreateUSERtx, _d, _e, _f, _g;
    return __generator(this, function (_h) {
        switch (_h.label) {
            case 0:
                _h.trys.push([0, 9, , 10]);
                // setup
                return [4 /*yield*/, (0, utils_1.establishConnection)()];
            case 1:
                // setup
                _h.sent();
                return [4 /*yield*/, (0, utils_1.establishOperator)()];
            case 2:
                _h.sent();
                return [4 /*yield*/, (0, utils_1.checkProgram)()];
            case 3:
                _h.sent();
                programID = "InterlockSupremeAccount";
                ownerVault = prompt("Please enter your Ethereum vault address: ");
                return [4 /*yield*/, (0, utils_1.deriveAddress)((0, utils_1.toUTF8Array)(programID))];
            case 4:
                _a = _h.sent(), pdaGLOBAL = _a[0], bumpGLOBAL = _a[1];
                console.log(". New GLOBAL pda:\t\t".concat(pdaGLOBAL.toBase58(), " found after ").concat(256 - bumpGLOBAL, " tries"));
                return [4 /*yield*/, (0, utils_1.deriveAddress)((0, utils_1.toUTF8Array)(ownerVault))];
            case 5:
                _b = _h.sent(), pdaUSER = _b[0], bumpUSER = _b[1];
                console.log(". New USER pda:\t\t".concat(pdaUSER.toBase58(), " found after ").concat(256 - bumpUSER, " tries"));
                return [4 /*yield*/, (0, utils_1.getUSERdata)(pdaUSER)];
            case 6:
                USER = _h.sent();
                countSTAKE = new Uint16Array(1);
                countSTAKE[0] = USER.count + 1;
                console.log(". This will be STAKE number ".concat(countSTAKE[0], "."));
                valence = new Uint8Array(1);
                valence = prompt("Please enter '1' if this entity is good, or '0' if it is bad: ");
                pdaSTAKEseed = (0, utils_1.createSeed)(pdaUSER, countSTAKE);
                return [4 /*yield*/, (0, utils_1.deriveAddress)(pdaSTAKEseed)];
            case 7:
                _c = _h.sent(), pdaSTAKE = _c[0], bumpSTAKE = _c[1];
                amount = prompt("Please enter the amount you wish to stake: ");
                ixDATA = [4, bumpSTAKE]
                    .concat(pdaSTAKEseed)
                    .concat(new BN(amount).toArray("le", 16))
                    .concat([valence[0]]);
                CreateUSERtx = new web3_js_1.Transaction().add(new web3_js_1.TransactionInstruction({
                    keys: [
                        { pubkey: utils_2.ownerKEY.publicKey, isSigner: true, isWritable: true },
                        { pubkey: pdaGLOBAL, isSigner: false, isWritable: true },
                        { pubkey: pdaUSER, isSigner: false, isWritable: true },
                        { pubkey: web3_js_1.SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
                        { pubkey: web3_js_1.SystemProgram.programId, isSigner: false, isWritable: false },
                    ],
                    data: Buffer.from(new Uint8Array(ixDATA)),
                    programId: utils_2.ilocksupremeID
                }));
                // send transaction
                _e = (_d = console).log;
                _f = "txhash: ".concat;
                return [4 /*yield*/, (0, web3_js_1.sendAndConfirmTransaction)(utils_2.connection, CreateUSERtx, [utils_2.ownerKEY])];
            case 8:
                // send transaction
                _e.apply(_d, [_f.apply("txhash: ", [_h.sent()])]);
                // confirmation
                console.log("\n* Successfully created new GLOBAL account for '".concat(programID, "'!\n"));
                return [3 /*break*/, 10];
            case 9:
                _g = _h.sent();
                console.log(Error);
                return [3 /*break*/, 10];
            case 10: return [2 /*return*/];
        }
    });
}); };
CreateUser();
