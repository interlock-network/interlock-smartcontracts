/****************************************************************
 * INTR Solana Contract
 ****************************************************************/

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use solana_program::program_error::ProgramError;

use std::array::TryFromSliceError;
use bit_vec::BitVec;

use crate::error::error::TemplateError::InvalidInstruction;


pub const PUBKEY_LEN: usize = 32;
pub const FLAGS_LEN: usize = 8;
pub const VALUES_LEN: usize = 512;
pub const SIZE_GLOBAL: u8 = (FLAGS_LEN + PUBKEY_LEN + VALUES_LEN) as u8;
    // 548 bytes

// pack flag values into a single u32
pub fn pack_flags(flags: BitVec) -> u64 {

    let flagbytes = BitVec::to_bytes(&flags);
    let bigflag =    (flagbytes[0] as u64) << 56
                   | (flagbytes[1] as u64) << 48
                   | (flagbytes[2] as u64) << 40
                   | (flagbytes[3] as u64) << 32
                   | (flagbytes[4] as u64) << 24
                   | (flagbytes[5] as u64) << 16
                   | (flagbytes[6] as u64) << 8
                   | (flagbytes[7] as u64);

    return bigflag
}

// unpack flag values from a single u32
pub fn unpack_flags(flags: u64) -> BitVec {

    let flag7: u8 = (flags >> 56) as u8;
    let flag6: u8 = (flags >> 48 & 0xff) as u8;
    let flag5: u8 = (flags >> 40 & 0xff) as u8;
    let flag4: u8 = (flags >> 32 & 0xff) as u8;
    let flag3: u8 = (flags >> 24 & 0xff) as u8;
    let flag2: u8 = (flags >> 16 & 0xff) as u8;
    let flag1: u8 = (flags >> 8 & 0xff) as u8;
    let flag0: u8 = (flags & 0xff) as u8;
    let flagbits = BitVec::from_bytes(&[flag7,
                                        flag6,
                                        flag5,
                                        flag4,
                                        flag3,
                                        flag2,
                                        flag1,
                                        flag0,
                                        ]);

    return flagbits
}

//  pack a string into fixed size byte array
pub fn pack_stringy(stringy: Vec<u8>) -> [u8; STRING_LEN] {

    let mut stringy_bytes: Vec<u8>;
    stringy_bytes = stringy.to_vec();
    let mut zeros: Vec<u8> = vec![0; STRING_LEN - stringy_bytes.len()];
    stringy_bytes.append(&mut zeros);

    return stringypack(stringy_bytes).unwrap();
}

type stringyOutput = [u8; STRING_LEN];
fn stringypack(vector: Vec<u8>) -> Result<stringyOutput, TryFromSliceError> {

    vector.as_slice().try_into()
}

// unpack instruction_data numbers
pub fn unpack_number_u64(input: &[u8]) -> Result<u64, ProgramError> {
    let amount = input
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(InvalidInstruction)?;
    Ok(amount)
}

pub fn unpack_array_u64(input: &[u8]) -> Result<[u64,64], ProgramError> {
    
    let mut array: [u64;64] = [0;64];
    let mut i = 0;
    let mut j = 0;
    for i < 512 {
        let mut number = input
            .get(i..(i + 8))
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        array[j] = number
        i += 8;
        j += 1;
}


pub fn unpack_number_u32(input: &[u8]) -> Result<u32, ProgramError> {
    let amount = input
        .get(..4)
        .and_then(|slice| slice.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or(InvalidInstruction)?;
    Ok(amount)
}
