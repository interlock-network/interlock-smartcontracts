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
    };

use crate::{
        error::error::ContractError::*,
        processor::run::Processor,
        utils::utils::*,
        state::{
            GLOBAL::*,
            USER::*,
        },
    };

// for this instruction, the expected accounts are:

impl Processor {

    pub fn process_set_hunter(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        status: u8,
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let pdaGLOBAL = next_account_info(account_info_iter)?;
        let pdaUSER = next_account_info(account_info_iter)?;

        // get GLOBAL
        let GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;

        // get USER
        let mut USERinfo = USER::unpack_unchecked(&pdaUSER.try_borrow_data()?)?;
        let mut USERflags = unpack_16_flags(USERinfo.flags);

        // convert serialized determination from u8 into boolean
        let status_bool: bool;
        if status == 0 { status_bool = false } else { status_bool = true }

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // check that owner is *actually* GLOBAL owner
        // only Interlock Network owner can settle entity
        if GLOBALinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        // check to see if USER is already hunter status
        if USERflags[3] == status_bool {
            return Err(HunterAlreadySetError.into());
        }

        // update USER
        USERflags.set(3, status_bool);          // hunter set
        USERinfo.flags = pack_16_flags(USERflags);
        USER::pack(USERinfo, &mut pdaUSER.try_borrow_mut_data()?)?;

        Ok(())
    }
}

