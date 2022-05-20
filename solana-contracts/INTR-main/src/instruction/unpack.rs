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
                bumpGLOBAL: rest[0],
                seedGLOBAL: rest[1..].to_vec(),
            },
            1 => Self::UpdateGlobal {
                updateFlags: unpack_number_u32(&rest[0..FLAGS_LEN])?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}



