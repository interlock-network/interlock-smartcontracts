// test to see that client side bigFlag / flags are handled properly

const bigFlag = 65535;
console.log(bigFlag);

const flags = unpackFlags(bigFlag);
console.log(flags);

console.log(packFlags(flags));



function packFlags(flags: Uint8Array) {
	parseInt(flags.map(i => i + 0).join(''), 2);
	return bigFlag
}

function unpackFlags(bigFlag: number) {
	const flags2 = (bigFlag >> 8) & 0xFF;
	const flags1 = bigFlag & 0xFF;
	var bitarray = new Uint8Array(16);
	for (var index = 0; index < 8; index++) {
		bitarray[index] = (flags2 >> (7 - index)) & 0x01;
	}
	for (var index = 0; index < 8; index++) {
		bitarray[8 + index] = (flags1 >> (7 - index)) & 0x01;
	}
	return bitarray

}


// flag packing is limited by number precision on client side to u16
