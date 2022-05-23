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
    pub flags: u64,
    pub owner: Pubkey,
    pub values: [u64;64],
    
}

impl Sealed for GLOBAL {}

impl Pack for GLOBAL {
    const LEN: usize = SIZE_GLOBAL as usize;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, GLOBAL::LEN];
        let (
            flags,
            owner,
            values,
        ) = array_refs![src, FLAGS_LEN, PUBKEY_LEN, VALUES_LEN];

        let mut valuesNumbers: [u64;64] = [0;64];
        let mut i = 0;
        let mut j = 0;
        for value in valuesNumbers {
            valuesNumbers[i] = u64::from_le_bytes(*values[j..(j + 8));
            i += 1;
            j += 8;
        }

        Ok( GLOBAL {
            flags: u64::from_le_bytes(*flags),
            owner: Pubkey::new_from_array(*owner),
            values: valuesNumbers, 
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, GLOBAL::LEN];
        let (
            flags_dst,
            owner_dst,
            values_dst,
        ) = mut_array_refs![dst, FLAGS_LEN, PUBKEY_LEN, VALUES_LEN];

        let GLOBAL {
            flags,
            owner,
            values,
        } = self;

        let mut valuesBytes: [u8;512] = [0;512];
        let mut i = 0;
        for value in values {
            valuesBytes[i..(i + 8)] = values[i..(i + 8)].to_le_bytes();
            i += 8;
        }
            
        *flags_dst = flags.to_le_bytes();
        owner_dst.copy_from_slice(owner.as_ref());
        values_dst = valuesBytes;
    }
}

