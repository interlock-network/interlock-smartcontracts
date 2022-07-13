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
        let pdaSTAKEend = next_account_info(account_info_iter)?;
        let pdaENTITY = next_account_info(account_info_iter)?;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // get USER info
        let mut USERinfo = USER::unpack_unchecked(&pdaUSER.try_borrow_data()?)?;

        // get ENTITY info
        let ENTITYinfo = ENTITY::unpack_unchecked(&pdaENTITY.try_borrow_data()?)?;

        // unpack ENTITY flags
        let ENTITYflags = unpack_16_flags(ENTITYinfo.flags);


        // get GLOBAL data
        let mut GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;

        // check that owner is *actually* owner
        if USERinfo.owner != *owner.key && GLOBALinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }
        
        // get STAKE  data
        let mut STAKEinfo = STAKE::unpack_unchecked(&pdaSTAKE.try_borrow_data()?)?;

        // unpack STAKE flags
        let STAKEflags = unpack_16_flags(STAKEinfo.flags);
        
        let endSTAKEinfo = STAKE::unpack_unchecked(&pdaSTAKEend.try_borrow_data()?)?;

        // check if STAKE is also bounty hunter claim
        if ENTITYinfo.hunter != GLOBALinfo.owner &&     // entity is claimed by bounty hunter 
            ENTITYinfo.hunter == STAKEinfo.entity &&    // this stake is the entity claim stake
            STAKEflags[3] == ENTITYflags[9] {           // entity determination matches stake valence

            // reward and pay out bounty hunter
            let reward = GLOBALinfo.values[0] * GLOBALinfo.values[1];
                // values[0] is entity total stake threshold
                // values[1] is bounty hunter reward threshold percentage
            USERinfo.rewards += reward as u128;
            USERinfo.balance += reward as u128;
            GLOBALinfo.rewards -= reward as u128;
            USER::pack(USERinfo, &mut pdaUSER.try_borrow_mut_data()?)?;
            GLOBAL::pack(GLOBALinfo, &mut pdaGLOBAL.try_borrow_mut_data()?)?;
        }

        // rearrange stake accounts to make index sequential
        STAKEinfo.entity = endSTAKEinfo.entity;
        STAKEinfo.amount = endSTAKEinfo.amount;
        STAKE::pack(STAKEinfo, &mut pdaSTAKE.try_borrow_mut_data()?)?;

        // transfer rent lamps back into GLOBAL pool
        let pdaGLOBALlamp = pdaGLOBAL.lamports();
        **pdaGLOBAL.lamports.borrow_mut() = pdaGLOBALlamp
            .checked_add(pdaSTAKEend.lamports())
            .unwrap();
        **pdaSTAKEend.lamports.borrow_mut() = 0;
        let mut pdaSTAKEdata = pdaSTAKEend.data.borrow_mut();
        pdaSTAKEdata.fill(0);

        Ok(())
    }
}

