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
        program::invoke_signed,
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
        sysvar::{
            rent::Rent,
            Sysvar,
        },
        msg,
        system_instruction,
    };

use bit_vec::BitVec;

use crate::{
        //error::error::ContractError::GlobalAlreadyExistsError,
        processor::run::Processor,
        utils::utils::*,
        state::{
            GLOBAL::*,
            ACCOUNT::*,
            STAKE::*,
        },
    };

// for this instruction, the expected accounts are
//
// 0, owner pubkey, is signer
// 1, GLOBAL pda
// 2, system rent account
// 3, register ACCOUNT pda

impl Processor {

    pub fn process_create_stake(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bumpSTAKE: u8,
        seedSTAKE: Vec<u8>,
        amount: u64,
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let pdaGLOBAL = next_account_info(account_info_iter)?;
        let rent = next_account_info(account_info_iter)?;
        let pdaSTAKE = next_account_info(account_info_iter)?;
        let hash = next_account_info(account_info_iter)?;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // calculate rent if we want to create new account
        let rentSTAKE = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_STAKE.into());

        // create pdaGLOBAL
        invoke_signed(
        &system_instruction::create_account(
            &owner.key,
            &pdaSTAKE.key,
            rentSTAKE,
            SIZE_STAKE.into(),
            &program_id
        ),
        &[
            owner.clone(),
            pdaSTAKE.clone()
        ],
        &[&[&seedSTAKE, &[bumpSTAKE]]]
        )?;
        msg!("Successfully created pdaSTAKE");
// need to determine if create_account reverts if account already exists
        
        // get unititialized GLOBAL data
        let mut STAKEinfo = STAKE::unpack_unchecked(&pdaSTAKE.try_borrow_data()?)?;
        
        // init flags
        let flags = BitVec::from_elem(16, false);

        // populate and pack GLOBAL account info
        STAKEinfo.flags = pack_16_flags(flags);
        STAKEinfo.identifier = *hash.key;
        STAKEinfo.amount = amount;
        STAKE::pack(STAKEinfo, &mut pdaSTAKE.try_borrow_mut_data()?)?;

        Ok(())
    }
}

