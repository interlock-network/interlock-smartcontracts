/****************************************************************
 * ILOCKsupreme Solana Contract
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
        },
    };

// for this instruction, the expected accounts are:

impl Processor {

    pub fn process_close_stake(
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
        let pdaACCOUNT = next_account_info(account_info_iter)?;
        let pdaSTAKE = next_account_info(account_info_iter)?;
        let pdaSTAKEend = next_account_info(account_info_iter)?;
        let hash = next_account_info(account_info_iter)?;
        let rent = next_account_info(account_info_iter)?;

        // calculate rent if we want to create new account
        let rentSTAKE = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_STAKE.into());

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // get ACCOUNT info
        let mut ACCOUNTinfo = ACCOUNT::unpack_unchecked(&pdaACCOUNT.try_borrow_data()?)?;

        // check that owner is *actually* owner
        if ACCOUNTinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }
        
        // get STAKE  data
        let mut STAKEinfo = STAKE::unpack_unchecked(&pdaSTAKE.try_borrow_data()?)?;
        let mut endSTAKEinfo = STAKE::unpack_unchecked(&pdaSTAKEend.try_borrow_data()?)?;

        // return tokens to ACCOUNT
        ACCOUNTinfo.balance += STAKEinfo.amount;
        
        STAKEinfo.identifier = endSTAKEinfo.identifier;
        STAKEinfo.amount = endSTAKEinfo.amount;
        STAKE::pack(STAKEinfo, &mut pdaSTAKE.try_borrow_mut_data()?)?;

        // close last account in index
        **pdaSTAKEend.try_borrow_mut_lamports()? -= rentSTAKE;

        Ok(())
    }
}

