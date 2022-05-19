/****************************************************************
 * Solana program template
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

        let instruction = TemplateInstruction::unpack(instruction_data)?;

        match instruction {

            TemplateInstruction::InstructionOne {
                dataNumberA,
                dataStringA,
                dataNumberB,
            } => {
                msg!("Instruction: InstructionOne");
                Self::process_instruction_one(
                    program_id,
                    accounts,
                    dataNumberA,
                    dataStringA,
                    dataNumberB,
                )
            },

            TemplateInstruction::InstructionTwo {
                dataNumberA,
                dataStringA,
            } => {
                msg!("Instruction: InstructionTwo");
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
