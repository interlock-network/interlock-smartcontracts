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

pub struct ACCOUNT {
    pub flags: u16,
    pub count: u16,
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub balance: u64,
}

impl Sealed for ACCOUNT {}

impl Pack for ACCOUNT {
    const LEN: usize = SIZE_ACCOUNT as usize;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, ACCOUNT::LEN];
        let (
            flags,
            count,
            owner,
            vault,
            balance,
        ) = array_refs![src, FLAGS_LEN, COUNT_LEN, PUBKEY_LEN, PUBKEY_LEN, BALANCE_LEN];

        Ok( ACCOUNT {
            flags: u16::from_le_bytes(*flags),
            count: u16::from_le_bytes(*count),
            owner: Pubkey::new_from_array(*owner),
            vault: Pubkey::new_from_array(*vault),
            balance: u64::from_be_bytes(*balance),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, ACCOUNT::LEN];
        let (
            flags_dst,
            count_dst,
            owner_dst,
            vault_dst,
            balance_dst,
        ) = mut_array_refs![dst, FLAGS_LEN, COUNT_LEN, PUBKEY_LEN, PUBKEY_LEN, BALANCE_LEN];

        let ACCOUNT {
            flags,
            count,
            owner,
            vault,
            balance,
        } = self;

        *flags_dst = flags.to_le_bytes();
        *count_dst = count.to_le_bytes();
        owner_dst.copy_from_slice(owner.as_ref());
        vault_dst.copy_from_slice(vault.as_ref());
        *balance_dst = balance.to_be_bytes();
    }
}
