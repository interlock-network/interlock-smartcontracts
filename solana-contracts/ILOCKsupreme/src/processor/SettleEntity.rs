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
            USER::*,
        },
    };

// for this instruction, the expected accounts are:

impl Processor {

    pub fn process_settle_entity(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        determination: u8,
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let pdaGLOBAL = next_account_info(account_info_iter)?;
        let pdaUSER = next_account_info(account_info_iter)?;
        let pdaENTITY = next_account_info(account_info_iter)?;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // get GLOBAL data
        let mut GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;

        // check that owner is *actually* GLOBAL owner
        // only Interlock Network owner can settle entity
        if GLOBALinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        // get ENTITY  data
        let mut ENTITYinfo = ENTITY::unpack_unchecked(&pdaENTITY.try_borrow_data()?)?;
        
        // unpack flags here 
        let mut ENTITYflags = unpack_16_flags(ENTITYinfo.flags);

        // entity is officially determined as of this ix running
        ENTITYflags[6] = true;

        // entity is of determination provided by caller
        ENTITYflags[9] = determination as bool;

        // repack new flag states
        ENTITYinfo.flags = pack_16_flags(ENTITYflags);

        // store flag state
        ENTITY::pack(ENTITYinfo, &mut pdaENTITY.try_borrow_mut_data()?)?;


        Ok(())
    }
}

