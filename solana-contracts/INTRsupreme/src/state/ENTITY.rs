/****************************************************************
 * Solana program template
 ****************************************************************/

#![allow(non_snake_case)]
use solana_program::{
        program_error::ProgramError,
        pubkey::Pubkey,
        program_pack::Pack,
        program_pack::Sealed,
    };
use arrayref::{
        array_mut_ref,
        array_ref,
        mut_array_refs,
        array_refs,
    };
use crate::utils::utils::*;

pub struct ENTITY {
    pub flags: u16,
    pub identifier: Pubkey,
    pub amount: u64,
}

impl Sealed for ENTITY {}

impl Pack for ENTITY {
    const LEN: usize = SIZE_ENTITY as usize;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, ENTITY::LEN];
        let (
            flags,
            identifier,
            balance,
        ) = array_refs![src, FLAGS_LEN, PUBKEY_LEN, BALANCE_LEN];

        Ok( ACCOUNT {
            flags: u16::from_le_bytes(*flags),
            identifier: Pubkey::new_from_array(*identifier),
            balance: u64::from_be_bytes(*balance),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, ENTITY::LEN];
        let (
            flags_dst,
            identifier_dst,
            balance_dst,
        ) = mut_array_refs![dst, FLAGS_LEN, PUBKEY_LEN, BALANCE_LEN];

        let ACCOUNT {
            flags,
            identifier,
            balance,
        } = self;

        *flags_dst = flags.to_le_bytes();
        identifier_dst.copy_from_slice(identifier.as_ref());
        *balance_dst = balance.to_be_bytes();
    }
}
