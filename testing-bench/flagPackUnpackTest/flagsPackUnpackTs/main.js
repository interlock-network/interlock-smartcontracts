//  // 17244031
var bigFlag = 17244031;
console.log(bigFlag);
var flags = unpackFlags(bigFlag);
console.log(flags);
console.log(packFlags);
function packFlags(flags) {
    parseInt(flags.map(function (i) { return i + 0; }).join(''), 2);
    return bigFlag;
}
function unpackFlags(bigFlag) {
    var flags4 = bigFlag >> 24;
    var flags3 = (bigFlag >> 16) & 0xFF;
    var flags2 = (bigFlag >> 8) & 0xFF;
    var flags1 = bigFlag & 0xFF;
    var bitarray = new Uint8Array(32);
    for (var index = 0; index < 8; index++) {
        bitarray[index] = (flags4 >> (7 - index)) & 0x01;
    }
    for (var index = 0; index < 8; index++) {
        bitarray[8 + index] = (flags3 >> (7 - index)) & 0x01;
    }
    for (index = 0; index < 8; index++) {
        bitarray[16 + index] = (flags2 >> (7 - index)) & 0x01;
    }
    for (index = 0; index < 8; index++) {
        bitarray[24 + index] = (flags1 >> (7 - index)) & 0x01;
    }
    return bitarray;
}
