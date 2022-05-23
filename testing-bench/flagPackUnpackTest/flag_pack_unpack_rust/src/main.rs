// test to make sure u32/64 flag pack unpack functions work

use bit_vec::BitVec;

fn main() {

// IC = 00000001.00000111.00011111.01111111
let mut flags = BitVec::from_elem(32, false);

flags.set(0, false);
flags.set(1, false);
flags.set(2, false);
flags.set(3, false);
flags.set(4, false);
flags.set(5, false);
flags.set(6, false);
flags.set(7, true);
flags.set(8, false);
flags.set(9, false);
flags.set(10, false);
flags.set(11, false);
flags.set(12, false);
flags.set(13, true);
flags.set(14, true);
flags.set(15, true);
flags.set(16, false);
flags.set(17, false);
flags.set(18, false);
flags.set(19, true);
flags.set(20, true);
flags.set(21, true);
flags.set(22, true);
flags.set(23, true);
flags.set(24, false);
flags.set(25, true);
flags.set(26, true);
flags.set(27, true);
flags.set(28, true);
flags.set(29, true);
flags.set(30, true);
flags.set(31, true);
//flags.set(32, true);


println!("{:?}", flags);

let flags_packed = pack_flags(flags);

let flags_unpacked = unpack_flags(flags_packed);

println!("{:?}", flags_unpacked);

}

// pack flag values into a single u32
pub fn pack_flags(flags: BitVec) -> u32 {

    let flagbytes = BitVec::to_bytes(&flags);
    let bigflag =    (flagbytes[0] as u32) << 24
                   | (flagbytes[1] as u32) << 16
                   | (flagbytes[2] as u32) << 8
                   | (flagbytes[3] as u32);

    return bigflag
}

// unpack flag values from a single u32
pub fn unpack_flags(flags: u32) -> BitVec {

    let flag4: u8 = (flags >> 24) as u8;
    let flag3: u8 = (flags >> 16 & 0xff) as u8;
    let flag2: u8 = (flags >> 8 & 0xff) as u8;
    let flag1: u8 = (flags & 0xff) as u8;
    let flagbits = BitVec::from_bytes(&[flag4, flag3, flag2, flag1]);

    return flagbits
}


