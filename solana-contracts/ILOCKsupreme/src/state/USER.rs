/****************************************************************
 * ILOCKsupreme Solana Contract
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

pub struct USER {
    pub flags: u16,
    pub count: u16,
    pub success: u16,
    pub fail: u16,
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub balance: u128,
    pub rewards: u128,
}

impl Sealed for USER {}

impl Pack for USER {
    const LEN: usize = SIZE_USER as usize;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, USER::LEN];
        let (
            flags,
            count,
            success,
            fail,
            owner,
            vault,
            balance,
            rewards,
        ) = array_refs![src, U16_LEN, U16_LEN, U16_LEN, U16_LEN, PUBKEY_LEN, PUBKEY_LEN, U128_LEN, U128_LEN];

        Ok( USER {
            flags: u16::from_le_bytes(*flags),
            count: u16::from_be_bytes(*count),
            success: u16::from_be_bytes(*success),
            fail: u16::from_be_bytes(*fail),
            owner: Pubkey::new_from_array(*owner),
            vault: Pubkey::new_from_array(*vault),
            balance: u128::from_be_bytes(*balance),
            rewards: u128::from_be_bytes(*rewards),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, USER::LEN];
        let (
            flags_dst,
            count_dst,
            success_dst,
            fail_dst,
            owner_dst,
            vault_dst,
            balance_dst,
            rewards_dst,
        ) = mut_array_refs![dst, U16_LEN, U16_LEN, U16_LEN, U16_LEN, PUBKEY_LEN, PUBKEY_LEN, U128_LEN, U128_LEN];

        let USER {
            flags,
            count,
            success,
            fail,
            owner,
            vault,
            balance,
            rewards,
        } = self;

        *flags_dst = flags.to_le_bytes();
        *count_dst = count.to_be_bytes();
        *success_dst = success.to_be_bytes();
        *fail_dst = fail.to_be_bytes();
        owner_dst.copy_from_slice(owner.as_ref());
        vault_dst.copy_from_slice(vault.as_ref());
        *balance_dst = balance.to_be_bytes();
        *rewards_dst = rewards.to_be_bytes();
    }
}
