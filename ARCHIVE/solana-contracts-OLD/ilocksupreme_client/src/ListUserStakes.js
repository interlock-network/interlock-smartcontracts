"use strict";
/****************************************************************
 * Fracpay client ListUserStakes
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
// utility constants
var utils_2 = require("./utils");
/****************************************************************
 * main
 ****************************************************************/
var ListUserStakes = function () { return __awaiter(void 0, void 0, void 0, function () {
    var count, pdaUSERseed, _a, pdaUSER, bumpUSER, USER, USERflags, index, index, index, index, countSTAKE, pdaSTAKEseed, pdaSTAKE, bumpSTAKE, STAKE, flags, index, index, index, index, _b;
    var _c;
    return __generator(this, function (_d) {
        switch (_d.label) {
            case 0:
                _d.trys.push([0, 11, , 12]);
                // setup
                return [4 /*yield*/, (0, utils_1.establishConnection)()];
            case 1:
                // setup
                _d.sent();
                return [4 /*yield*/, (0, utils_1.establishOperator)()];
            case 2:
                _d.sent();
                return [4 /*yield*/, (0, utils_1.checkProgram)()];
            case 3:
                _d.sent();
                count = new Uint16Array(1);
                count[0] = 1; // in production, this is always 0
                pdaUSERseed = (0, utils_1.createSeed)(utils_2.ownerKEY.publicKey, count);
                return [4 /*yield*/, (0, utils_1.deriveAddress)(pdaUSERseed)];
            case 4:
                _a = _d.sent(), pdaUSER = _a[0], bumpUSER = _a[1];
                console.log(". USER pda:\t\t".concat(pdaUSER.toBase58(), " found after ").concat(256 - bumpUSER, " tries"));
                return [4 /*yield*/, (0, utils_1.getUSERdata)(pdaUSER)];
            case 5:
                USER = _d.sent();
                // state intention
                console.log("\nUSER STAKEs:\n");
                USERflags = (0, utils_1.unpackFlags)(USER.flags);
                // print USER data
                console.log("| USER");
                console.log("| ADDRESS: ----- ".concat(pdaUSER.toBase58()));
                console.log("| SUCCESS: ----- ".concat(USER.success));
                console.log("| FAIL: -------- ".concat(USER.fail));
                console.log("| OWNER: ------- ".concat(USER.owner));
                console.log("| VAULT: ------- ".concat(USER.vault));
                console.log("| BALANCE: ----- ".concat(USER.balance));
                console.log("| REWARDS: ----- ".concat(USER.rewards));
                process.stdout.write("| FLAGS: ------- ");
                process.stdout.write("[ ");
                for (index = 0; index < 4; index++) {
                    process.stdout.write("".concat(USERflags[index], " "));
                }
                process.stdout.write("| ");
                for (index = 4; index < 8; index++) {
                    process.stdout.write("".concat(USERflags[index], " "));
                }
                process.stdout.write("| ");
                for (index = 8; index < 12; index++) {
                    process.stdout.write("".concat(USERflags[index], " "));
                }
                process.stdout.write("| ");
                for (index = 12; index < 16; index++) {
                    process.stdout.write("".concat(USERflags[index], " "));
                }
                process.stdout.write("]");
                process.stdout.write("\n\n");
                countSTAKE = new Uint16Array(1);
                countSTAKE[0] = 0;
                _d.label = 6;
            case 6:
                if (!(countSTAKE[0] <= USER.count)) return [3 /*break*/, 10];
                // find STAKE address
                pdaSTAKEseed = (0, utils_1.createSeed)(pdaUSER, countSTAKE);
                return [4 /*yield*/, (0, utils_1.deriveAddress)(pdaSTAKEseed)];
            case 7:
                _c = _d.sent(), pdaSTAKE = _c[0], bumpSTAKE = _c[1];
                return [4 /*yield*/, (0, utils_1.getSTAKEdata)(pdaSTAKE)];
            case 8:
                // get STAKE data
                STAKE = _d.sent();
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
                _d.label = 9;
            case 9:
                countSTAKE[0]++;
                return [3 /*break*/, 6];
            case 10: return [3 /*break*/, 12];
            case 11:
                _b = _d.sent();
                console.log(Error);
                return [3 /*break*/, 12];
            case 12: return [2 /*return*/];
        }
    });
}); };
ListUserStakes();
