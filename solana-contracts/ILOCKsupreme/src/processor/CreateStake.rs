/****************************************************************
 * ILOCKsupreme Solana Contract
 ****************************************************************/

#![allow(non_snake_case)]
use solana_program::{
        account_info::{
            next_account_info,
            AccountInfo
        },
        clock::Clock,
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
        processor::run::Processor,
        utils::utils::*,
        error::error::ContractError::*,
        state::{
            USER::*,
            STAKE::*,
            GLOBAL::*,
            ENTITY::*,
        },
    };

// for this instruction, the expected accounts are:

impl Processor {

    pub fn process_create_stake(
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
        let pdaUSER = next_account_info(account_info_iter)?;
        let pdaSTAKE = next_account_info(account_info_iter)?;
        let pdaENTITY = next_account_info(account_info_iter)?;
        let rent = next_account_info(account_info_iter)?;
        let hash = next_account_info(account_info_iter)?;
        let clock = next_account_info(account_info_iter)?;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // get user account info
        let mut USERinfo = USER::unpack_unchecked(&pdaUSER.try_borrow_data()?)?;

        // check that owner is *actually* owner
        if USERinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        // get user account info
        let ENTITYinfo = ENTITY::unpack_unchecked(&pdaENTITY.try_borrow_data()?)?;

        // unpack ENTITY flags
        let ENTITYflags = unpack_16_flags(ENTITYinfo.flags);

        // make sure ENTITY is not settling
        if ENTITYflags[7] {
            return Err(EntitySettlingError.into());
        }

        // get current time
        let timestamp = Clock::from_account_info(&clock)?.unix_timestamp;
        
        // computer time delta
        let timedelta = timestamp - ENTITYinfo.timestamp;

        // get GLOBAL data
        let GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;

        // is delta over threshold?
        if timedelta as u32 > GLOBALinfo.values[2] {
            return Err(TimeThresholdPassedError.into());
        }

        // calculate rent if we want to create new account
        let rentSTAKE = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_STAKE.into());

        // create pdaGLOBAL
        invoke_signed(
        &system_instruction::create_account(
            &pdaGLOBAL.key,
            &pdaSTAKE.key,
            rentSTAKE,
            SIZE_STAKE.into(),
            &program_id
        ),
        &[
            pdaGLOBAL.clone(),
            pdaSTAKE.clone(),
        ],
        &[&[&seedSTAKE, &[bumpSTAKE]]]
        )?;
        msg!("Successfully created pdaSTAKE account");
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
        STAKEinfo.timestamp = timestamp;
        STAKE::pack(STAKEinfo, &mut pdaSTAKE.try_borrow_mut_data()?)?;

        // credit USER
        USERinfo.balance -= amount;

        // increment USER STAKE account count
        USERinfo.count += 1;

        // pack USER info
        USER::pack(USERinfo, &mut pdaUSER.try_borrow_mut_data()?)?;

        Ok(())
    }
}

