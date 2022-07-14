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
            STAKE::*,
            ENTITY::*,
            USER::*,
        },
    };

// for this instruction, the expected accounts are:

impl Processor {

    pub fn process_close_stake(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let pdaGLOBAL = next_account_info(account_info_iter)?;
        let pdaUSER = next_account_info(account_info_iter)?;
        let pdaSTAKE = next_account_info(account_info_iter)?;
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
        let mut GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;
        
        // get USER info
        let mut USERinfo = USER::unpack_unchecked(&pdaUSER.try_borrow_data()?)?;

        // check that owner is *actually* owner
        if USERinfo.owner != *owner.key && GLOBALinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        // get STAKE  data
        let mut STAKEinfo = STAKE::unpack_unchecked(&pdaSTAKE.try_borrow_data()?)?;

        // unpack STAKE flags
        let mut STAKEflags = unpack_16_flags(STAKEinfo.flags);
        

        // set STAKE to 'resolved'
        STAKEflags.set(4, true);

        // repack flags and STAKE info
        STAKEinfo.flags = pack_16_flags(STAKEflags);
        STAKE::pack(STAKEinfo, &mut pdaSTAKE.try_borrow_mut_data()?)?;

        Ok(())
    }
}

