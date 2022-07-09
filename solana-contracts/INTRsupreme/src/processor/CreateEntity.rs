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

    pub fn process_create_entity(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bumpSTAKE: u8,
        seedSTAKE: Vec<u8>,
        amount: u128,
        valence: u8,
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let pdaGLOBAL = next_account_info(account_info_iter)?;
        let pdaACCOUNT = next_account_info(account_info_iter)?;
        let pdaSTAKE = next_account_info(account_info_iter)?;
        let pdaENTITY = next_account_info(account_info_iter)?;
        let hash = next_account_info(account_info_iter)?; // delete.. baked into PDA. [?]
        let rent = next_account_info(account_info_iter)?;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // get ACCOUNT data
        let mut ACCOUNTinfo = ACCOUNT::unpack_unchecked(&pdaACCOUNT.try_borrow_data()?)?;

        let ACCOUNTflags = unpack_16_flags(ACCOUNTinfo);

        // calculate rent if we want to create new account
        let rentENTITY = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_ENTITY.into());

        // create pdaENTITY
        invoke_signed(
        &system_instruction::create_account(
            &owner.key,
            &pdaENTITY.key,
            rentSTAKE,
            SIZE_ENTITY.into(),
            &program_id
        ),
        &[
            owner.clone(),
            pdaENTITY.clone()
        ],
        &[&[&seedENTITY, &[bumpENTITY]]]
        )?;
        msg!("Successfully created pdaENTITY");
// need to determine if create_account reverts if account already exists


        // get unititialized ENTITY data
        let mut ENTITYinfo = ENTITY::unpack_unchecked(&pdaENTITY.try_borrow_data()?)?;

        // get GLOBAL data
        let mut GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;

        // if entity creator is a bounty hunter, declare them the owner
        // if entity created just from regulare security staker, entity is owned by global
        if ACCOUNTflags[3] == true && ACCOUNTinfo.owner == *owner.key {
            ENTITYinfo.hunter = ACCOUNTinfo.owner;
        } else {
            ENTITYinfo.hunter = GLOBALinfo.owner;
        }

        // init flags
        let ENTITYflags = BitVec::from_elem(16, false);

            // account type is ENTITY == 010
            // ENTITYflags[0] = false;
            ENTITYflags[1] = true;
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

        // create pdaSTAKE
        invoke_signed(
        &system_instruction::create_account(
            &program_id
            &owner.key,
            &pdaSTAKE.key,
            rentSTAKE,
            SIZE_STAKE.into(),
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
    
            // account type is STAKE == 001
            // flags[0] = false;
            // flags[1] = false;
            flags[2] = true;
            // stake valence
            flags[3] = valence as bool;

        // populate and pack GLOBAL account info
        STAKEinfo.flags = pack_16_flags(flags);
        STAKEinfo.identifier = *hash.key;
        STAKEinfo.amount = amount;
        STAKE::pack(STAKEinfo, &mut pdaSTAKE.try_borrow_mut_data()?)?;

        ACCOUNT::pack(ACCOUNTinfo, &mut pdaACCOUNT.try_borrow_mut_data()?)?;


        Ok(())
    }
}

