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
            ENTITY::*,
        },
    };

// for this instruction, the expected accounts are:

impl Processor {

    pub fn process_close_entity(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let pdaGLOBAL = next_account_info(account_info_iter)?;
        let pdaENTITY = next_account_info(account_info_iter)?;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // get ENTITY info
        let ENTITYinfo = ENTITY::unpack_unchecked(&pdaENTITY.try_borrow_data()?)?;

        // unpack ENTITY flags
        let ENTITYflags = unpack_16_flags(ENTITYinfo.flags);

        // make sure entity is settled
        if ENTITYflags[6] == false {
            return Err(EntityNotSettledError.into());
        }

        // get GLOBAL data
        let GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;

        // check that owner is *actually* owner
        if GLOBALinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }
        
        // transfer rent lamps back into GLOBAL pool
        let pdaGLOBALlamp = pdaGLOBAL.lamports();
        **pdaGLOBAL.lamports.borrow_mut() = pdaGLOBALlamp
            .checked_add(pdaENTITY.lamports())
            .unwrap();
        **pdaENTITY.lamports.borrow_mut() = 0;
        let mut pdaENTITYdata = pdaENTITY.data.borrow_mut();
        pdaENTITYdata.fill(0);

        Ok(())
    }
}

