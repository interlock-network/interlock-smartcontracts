/****************************************************************
 * Solana program template
 ****************************************************************/

#![allow(non_snake_case)]

use solana_program::{
        program_error::ProgramError,
        pubkey::Pubkey,
        program_pack::{
            Pack,
            Sealed,
        },
    };

use arrayref::{
        array_mut_ref,
        array_ref,
        mut_array_refs,
        array_refs,
    };

use crate::utils::utils::*;

// example Pack implementation for struct defining state for an account type called FIRST
//
// I used a variety of variables to paint a comprehensive picture of this implementation
//
// flags is a u16 I use in general for 16 different state flags
//
// the string variable is fixed at 32B

pub struct FIRST {
    pub flags: u16,
    pub operator: Pubkey,
    pub balance: u64,
    pub stringy: [u8; STRING_LEN],
}

impl Sealed for FIRST {}

impl Pack for FIRST {
    const LEN: usize = SIZE_FIRST as usize;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, FIRST::LEN];
        let (
            flags,
            operator,
            balance,
            stringy,
        ) = array_refs![src, FLAGS_LEN, PUBKEY_LEN, BALANCE_LEN, STRING_LEN];

        Ok( FIRST {
            flags: u16::from_le_bytes(*flags),
            operator: Pubkey::new_from_array(*operator),
            balance: u64::from_be_bytes(*balance),
            stringy: *stringy,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, FIRST::LEN];
        let (
            flags_dst,
            operator_dst,
            balance_dst,
            stringy_dst
        ) = mut_array_refs![dst, FLAGS_LEN, PUBKEY_LEN, BALANCE_LEN, STRING_LEN];

        let FIRST {
            flags,
            operator,
            balance,
            stringy,
        } = self;

        *flags_dst = flags.to_le_bytes();
        operator_dst.copy_from_slice(operator.as_ref());
        *balance_dst = balance.to_be_bytes();
        *stringy_dst = *stringy;
    }
}
