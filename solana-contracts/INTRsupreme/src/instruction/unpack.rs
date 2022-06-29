/****************************************************************
 * INTR Solana Contract
 ****************************************************************/

#![allow(non_snake_case)]


use solana_program::{
        program_error::ProgramError,
    };

use crate::{
        error::error::ContractError::InvalidInstruction,
        instruction::data::ContractInstruction,
        utils::utils::*,
    };


// it is customary to specify instruction type with leading tag
//
// tag is one byte, so we could have up to 256 instructions


impl ContractInstruction {

    // Unpacks a byte buffer into a TemplateInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {

        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok( match tag {
            0 => Self::ProgramInit {
                bumpGLOBAL: rest[0],
                seedGLOBAL: rest[1..].to_vec(),
            },
            1 => Self::UpdateGlobal {
                updateFlags: unpack_number_u32(&rest[0..FLAGS_LEN])?,
                values: unpack_array_u32(&rest[FLAGS_LEN..])?,
            },
            2 => Self::CreateAccount {
                bumpACCOUNT: rest[0],
                seedACCOUNT: rest[1..].to_vec(),
            },
            3 => Self::FillAccount {
            },
            4 => Self::CreateStake {
                bumpSTAKE: rest[0],
                seedSTAKE: rest[1..(1 + PUBKEY_LEN)].to_vec(),
                amount: rest.get((1 + PUBKEY_LEN)..(1 + PUBKEY_LEN + BALANCE_LEN))
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_be_bytes)
                    .ok_or(InvalidInstruction)?,
            },
            5 => Self::ChangeStake {
                amount: rest.get(..)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_be_bytes)
                    .ok_or(InvalidInstruction)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}



