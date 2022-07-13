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
        error::error::ContractError::*,
        state::{
            GLOBAL::*,
            USER::*,
            STAKE::*,
            ENTITY::*,
        },
    };

// for this instruction, the expected accounts are:

impl Processor {

    pub fn process_create_entity(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bumpSTAKE: u8,
        seedSTAKE: Vec<u8>,
        bumpENTITY: u8,
        seedENTITY: Vec<u8>,
        amount: u128,
        valence: u8,
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let ownerGLOBAL = next_account_info(account_info_iter)?;
        let pdaGLOBAL = next_account_info(account_info_iter)?;
        let pdaUSER = next_account_info(account_info_iter)?;
        let pdaSTAKE = next_account_info(account_info_iter)?;
        let pdaENTITY = next_account_info(account_info_iter)?;
        let hash = next_account_info(account_info_iter)?; // delete.. baked into PDA. [?]
        let rent = next_account_info(account_info_iter)?;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // get USER data
        let mut USERinfo = USER::unpack_unchecked(&pdaUSER.try_borrow_data()?)?;

        // check that owner is *actually* owner
        if USERinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        let USERflags = unpack_16_flags(USERinfo.flags);

        // calculate rent if we want to create new account
        let rentENTITY = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_ENTITY.into());

        // get GLOBAL data
        let GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;

        // create pdaENTITY
        invoke_signed(
        &system_instruction::create_account(
            &ownerGLOBAL.key,
            &pdaENTITY.key,
            rentENTITY,
            SIZE_ENTITY.into(),
            &program_id,
        ),
        &[
            ownerGLOBAL.clone(),
            pdaENTITY.clone(),
        ],
        &[&[&seedENTITY, &[bumpENTITY]]]
        )?;
        msg!("Successfully created pdaENTITY");
// need to determine if create_account reverts if account already exists

        // get unititialized ENTITY data
        let mut ENTITYinfo = ENTITY::unpack_unchecked(&pdaENTITY.try_borrow_data()?)?;

        // if entity creator is a bounty hunter, declare them the owner
        // if entity created just from regulare security staker, entity is owned by global
        if USERflags[3] == true && USERinfo.owner == *owner.key {
            ENTITYinfo.hunter = USERinfo.owner;
        } else {
            ENTITYinfo.hunter = GLOBALinfo.owner;
        }

        // init flags
        let mut ENTITYflags = BitVec::from_elem(16, false);

            // account type is ENTITY == 010
            // ENTITYflags[0] = false;
            ENTITYflags.set(1, true);
            // ENTITYflags[2] = false;
            
            // stake total minimum threshold triggered
            // ENTITYflags[3] = false;
            // time total minimum threshold triggered
            // ENTITYflags[4] = false;
            // staker number total minumum threshold triggered
            // ENTITYflags[5] = false;
            // entity settled status
            // ENTITYflags[6] = false;
            // entity settling status
            // ENTITYflags[7] = false;
            // entity valence
            // ENTITYflags[8] = false;
            // entity determination
            // ENTITYflags[9] = false;

        // calculate rent if we want to create new account
        let rentSTAKE = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_STAKE.into());


        // create pdaSTAKE
        invoke_signed(
        &system_instruction::create_account(
            &ownerGLOBAL.key,
            &pdaSTAKE.key,
            rentSTAKE,
            SIZE_STAKE.into(),
            &program_id
        ),
        &[
            ownerGLOBAL.clone(),
            pdaSTAKE.clone(),
        ],
        &[&[&seedSTAKE, &[bumpSTAKE]]]
        )?;
        msg!("Successfully created pdaSTAKE");
// need to determine if create_account reverts if account already exists
        
        // get unititialized GLOBAL data
        let mut STAKEinfo = STAKE::unpack_unchecked(&pdaSTAKE.try_borrow_data()?)?;
        
        // convert serialized valence from u8 into boolean
        let valence_bool: bool;
        if valence == 0 {
            valence_bool = false;
        } else {
            valence_bool = true;
        }

        // init flags
        let mut flags = BitVec::from_elem(16, false);
    
            // account type is STAKE == 001
            // flags[0] = false;
            // flags[1] = false;
            flags.set(2, true);
            // stake valence
            flags.set(3, valence_bool);

        // populate and pack GLOBAL account info
        STAKEinfo.flags = pack_16_flags(flags);
        STAKEinfo.entity = *hash.key;
        STAKEinfo.amount = amount;
        STAKE::pack(STAKEinfo, &mut pdaSTAKE.try_borrow_mut_data()?)?;

        // credit account for stake amount
        USERinfo.balance -= amount;

        USER::pack(USERinfo, &mut pdaUSER.try_borrow_mut_data()?)?;

        Ok(())
    }
}

