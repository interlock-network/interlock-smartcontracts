/****************************************************************
 * INTR Solana Contract
 ****************************************************************/

#![allow(non_snake_case)]
use solana_program::{
        entrypoint::ProgramResult,
        pubkey::Pubkey,
        account_info::AccountInfo,
        msg,
    };

use crate::instruction::data::*;

pub struct Processor;

impl Processor {

    pub fn run_process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {

        let instruction = ContractInstruction::unpack(instruction_data)?;

        match instruction {

            ContractInstruction::ProgramInit  {
                bumpGLOBAL,
                seedGLOBAL,
            } => {
                msg!("Instruction: ProgramInit");
                Self::process_program_init(
                    program_id,
                    accounts,
                    bumpGLOBAL,
                    seedGLOBAL,
                )
            },

            ContractInstruction::UpdateGlobal {
                dataNumberA,
                dataStringA,
            } => {
                msg!("Instruction: UpdateGlobal");
                Self::process_instruction_two(
                    program_id,
                    accounts,
                    dataNumberA,
                    dataStringA,
                )
            },

            TemplateInstruction::InstructionThree {
                dataNumberC,
            } => {
                msg!("Instruction: InstructionThree");
                Self::process_instruction_three(
                    program_id,
                    accounts,
                    dataNumberC,
                )
            },
        }
    }
}
