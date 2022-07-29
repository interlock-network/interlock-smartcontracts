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
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
        msg,
    };

use crate::{
        error::error::ContractError::OwnerImposterError,
        processor::run::Processor,
        utils::utils::*,
        state::{
            GLOBAL::*,
        },
    };

// for this instruction, the expected accounts are:

impl Processor {

    pub fn process_update_global(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        updateFlags1: u32,
        updateFlags2: u32,
        values: [u32; VALUES],
    ) -> ProgramResult {

        // iterate and get accounts
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let pdaGLOBAL = next_account_info(account_info_iter)?;
        let newOwner = next_account_info(account_info_iter)?;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        
        // get GLOBAL account info
        let mut GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;

        // check that owner is *actually* owner
        if GLOBALinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        // if newOwner is different than owner, set new owner and return
        if owner.key != newOwner.key {
            GLOBALinfo.owner = *newOwner.key;
            GLOBAL::pack(GLOBALinfo, &mut pdaGLOBAL.try_borrow_mut_data()?)?;
            msg!("Owner changed to {:?}", *newOwner.key);
            return Ok(())
        }
        
        // unpack ix data flags specifying which global variable to update
        let flags1 = unpack_32_flags(updateFlags1);
        let flags2 = unpack_32_flags(updateFlags2);

        // . check for values that need to be updated
        // . flag high => value is to be changed
        let mut i = 0;
        for flag in &flags1 {
            if flag {GLOBALinfo.values[i] = values[i]}
            i += 1;
        }

        let mut i = 32;
        for flag in &flags2 {
            if flag {GLOBALinfo.values[i] = values[i]}
            i += 1;
        }

        // populate and pack GLOBAL account info
        GLOBALinfo.owner = *owner.key;
        GLOBAL::pack(GLOBALinfo, &mut pdaGLOBAL.try_borrow_mut_data()?)?;

        Ok(())
    }
}

