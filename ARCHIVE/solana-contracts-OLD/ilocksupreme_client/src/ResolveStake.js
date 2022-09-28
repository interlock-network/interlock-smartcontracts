"use strict";
/****************************************************************
 * ILOCKsupreme client CreateStake				*
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
var CreateStake = function () { return __awaiter(void 0, void 0, void 0, function () {
    var programID, ownerVault, _a, pdaGLOBAL, bumpGLOBAL, _b, pdaUSER, bumpUSER, STAKEnumber, pdaSTAKEseed, _c, pdaSTAKE, bumpSTAKE, STAKE, ENTITYhash, _d, pdaENTITY, bumpENTITY, ixDATA, ResolveSTAKEtx, _e, _f, _g, _h;
    return __generator(this, function (_j) {
        switch (_j.label) {
            case 0:
                _j.trys.push([0, 10, , 11]);
                // setup
                return [4 /*yield*/, (0, utils_1.establishConnection)()];
            case 1:
                // setup
                _j.sent();
                return [4 /*yield*/, (0, utils_1.establishOperator)()];
            case 2:
                _j.sent();
                return [4 /*yield*/, (0, utils_1.checkProgram)()];
            case 3:
                _j.sent();
                programID = "InterlockSupremeAccount";
                ownerVault = prompt("Please enter your Ethereum vault address: ");
                return [4 /*yield*/, (0, utils_1.deriveAddress)((0, utils_1.toUTF8Array)(programID))];
            case 4:
                _a = _j.sent(), pdaGLOBAL = _a[0], bumpGLOBAL = _a[1];
                console.log(". GLOBAL pda:\t\t".concat(pdaGLOBAL.toBase58(), " found after ").concat(256 - bumpGLOBAL, " tries"));
                return [4 /*yield*/, (0, utils_1.deriveAddress)((0, utils_1.toUTF8Array)(ownerVault))];
            case 5:
                _b = _j.sent(), pdaUSER = _b[0], bumpUSER = _b[1];
                console.log(". USER pda:\t\t".concat(pdaUSER.toBase58(), " found after ").concat(256 - bumpUSER, " tries"));
                STAKEnumber = new Uint16Array(1);
                STAKEnumber[0] = parseInt(prompt("From the STAKE list, please enter STAKE # that you wish to resolve: "));
                pdaSTAKEseed = (0, utils_1.createSeed)(pdaUSER, STAKEnumber);
                return [4 /*yield*/, (0, utils_1.deriveAddress)(pdaSTAKEseed)];
            case 6:
                _c = _j.sent(), pdaSTAKE = _c[0], bumpSTAKE = _c[1];
                console.log(". STAKE pda:\t\t".concat(pdaSTAKE.toBase58(), " found after ").concat(256 - bumpSTAKE, " tries"));
                return [4 /*yield*/, (0, utils_1.getSTAKEdata)(pdaSTAKE)];
            case 7:
                STAKE = _j.sent();
                ENTITYhash = STAKE.entity.toString();
                return [4 /*yield*/, (0, utils_1.deriveAddress)((0, utils_1.toUTF8Array)(ENTITYhash))];
            case 8:
                _d = _j.sent(), pdaENTITY = _d[0], bumpENTITY = _d[1];
                console.log(". ENTITY pda:\t\t".concat(pdaENTITY.toBase58(), " found after ").concat(256 - bumpENTITY, " tries"));
                ixDATA = [12];
                ResolveSTAKEtx = new web3_js_1.Transaction().add(new web3_js_1.TransactionInstruction({
                    keys: [
                        { pubkey: utils_2.ownerKEY.publicKey, isSigner: true, isWritable: true },
                        { pubkey: pdaGLOBAL, isSigner: false, isWritable: true },
                        { pubkey: pdaUSER, isSigner: false, isWritable: true },
                        { pubkey: pdaSTAKE, isSigner: false, isWritable: true },
                        { pubkey: pdaENTITY, isSigner: false, isWritable: true },
                        { pubkey: web3_js_1.SystemProgram.programId, isSigner: false, isWritable: false },
                    ],
                    data: Buffer.from(new Uint8Array(ixDATA)),
                    programId: utils_2.ilocksupremeID
                }));
                // send transaction
                _f = (_e = console).log;
                _g = "txhash: ".concat;
                return [4 /*yield*/, (0, web3_js_1.sendAndConfirmTransaction)(utils_2.connection, ResolveSTAKEtx, [utils_2.ownerKEY])];
            case 9:
                // send transaction
                _f.apply(_e, [_g.apply("txhash: ", [_j.sent()])]);
                // confirmation
                console.log("\n* Successfully created new STAKE account '".concat(pdaSTAKE.toBase58(), "'!\n"));
                return [3 /*break*/, 11];
            case 10:
                _h = _j.sent();
                console.log(Error);
                return [3 /*break*/, 11];
            case 11: return [2 /*return*/];
        }
    });
}); };
CreateStake();
