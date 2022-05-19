/****************************************************************
 * Solana program template
 ****************************************************************/

#![allow(non_snake_case)]
use solana_program::{
        account_info::{
            next_account_info,
            AccountInfo
        },
        entrypoint::ProgramResult,
        program_error::ProgramError,
        pubkey::Pubkey,
        msg,
    };

use crate::processor::run::Processor;

// for this instruction, the expected accounts are
//
// 0, operator pubkey
// 1, system rent account
// 2, pubkey for account type FIRST
// 3, pubkey for account type SEECOND

impl Processor {

    pub fn process_instruction_three(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _dataNumberC: u32,
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let operator = next_account_info(account_info_iter)?;
        let _rent = next_account_info(account_info_iter)?;
        let _first = next_account_info(account_info_iter)?;
        let _second = next_account_info(account_info_iter)?;

        // customary to check to make sure tx operator is signer
        if !operator.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        msg!("I AM INSTRUCTION NUMBER THREE!");

        // so a bunch of stuff
        // and other stuff

        Ok(())
    }
}

