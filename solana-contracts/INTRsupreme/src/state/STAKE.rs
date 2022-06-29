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

pub struct STAKE {
    pub flags: u16,
    pub identifier: Pubkey,
    pub amount: u64,
}

impl Sealed for STAKE {}

impl Pack for STAKE {
    const LEN: usize = SIZE_STAKE as usize;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, STAKE::LEN];
        let (
            flags,
            identifier,
            amount,
        ) = array_refs![src, FLAGS_LEN, PUBKEY_LEN, BALANCE_LEN];

        Ok( STAKE {
            flags: u16::from_le_bytes(*flags),
            identifier: Pubkey::new_from_array(*identifier),
            amount: u64::from_be_bytes(*amount),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, STAKE::LEN];
        let (
            flags_dst,
            identifier_dst,
            amount_dst,
        ) = mut_array_refs![dst, FLAGS_LEN, PUBKEY_LEN, BALANCE_LEN];

        let STAKE {
            flags,
            identifier,
            amount,
        } = self;

        *flags_dst = flags.to_le_bytes();
        identifier_dst.copy_from_slice(identifier.as_ref());
        *amount_dst = amount.to_be_bytes();
    }
}
