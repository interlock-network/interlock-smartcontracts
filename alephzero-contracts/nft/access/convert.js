//import * as ss58 from "@subsquid/ss58-codec"

// Import Polkadot.js API dependencies.
const { decodeAddress, encodeAddress } = require('@polkadot/keyring')
const { hexToU8a, isHex } = require('@polkadot/util')

/*
let address = ss58.decode('EXtQYFeY2ivDsfazZvGC9aG87DxnhWH2f9kjUUq2pXTZKF5')
address.prefix // => 2 (address type)
address.bytes  // => Uint8Array of raw address bytes

ss58.encode(address) // => EXtQYFeY2ivDsfazZvGC9aG87DxnhWH2f9kjUUq2pXTZKF5
*/
// Specify an address to test.
const address = '5HLe4u7uZYsMhhHa9NVzH8JGJ24P2ytFPLcnsgeErymVzKtT'

//var bytes = new Uint8Array(32);
/*
const bytes: number[] = [ 126,   1, 136,  73,  61, 174,  92, 205,
                                                 63, 234, 235, 161,  66, 183, 136,  70,
                                                 28, 215, 119, 191, 103, 124,  80,  76,
                                                 63, 101, 119, 186,  70,  19, 240, 162 ];
*/

function toHexString(byteArray) {
  return Array.from(byteArray, function(byte) {
    return ('0' + (byte & 0xFF).toString(16)).slice(-2);
  }).join('')
}

function hexStringToByteArray(hexString) {
    if (hexString.length % 2 !== 0) {
        throw "Must have an even number of hex digits to convert to bytes";
    }/* w w w.  jav  a2 s .  c o  m*/
    var numBytes = hexString.length / 2;
    var byteArray = new Uint8Array(numBytes);
    for (var i=0; i<numBytes; i++) {
        byteArray[i] = parseInt(hexString.substr(i*2, 2), 16);
    }
    return byteArray;
}
console.log(toHexString([
  203,  83,  52,  69, 223, 183,  71, 212,
  222, 107, 116,  89, 140,  22, 196,  69,
  121,  17,   3, 158,  85,  89,  19, 188,
  103,  96, 167,  67, 135,  28, 116, 208
]))
console.log(toHexString([174, 51, 83, 58, 133, 144, 20, 128, 15, 120, 75, 15, 60, 65, 63, 163, 253, 231, 182, 5, 161, 80, 106, 159, 185, 176, 146, 31, 45, 51, 12, 105]))

console.log(hexStringToByteArray('377b1941d19075979a6b2579a1e8dd7dffce9ae58f54846f714608b2f6c785b0'));
