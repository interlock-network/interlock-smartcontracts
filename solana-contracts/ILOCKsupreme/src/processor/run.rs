/****************************************************************
 * ILOCKsupreme Solana Contract
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

            ContractInstruction::ProgramInit {
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
                updateFlags1,    
                updateFlags2,    
                values,
            } => {
                msg!("Instruction: UpdateGlobal");
                Self::process_update_global(
                    program_id,
                    accounts,
                    updateFlags1,
                    updateFlags2,
                    values,
                )
            },

            ContractInstruction::CreateUser {
                bumpUSER,
                seedUSER,
            } => {
                msg!("Instruction: CreateAccount");
                Self::process_create_account(
                    program_id,
                    accounts,
                    bumpUSER,
                    seedUSER,
                )
            },

            ContractInstruction::FillUser {
                amount,
            } => {
                msg!("Instruction: FillUser");
                Self::process_fill_user(
                    program_id,
                    accounts,
                    amount,
                )
            },

            ContractInstruction::CreateStake {
                bumpSTAKE,
                seedSTAKE,
                amount,
                valence,
            } => {
                msg!("Instruction: CreateStake");
                Self::process_create_stake(
                    program_id,
                    accounts,
                    bumpSTAKE,
                    seedSTAKE,
                    amount,
                    valence,
                )
            },

            ContractInstruction::SettleEntity {
                determination,
            } => {
                msg!("Instruction: SettleEntity");
                Self::process_settle_entity(
                    program_id,
                    accounts,
                    determination,
                )
            },

            ContractInstruction::CloseStake {
                seedENTITY,

            } => {
                msg!("Instruction: CloseStake");
                Self::process_close_stake(
                    program_id,
                    accounts,
                    seedENTITY,
                )
            },

            ContractInstruction::CreateEntity {
                bumpSTAKE,
                seedSTAKE,
                bumpENTITY,
                seedENTITY,
                amount,
                valence,
            } => {
                msg!("Instruction: CreateEntity");
                Self::process_create_entity(
                    program_id,
                    accounts,
                    bumpSTAKE,
                    seedSTAKE,
                    bumpENTITY,
                    seedENTITY,
                    amount,
                    valence,
                )
            },

            ContractInstruction::SetHunter {
                status,
            } => {
                msg!("Instruction: SetHunter");
                Self::process_set_hunter(
                    program_id,
                    accounts,
                    status,
                )
            },
            
            ContractInstruction::CheckEntity {
            } => {
                msg!("Instruction: CheckEntity");
                Self::process_check_entity(
                    program_id,
                    accounts,
                )
            },
            
            ContractInstruction::CloseEntity {
            } => {
                msg!("Instruction: CloseEntity");
                Self::process_close_entity(
                    program_id,
                    accounts,
                )
            },

            ContractInstruction::ClaimEntity {
                bumpSTAKE,
                seedSTAKE,
                amount,
                valence,
            } => {
                msg!("Instruction: ClaimEntity");
                Self::process_claim_entity(
                    program_id,
                    accounts,
                    bumpSTAKE,
                    seedSTAKE,
                    amount,
                    valence,
                )
            },

            ContractInstruction::ResolveStake {
            } => {
                msg!("Instruction: ResolveStake");
                Self::process_resolve_stake(
                    program_id,
                    accounts,
                )
            },
        }
    }
}
