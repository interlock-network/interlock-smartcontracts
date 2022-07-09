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
    pub time: i64,
    pub entity: Pubkey,
    pub amount: u128,
}

impl Sealed for STAKE {}

impl Pack for STAKE {
    const LEN: usize = SIZE_STAKE as usize;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, STAKE::LEN];
        let (
            flags,
            time,
            entity,
            amount,
        ) = array_refs![src, U16_LEN, U64_LEN, PUBKEY_LEN, U128_LEN];

        Ok( STAKE {
            flags: u16::from_le_bytes(*flags),
            time: i64::from_be_bytes(*time),
            entity: Pubkey::new_from_array(*entity),
            amount: u128::from_be_bytes(*amount),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, STAKE::LEN];
        let (
            flags_dst,
            time_dst
            identifier_dst,
            amount_dst,
        ) = mut_array_refs![dst, U16_LEN, U64_LEN, PUBKEY_LEN, U128_LEN];

        let STAKE {
            flags,
            time,
            entity,
            amount,
        } = self;

        *flags_dst = flags.to_le_bytes();
        *time_dst = time.to_be_bytes();
        entity_dst.copy_from_slice(entity.as_ref());
        *amount_dst = amount.to_be_bytes();
    }
}
