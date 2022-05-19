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

// example Pack implementation for struct defining state for an account type called SECOND
//
// I used a variety of variables to paint a comprehensive picture of this implementation
//


pub struct SECOND {
    pub operator: Pubkey,
    pub balance: u64,
    pub stringy: [u8; STRING_LEN],
}

impl Sealed for SECOND {}

impl Pack for SECOND {
    const LEN: usize = SIZE_SECOND as usize;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, SECOND::LEN];
        let (
            operator,
            balance,
            stringy,
        ) = array_refs![src, PUBKEY_LEN, BALANCE_LEN, STRING_LEN];

        Ok( SECOND {
            operator: Pubkey::new_from_array(*operator),
            balance: u64::from_be_bytes(*balance),
            stringy: *stringy,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, SECOND::LEN];
        let (
            operator_dst,
            balance_dst,
            stringy_dst,
        ) = mut_array_refs![dst, PUBKEY_LEN, BALANCE_LEN, STRING_LEN];

        let SECOND {
            operator,
            balance,
            stringy,
        } = self;

        operator_dst.copy_from_slice(operator.as_ref());
        *balance_dst = balance.to_be_bytes();
        *stringy_dst = *stringy;

    }
}
