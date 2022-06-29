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
                updateFlags,    
                values,
            } => {
                msg!("Instruction: UpdateGlobal");
                Self::process_update_global(
                    program_id,
                    accounts,
                    updateFlags,
                    values,
                )
            },

            ContractInstruction::CreateAccount  {
                bumpACCOUNT,
                seedACCOUNT,
            } => {
                msg!("Instruction: CreateAccount");
                Self::process_create_account(
                    program_id,
                    accounts,
                    bumpACCOUNT,
                    seedACCOUNT,
                )
            },

            ContractInstruction::FillAccount  {
            } => {
                msg!("Instruction: FillAccount");
                Self::process_fill_account(
                    program_id,
                    accounts,
                )
            },

            ContractInstruction::CreateStake  {
                bumpSTAKE,
                seedSTAKE,
                amount,
            } => {
                msg!("Instruction: CreateStake");
                Self::process_create_stake(
                    program_id,
                    accounts,
                    bumpSTAKE,
                    seedSTAKE,
                    amount,
                )
            },

            ContractInstruction::ChangeStake  {
                amount,
            } => {
                msg!("Instruction: ChangeStake");
                Self::process_change_stake(
                    program_id,
                    accounts,
                    amount,
                )
            },
        }
    }
}
