"use strict";
/****************************************************************
 * Fracpay client ListEntityStakes
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
 * imports
 ****************************************************************/
var prompt = require("prompt-sync")({ sigint: true });
var utils_1 = require("./utils");
/****************************************************************
 * main
 ****************************************************************/
var ListEntityStakes = function () { return __awaiter(void 0, void 0, void 0, function () {
    var ENTITYhash, _a, pdaENTITY, bumpENTITY, ENTITY, ENTITYflags, index, index, index, index, STAKEs, countSTAKE, STAKE, flags, index, index, index, index, _b;
    return __generator(this, function (_c) {
        switch (_c.label) {
            case 0:
                _c.trys.push([0, 11, , 12]);
                // setup
                return [4 /*yield*/, (0, utils_1.establishConnection)()];
            case 1:
                // setup
                _c.sent();
                return [4 /*yield*/, (0, utils_1.establishOperator)()];
            case 2:
                _c.sent();
                return [4 /*yield*/, (0, utils_1.checkProgram)()];
            case 3:
                _c.sent();
                ENTITYhash = prompt("Please enter the ENTITY hash: ");
                return [4 /*yield*/, (0, utils_1.deriveAddress)((0, utils_1.toUTF8Array)(ENTITYhash))];
            case 4:
                _a = _c.sent(), pdaENTITY = _a[0], bumpENTITY = _a[1];
                console.log(". Operator MAIN pda:\t".concat(pdaENTITY.toBase58(), " found after ").concat(256 - bumpENTITY, " tries"));
                return [4 /*yield*/, (0, utils_1.getENTITYdata)(pdaENTITY)];
            case 5:
                ENTITY = _c.sent();
                // state intention
                console.log("\nENTITY STAKEs:\n");
                ENTITYflags = (0, utils_1.unpackFlags)(ENTITY.flags);
                // print USER data
                console.log("| USER");
                console.log("| ADDRESS: ----- ".concat(pdaENTITY.toBase58()));
                console.log("| HUNTER: ------ ".concat(ENTITY.hunter));
                console.log("| STAKEPOS: ---- ".concat(ENTITY.stakepos));
                console.log("| STAKENEG: ---- ".concat(ENTITY.stakeneg));
                console.log("| STAKERS: ----- ".concat(ENTITY.stakers));
                console.log("| TIMESTAMP: --- ".concat(ENTITY.timestamp));
                process.stdout.write("| FLAGS: ------- ");
                process.stdout.write("[ ");
                for (index = 0; index < 4; index++) {
                    process.stdout.write("".concat(ENTITYflags[index], " "));
                }
                process.stdout.write("| ");
                for (index = 4; index < 8; index++) {
                    process.stdout.write("".concat(ENTITYflags[index], " "));
                }
                process.stdout.write("| ");
                for (index = 8; index < 12; index++) {
                    process.stdout.write("".concat(ENTITYflags[index], " "));
                }
                process.stdout.write("| ");
                for (index = 12; index < 16; index++) {
                    process.stdout.write("".concat(ENTITYflags[index], " "));
                }
                process.stdout.write("]");
                process.stdout.write("\n\n");
                return [4 /*yield*/, (0, utils_1.getSTAKEs)(ENTITYhash)];
            case 6:
                STAKEs = _c.sent();
                // state intention
                console.log("STAKEs:\n");
                countSTAKE = 1;
                _c.label = 7;
            case 7:
                if (!(countSTAKE <= STAKEs.length)) return [3 /*break*/, 10];
                return [4 /*yield*/, (0, utils_1.getSTAKEdata)(STAKEs[countSTAKE].pubkey)];
            case 8:
                STAKE = _c.sent();
                flags = (0, utils_1.unpackFlags)(STAKE.flags);
                // print STAKE data
                console.log("# ".concat(countSTAKE[0], "\t| STAKE ID: ----> ").concat(STAKE.entity));
                console.log("\t| TIMESTAMP: --- ".concat(STAKE.timestamp));
                console.log("\t| ENTITY: ------ ".concat(STAKE.entity));
                console.log("\t| AMOUNT: ------ ".concat(STAKE.amount));
                process.stdout.write("\t| FLAGS: ------- ");
                process.stdout.write("[ ");
                for (index = 0; index < 4; index++) {
                    process.stdout.write("".concat(flags[index], " "));
                }
                process.stdout.write("| ");
                for (index = 4; index < 8; index++) {
                    process.stdout.write("".concat(flags[index], " "));
                }
                process.stdout.write("| ");
                for (index = 8; index < 12; index++) {
                    process.stdout.write("".concat(flags[index], " "));
                }
                process.stdout.write("| ");
                for (index = 12; index < 16; index++) {
                    process.stdout.write("".concat(flags[index], " "));
                }
                process.stdout.write("]");
                process.stdout.write("\n\n");
                _c.label = 9;
            case 9:
                countSTAKE++;
                return [3 /*break*/, 7];
            case 10: return [3 /*break*/, 12];
            case 11:
                _b = _c.sent();
                console.log(Error);
                return [3 /*break*/, 12];
            case 12: return [2 /*return*/];
        }
    });
}); };
ListEntityStakes();
