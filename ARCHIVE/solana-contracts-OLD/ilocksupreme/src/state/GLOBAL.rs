/****************************************************************
 * ILOCKsupreme Solana Contract
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
    pub pool: u128,
    pub flags: u32,
    pub owner: Pubkey,
    pub values: [u32; VALUES],
    
}

impl Sealed for GLOBAL {}

impl Pack for GLOBAL {
    const LEN: usize = SIZE_GLOBAL as usize;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, GLOBAL::LEN];
        let (
            pool,
            flags,
            owner,
            _values,
        ) = array_refs![src, U128_LEN, 2*U16_LEN, PUBKEY_LEN, VALUES_LEN];

        let mut valuesNumbers: [u32; VALUES] = [0; VALUES];
        let valuesBytes = &src[(U128_LEN + 2*U16_LEN + PUBKEY_LEN)..]; 
        let mut i = 0;
        let mut j = 0;
        for _ in valuesNumbers {
            valuesNumbers[i] = u32::from_le_bytes(array_4u8(&valuesBytes[j..(j + U32_LEN)]));
            i += 1;
            j += U32_LEN;
        }

        Ok( GLOBAL {
            pool: u128::from_be_bytes(*pool),
            flags: u32::from_le_bytes(*flags),
            owner: Pubkey::new_from_array(*owner),
            values: valuesNumbers, 
        })
    }



    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, GLOBAL::LEN];
        let (
            pool_dst,
            flags_dst,
            owner_dst,
            values_dst,
        ) = mut_array_refs![dst, U128_LEN, 2*U16_LEN, PUBKEY_LEN, VALUES_LEN];

        let GLOBAL {
            pool,
            flags,
            owner,
            values,
        } = self;

        let mut valuesBytes = vec![];
        for value in values {
            valuesBytes.extend(&value.to_le_bytes()[..]);
        }
            
        *pool_dst = pool.to_be_bytes();
        *flags_dst = flags.to_le_bytes();
        owner_dst.copy_from_slice(owner.as_ref());
        *values_dst = pack_values(valuesBytes);
    }
}

