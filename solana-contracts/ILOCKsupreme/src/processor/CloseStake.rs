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
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let pdaGLOBAL = next_account_info(account_info_iter)?;
        let pdaACCOUNT = next_account_info(account_info_iter)?;
        let pdaSTAKE = next_account_info(account_info_iter)?;
        let pdaSTAKEend = next_account_info(account_info_iter)?;
        let pdaENTITY = next_account_info(account_info_iter)?;
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

        // unpack ACCOUNT flags
        let ACCOUNTflags = unpack_16_flags(ACCOUNTinfo.flags);

        // get ENTITY info
        let mut ENTITYinfo = ENTITY::unpack_unchecked(&pdaENTITY.try_borrow_data()?)?;

        // unpack ENTITY flags
        let ENTITYflags = unpack_16_flags(ENTITYinfo.flags);


        // get GLOBAL data
        let mut GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;

        // check that owner is *actually* owner
        if ACCOUNTinfo.owner != *owner.key && GLOBALinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }
        
        // get STAKE  data
        let mut STAKEinfo = STAKE::unpack_unchecked(&pdaSTAKE.try_borrow_data()?)?;

        // unpack STAKE flags
        let STAKEflags = unpack_16_flags(STAKEinfo.flags);
        
        let mut endSTAKEinfo = STAKE::unpack_unchecked(&pdaSTAKEend.try_borrow_data()?)?;

        // check if STAKE is also bounty hunter claim
        if ENTITYinfo.hunter != GLOBALinfo.owner &&     // entity is claimed by bounty hunter 
            ENTITYinfo.hunter == STAKEinfo.entity &&    // this stake is the entity claim stake
            STAKEflags[3] == ENTITYflags[9] {           // entity determination matches stake valence

            // reward and pay out bounty hunter
            let reward = GLOBALinfo.values[0] * GLOBALinfo.values[1];
                // values[0] is entity total stake threshold
                // values[1] is bounty hunter reward threshold percentage
            ACCOUNTinfo.rewards += reward;
            ACCOUNTinfo.balance += reward;
            GLOBALinfo.rewards -= reward;
            ACCOUNT::pack(ACCOUNTinfo, &mut pdaACCOUNT.try_borrow_mut_data()?)?;
            GLOBAL::pack(GLOBALinfo, &mut pdaGLOBAL.try_borrow_mut_data()?)?;
        }

        // rearrange stake accounts to make index sequential
        STAKEinfo.identifier = endSTAKEinfo.identifier;
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

