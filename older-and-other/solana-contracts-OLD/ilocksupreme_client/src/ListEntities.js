"use strict";
/****************************************************************
 * Fracpay client ListEntities
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
var ListEntities = function () { return __awaiter(void 0, void 0, void 0, function () {
    var ENTITYflags, ENTITYs, countENTITY, ENTITY, flags, index, index, index, index, _a;
    return __generator(this, function (_b) {
        switch (_b.label) {
            case 0:
                _b.trys.push([0, 9, , 10]);
                // setup
                return [4 /*yield*/, (0, utils_1.establishConnection)()];
            case 1:
                // setup
                _b.sent();
                return [4 /*yield*/, (0, utils_1.establishOperator)()];
            case 2:
                _b.sent();
                return [4 /*yield*/, (0, utils_1.checkProgram)()];
            case 3:
                _b.sent();
                // state intention
                console.log("\nENTITYs:\n");
                ENTITYflags = (0, utils_1.unpackFlags)(ENTITY.flags);
                return [4 /*yield*/, (0, utils_1.getENTITYs)()];
            case 4:
                ENTITYs = _b.sent();
                // state intention
                console.log("ENTITYs:\n");
                countENTITY = 0;
                _b.label = 5;
            case 5:
                if (!(countENTITY <= ENTITYs.length)) return [3 /*break*/, 8];
                return [4 /*yield*/, (0, utils_1.getENTITYdata)(ENTITYs[countENTITY].pubkey)];
            case 6:
                ENTITY = _b.sent();
                flags = (0, utils_1.unpackFlags)(ENTITY.flags);
                // print STAKE data
                console.log("| ADDRESS: ----- ".concat(ENTITYs[countENTITY].pubkey));
                console.log("| HUNTER: ------ ".concat(ENTITY.hunter));
                console.log("| STAKEPOS: ---- ".concat(ENTITY.stakepos));
                console.log("| STAKENEG: ---- ".concat(ENTITY.stakeneg));
                console.log("| STAKERS: ----- ".concat(ENTITY.stakers));
                console.log("| TIMESTAMP: --- ".concat(ENTITY.timestamp));
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
                _b.label = 7;
            case 7:
                countENTITY++;
                return [3 /*break*/, 5];
            case 8: return [3 /*break*/, 10];
            case 9:
                _a = _b.sent();
                console.log(Error);
                return [3 /*break*/, 10];
            case 10: return [2 /*return*/];
        }
    });
}); };
ListEntities();
