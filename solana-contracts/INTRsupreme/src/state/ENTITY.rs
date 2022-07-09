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
    pub hunter: Pubkey,
    pub stakepos: u128,
    pub stakeneg: u128,
    pub stakers: u16,
    pub time: i64,
}

impl Sealed for ENTITY {}

impl Pack for ENTITY {
    const LEN: usize = SIZE_ENTITY as usize;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, ENTITY::LEN];
        let (
            flags,
            hunter,
            stakepos,
            stakeneg,
            stakers,
            time,
        ) = array_refs![src, U16_LEN, PUBKEY_LEN, U128_LEN, U128_LEN, U16_LEN, U64_LEN];

        Ok( ENTITY {
            flags: u16::from_le_bytes(*flags),
            hunter: Pubkey::new_from_array(*hunter),
            stakepos: u128::from_be_bytes(*stakepos),
            stakeneg: u128::from_be_bytes(*stakeneg),
            stakers: u16::from_be_bytes(*stakers),
            time: i64::from_be_bytes(*time),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, ENTITY::LEN];
        let (
            flags_dst,
            hunter_dst,
            stakepos_dst,
            stakeneg_dst,
            stakers_dst,
            time_dst,
        ) = mut_array_refs![dst, U16_LEN, PUBKEY_LEN, U128_LEN, U128_LEN, U16_LEN, U64_LEN];

        let ENTITY {
            flags,
            hunter,
            stakepos,
            stakeneg,
            stakers,
            time,
        } = self;

        *flags_dst = flags.to_le_bytes();
        hunter_dst.copy_from_slice(hunter.as_ref());
        *stakepos_dst = stakepos.to_be_bytes();
        *stakeneg_dst = stakeneg.to_be_bytes();
        *stakers_dst = stakers.to_be_bytes();
        *time_dst = time.to_be_bytes();

    }
}
