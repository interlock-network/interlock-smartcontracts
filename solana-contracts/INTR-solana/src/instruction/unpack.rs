/****************************************************************
 * INTR Solana Contract
 ****************************************************************/

#![allow(non_snake_case)]

use solana_program::{
        program_error::ProgramError,
    };

use crate::{
        error::error::TemplateError::InvalidInstruction,
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
            0 => Self::CreateGlobal {
                dataNumberA: rest[0],
                dataStringA: rest[9..].to_vec(),
                dataNumberB: unpack_number_u64(&rest[1..9])?,
            },
            1 => Self::UpdateGlobal {
                dataNumberA: rest[0],
                dataStringA: rest[1..].to_vec(),
            },
            2 => Self::InstructionThree {
                dataNumberC: unpack_number_u32(&rest[0..])?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}



