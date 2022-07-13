/****************************************************************
 * INTR Solana Contract
 ****************************************************************/

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::array::TryFromSliceError;

use solana_program::program_error::ProgramError;

// use std::array::TryFromSliceError;
use bit_vec::BitVec;

use crate::error::error::ContractError::InvalidInstruction;



pub const VALUES: usize = 64;
pub const PUBKEY_LEN: usize = 32;
pub const U16: usize = 2;

pub const U32_LEN: usize = 4;
pub const U64_LEN: usize = 8;

pub const U128_LEN: usize = 16;
pub const VALUES_LEN: usize = VALUES * U32_LEN;
pub const SIZE_GLOBAL: u16 = (2*U16_LEN + PUBKEY_LEN + VALUES_LEN) as u16;
    // 292
pub const SIZE_ACCOUNT: u16 = (4*U16_LEN + PUBKEY_LEN + 2*U128_LEN) as u16;
    // 72
pub const SIZE_STAKE: u16 = (U16_LEN + U64_LEN + PUBKEY_LEN + U128_LEN) as u16;
    // 58
pub const SIZE_ENTITY: u16 = (2*U16_LEN + PUBKEY_LEN + 2*U128_LEN + U64_LEN) as u16;
    // 76

// pack flag values into a single u32
pub fn pack_32_flags(flags: BitVec) -> u32 {

    let flagbytes = BitVec::to_bytes(&flags);
    let bigflag =  (flagbytes[0] as u32) << 24
                   | (flagbytes[1] as u32) << 16
                   | (flagbytes[2] as u32) << 8
                   | (flagbytes[3] as u32);

    return bigflag
}

// unpack flag values from a single u32
pub fn unpack_32_flags(flags: u32) -> BitVec {

    let flag3: u8 = (flags >> 24) as u8;
    let flag2: u8 = (flags >> 16 & 0xff) as u8;
    let flag1: u8 = (flags >> 8 & 0xff) as u8;
    let flag0: u8 = (flags & 0xff) as u8;
    let flagbits = BitVec::from_bytes(&[flag3,
                                        flag2,
                                        flag1,
                                        flag0,
                                        ]);

    return flagbits
}

// pack flags into a single u16
pub fn pack_16_flags(flags: BitVec) -> u16 {

    let flagbytes = BitVec::to_bytes(&flags);
    let bigflag = ((flagbytes[0] as u16) << 8) | flagbytes[1] as u16;

    return bigflag
}

// unpack flags from a single u16
pub fn unpack_16_flags(flags: u16) -> BitVec {

    let highflag: u8 = (flags >> 8) as u8;
    let lowflag: u8 = (flags & 0xff) as u8;
    let flagbits = BitVec::from_bytes(&[highflag, lowflag]);

    return flagbits
}



//  pack a string into fixed size byte array:w

pub fn pack_values(vector: Vec<u8>) -> [u8; VALUES_LEN] {

    let mut values_bytes: Vec<u8>;
    values_bytes = vector.to_vec();
    let mut zeros: Vec<u8> = vec![0; VALUES_LEN - values_bytes.len()];
    values_bytes.append(&mut zeros);

    return valuespack(values_bytes).unwrap();
}

type valuesOutput = [u8; VALUES_LEN];
fn valuespack(valuestore: Vec<u8>) -> Result<valuesOutput, TryFromSliceError> {

    valuestore.as_slice().try_into()
}


// unpack instruction_data numbers
pub fn unpack_number_u32(input: &[u8]) -> Result<u32, ProgramError> {
    let amount = input
        .get(..4)
        .and_then(|slice| slice.try_into().ok())
        .map(u32::from_le_bytes)
        .ok_or(InvalidInstruction)?;
    Ok(amount)
}

pub fn unpack_array_u32(input: &[u8]) -> Result<[u32; VALUES], ProgramError> {
    
    let mut array: [u32; VALUES] = [0; VALUES];
    let mut i: usize = 0;
    let mut j: usize = 0;
    for _ in array {
        let number = input
            .get(i..(i + 4))
            .and_then(|slice| slice.try_into().ok())
            .map(u32::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        array[j] = number;
        i += 4;
        j += 1;
    }
    return Ok(array)
}


pub fn unpack_number_u64(input: &[u8]) -> Result<u64, ProgramError> {
    let amount = input
        .get(..4)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(InvalidInstruction)?;
    Ok(amount)
}

pub fn array_4u8(slice: &[u8]) -> [u8; 4] {
    slice.try_into().expect("slice wrong length")
}

