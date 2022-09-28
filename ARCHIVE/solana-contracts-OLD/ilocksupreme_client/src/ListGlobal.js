"use strict";
/****************************************************************
 * Fracpay client ListGlobal
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
var ListGlobal = function () { return __awaiter(void 0, void 0, void 0, function () {
    var GLOBE, GLOBAL, flags1, flags2, index, index, index, index, index, index, index, index, _a;
    return __generator(this, function (_b) {
        switch (_b.label) {
            case 0:
                _b.trys.push([0, 6, , 7]);
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
                return [4 /*yield*/, (0, utils_1.getGLOBAL)()];
            case 4:
                GLOBE = _b.sent();
                // state intention
                console.log("GLOBAL:\n");
                return [4 /*yield*/, (0, utils_1.getGLOBALdata)(GLOBE[0].pubkey)];
            case 5:
                GLOBAL = _b.sent();
                flags1 = (0, utils_1.unpackFlags)(GLOBAL.flags1);
                flags2 = (0, utils_1.unpackFlags)(GLOBAL.flags2);
                // print STAKE data
                console.log("| ADDRESS: ----- ".concat(GLOBE[0].pubkey));
                console.log("| POOL: -------- ".concat(GLOBAL.pool));
                console.log("| FLAGS1: ------ ".concat(GLOBAL.flags1));
                console.log("| FLAGS2: ------ ".concat(GLOBAL.flags2));
                console.log("| OWNER: ------- ".concat(GLOBAL.owner));
                console.log("| VALUE0: ------ ".concat(GLOBAL.value0));
                console.log("| VALUE1: ------ ".concat(GLOBAL.value1));
                console.log("| VALUE2: ------ ".concat(GLOBAL.value2));
                console.log("| VALUE3: ------ ".concat(GLOBAL.value3));
                console.log("| VALUE4: ------ ".concat(GLOBAL.value4));
                console.log("| VALUE5: ------ ".concat(GLOBAL.value5));
                console.log("| VALUE6: ------ ".concat(GLOBAL.value6));
                console.log("| VALUE7: ------ ".concat(GLOBAL.value7));
                console.log("| VALUE8: ------ ".concat(GLOBAL.value8));
                console.log("| VALUE9: ------ ".concat(GLOBAL.value9));
                console.log("| VALUE10: ----- ".concat(GLOBAL.value10));
                console.log("| VALUE11: ----- ".concat(GLOBAL.value11));
                console.log("| VALUE12: ----- ".concat(GLOBAL.value12));
                console.log("| VALUE13: ----- ".concat(GLOBAL.value13));
                console.log("| VALUE14: ----- ".concat(GLOBAL.value14));
                console.log("| VALUE15: ----- ".concat(GLOBAL.value15));
                console.log("| VALUE16: ----- ".concat(GLOBAL.value16));
                console.log("| VALUE17: ----- ".concat(GLOBAL.value17));
                console.log("| VALUE18: ----- ".concat(GLOBAL.value18));
                console.log("| VALUE19: ----- ".concat(GLOBAL.value19));
                console.log("| VALUE20: ----- ".concat(GLOBAL.value20));
                console.log("| VALUE21: ----- ".concat(GLOBAL.value21));
                console.log("| VALUE22: ----- ".concat(GLOBAL.value22));
                console.log("| VALUE23: ----- ".concat(GLOBAL.value23));
                console.log("| VALUE24: ----- ".concat(GLOBAL.value24));
                console.log("| VALUE25: ----- ".concat(GLOBAL.value25));
                console.log("| VALUE26: ----- ".concat(GLOBAL.value26));
                console.log("| VALUE27: ----- ".concat(GLOBAL.value27));
                console.log("| VALUE28: ----- ".concat(GLOBAL.value28));
                console.log("| VALUE29: ----- ".concat(GLOBAL.value29));
                console.log("| VALUE30: ----- ".concat(GLOBAL.value30));
                console.log("| VALUE31: ----- ".concat(GLOBAL.value31));
                console.log("| VALUE32: ----- ".concat(GLOBAL.value32));
                console.log("| VALUE33: ----- ".concat(GLOBAL.value33));
                console.log("| VALUE34: ----- ".concat(GLOBAL.value34));
                console.log("| VALUE35: ----- ".concat(GLOBAL.value35));
                console.log("| VALUE36: ----- ".concat(GLOBAL.value36));
                console.log("| VALUE37: ----- ".concat(GLOBAL.value37));
                console.log("| VALUE38: ----- ".concat(GLOBAL.value38));
                console.log("| VALUE39: ----- ".concat(GLOBAL.value39));
                console.log("| VALUE40: ----- ".concat(GLOBAL.value40));
                console.log("| VALUE41: ----- ".concat(GLOBAL.value41));
                console.log("| VALUE42: ----- ".concat(GLOBAL.value42));
                console.log("| VALUE43: ----- ".concat(GLOBAL.value43));
                console.log("| VALUE44: ----- ".concat(GLOBAL.value44));
                console.log("| VALUE45: ----- ".concat(GLOBAL.value45));
                console.log("| VALUE46: ----- ".concat(GLOBAL.value46));
                console.log("| VALUE47: ----- ".concat(GLOBAL.value47));
                console.log("| VALUE48: ----- ".concat(GLOBAL.value48));
                console.log("| VALUE49: ----- ".concat(GLOBAL.value49));
                console.log("| VALUE50: ----- ".concat(GLOBAL.value50));
                console.log("| VALUE51: ----- ".concat(GLOBAL.value51));
                console.log("| VALUE52: ----- ".concat(GLOBAL.value52));
                console.log("| VALUE53: ----- ".concat(GLOBAL.value53));
                console.log("| VALUE54: ----- ".concat(GLOBAL.value54));
                console.log("| VALUE55: ----- ".concat(GLOBAL.value55));
                console.log("| VALUE56: ----- ".concat(GLOBAL.value56));
                console.log("| VALUE57: ----- ".concat(GLOBAL.value57));
                console.log("| VALUE58: ----- ".concat(GLOBAL.value58));
                console.log("| VALUE59: ----- ".concat(GLOBAL.value59));
                console.log("| VALUE60: ----- ".concat(GLOBAL.value60));
                console.log("| VALUE61: ----- ".concat(GLOBAL.value61));
                console.log("| VALUE62: ----- ".concat(GLOBAL.value62));
                console.log("| VALUE63: ----- ".concat(GLOBAL.value63));
                process.stdout.write("\t| FLAGS1: ------ ");
                process.stdout.write("[ ");
                for (index = 0; index < 4; index++) {
                    process.stdout.write("".concat(flags1[index], " "));
                }
                process.stdout.write("| ");
                for (index = 4; index < 8; index++) {
                    process.stdout.write("".concat(flags1[index], " "));
                }
                process.stdout.write("| ");
                for (index = 8; index < 12; index++) {
                    process.stdout.write("".concat(flags1[index], " "));
                }
                process.stdout.write("| ");
                for (index = 12; index < 16; index++) {
                    process.stdout.write("".concat(flags1[index], " "));
                }
                process.stdout.write("]");
                process.stdout.write("\n\n");
                process.stdout.write("\t| FLAGS2: ------ ");
                process.stdout.write("[ ");
                for (index = 0; index < 4; index++) {
                    process.stdout.write("".concat(flags2[index], " "));
                }
                process.stdout.write("| ");
                for (index = 4; index < 8; index++) {
                    process.stdout.write("".concat(flags2[index], " "));
                }
                process.stdout.write("| ");
                for (index = 8; index < 12; index++) {
                    process.stdout.write("".concat(flags2[index], " "));
                }
                process.stdout.write("| ");
                for (index = 12; index < 16; index++) {
                    process.stdout.write("".concat(flags2[index], " "));
                }
                process.stdout.write("]");
                process.stdout.write("\n\n");
                return [3 /*break*/, 7];
            case 6:
                _a = _b.sent();
                console.log(Error);
                return [3 /*break*/, 7];
            case 7: return [2 /*return*/];
        }
    });
}); };
ListGlobal();
