/****************************************************************
 * ILOCKsupreme Solana Contract
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
                updateFlags1: unpack_number_u32(&rest[0..U32_LEN])?,
                updateFlags2: unpack_number_u32(&rest[U32_LEN..2*U32_LEN])?,
                values: unpack_array_u32(&rest[2*U32_LEN..])?,
            },
            2 => Self::CreateUser {
                bumpUSER: rest[0],
                seedUSER: rest[1..].to_vec(),
            },
            3 => Self::FillUser {
                amount: rest.get(0..U128_LEN)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u128::from_be_bytes)
                    .ok_or(InvalidInstruction)?.try_into().unwrap(),
            },
            4 => Self::CreateStake {
                bumpSTAKE: rest[0],
                seedSTAKE: rest[1..(1 + PUBKEY_LEN)].to_vec(),
                amount: rest.get((1 + PUBKEY_LEN)..(1 + PUBKEY_LEN + U128_LEN))
                    .and_then(|slice| slice.try_into().ok())
                    .map(u128::from_be_bytes)
                    .ok_or(InvalidInstruction)?.try_into().unwrap(),
                valence: rest[1 + PUBKEY_LEN + U128_LEN],
            },
            5 => Self::SettleEntity {
                determination: rest[0],
            },
            6 => Self::CloseStake {
                seedENTITY: rest[0..PUBKEY_LEN].to_vec(),
            },
            7 => Self::CreateEntity {
                bumpSTAKE: rest[0],
                seedSTAKE: rest[2..(2 + PUBKEY_LEN)].to_vec(),
                bumpENTITY: rest[1],
                seedENTITY: rest[(2 + PUBKEY_LEN)..(2 + 2*PUBKEY_LEN)].to_vec(),
                amount: rest.get((2 + 2*PUBKEY_LEN)..(2 + 2*PUBKEY_LEN + U128_LEN))
                    .and_then(|slice| slice.try_into().ok())
                    .map(u128::from_be_bytes)
                    .ok_or(InvalidInstruction)?.try_into().unwrap(),
                valence: rest[2 + 2*PUBKEY_LEN + U128_LEN],
            },
            8 => Self::SetHunter {
                status: rest[0],
            },
            9 => Self::CheckEntity {
            },
            10 => Self::CloseEntity {
            },
            11 => Self::ClaimEntity {
                bumpSTAKE: rest[0],
                seedSTAKE: rest[1..(1 + PUBKEY_LEN)].to_vec(),
                amount: rest.get((1 + PUBKEY_LEN)..(1 + PUBKEY_LEN + U128_LEN))
                    .and_then(|slice| slice.try_into().ok())
                    .map(u128::from_be_bytes)
                    .ok_or(InvalidInstruction)?.try_into().unwrap(),
                valence: rest[1 + PUBKEY_LEN + U128_LEN],
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}



