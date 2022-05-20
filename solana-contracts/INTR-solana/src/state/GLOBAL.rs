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


// pack/unpack implementation for GLOBAL state account
pub struct GLOBAL {
    pub flags: u32,
    pub owner: Pubkey,
}

impl Sealed for GLOBAL {}

impl Pack for GLOBAL {
    const LEN: usize = SIZE_GLOBAL as usize;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, GLOBAL::LEN];
        let (
            flags,
            owner,
        ) = array_refs![src, FLAGS_LEN, PUBKEY_LEN];

        Ok( GLOBAL {
            flags: u32::from_le_bytes(*flags),
            owner: Pubkey::new_from_array(*owner),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, GLOBAL::LEN];
        let (
            flags_dst,
            owner_dst,
        ) = mut_array_refs![dst, FLAGS_LEN, PUBKEY_LEN];

        let GLOBAL {
            flags,
            owner,
        } = self;

        *flags_dst = flags.to_le_bytes();
        owner_dst.copy_from_slice(owner.as_ref());
    }
}
