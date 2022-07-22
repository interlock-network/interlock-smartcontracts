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
        clock::Clock,
        sysvar::{
            Sysvar,
            rent::Rent,
        },
        msg,
        system_instruction,
    };

use bit_vec::BitVec;

use crate::{
        error::error::ContractError::*,
        processor::run::Processor,
        utils::utils::*,
        state::{
            GLOBAL::*,
            ENTITY::*,
            USER::*,
            STAKE::*,
        },
    };

// for this instruction, the expected accounts are:

impl Processor {

    pub fn process_claim_entity(
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
        let hash = next_account_info(account_info_iter)?;
        let rent = next_account_info(account_info_iter)?;
        let clock = next_account_info(account_info_iter)?;

        // get GLOBAL
        let GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;

        // get USER info
        let USERinfo = USER::unpack_unchecked(&pdaUSER.try_borrow_data()?)?;

        // get ENTITY
        let mut ENTITYinfo = ENTITY::unpack_unchecked(&pdaENTITY.try_borrow_data()?)?;
        let mut ENTITYflags = unpack_16_flags(ENTITYinfo.flags);

        // convert serialized valence from u8 into boolean
        let valence_bool: bool;
        if valence == 0 { valence_bool = false } else { valence_bool = true }

        // get current time
        let timestamp = Clock::from_account_info(&clock)?.unix_timestamp;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // make sure entity is not settled
        if ENTITYflags[6] {
            return Err(EntitySettledError.into());
        }

        // make sure entity is not settling
        if ENTITYflags[7] {
            return Err(EntitySettlingError.into());
        }

        // make sure entity is not already claimed
        if ENTITYinfo.hunter != GLOBALinfo.owner || ENTITYflags[10] {
            return Err(EntityClaimedError.into());
        }

        // check that owner is *actually* owner
        if USERinfo.owner != *owner.key && GLOBALinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        // make sure user is hunter
        if !ENTITYflags[3] {
            return Err(UserNotHunterError.into());
        }

        // calculate rent and create pda STAKE account
        let rentSTAKE = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_STAKE.into());
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
        msg!("Successfully created pdaSTAKE");
        // get unititialized STAKE data
        let mut STAKEinfo = STAKE::unpack_unchecked(&pdaSTAKE.try_borrow_data()?)?;

        // init flags
        let mut STAKEflags = BitVec::from_elem(16, false);
            // false                            // 1: account type is STAKE == 001
            // false                            // 2: account type is STAKE == 001
            STAKEflags.set(2, true);            // 3: account type is STAKE == 001
            STAKEflags.set(3, valence_bool);    // 4: STAKE valence, high == good

        // populate and pack GLOBAL account info
        STAKEinfo.flags = pack_16_flags(STAKEflags);
        STAKEinfo.entity = *hash.key;
        STAKEinfo.amount = amount;
        STAKEinfo.timestamp = timestamp;
        STAKE::pack(STAKEinfo, &mut pdaSTAKE.try_borrow_mut_data()?)?;
        
        // update ENTITY
        ENTITYinfo.hunter = USERinfo.owner;
        ENTITYflags.set(10, true);              // ENTITY claimed by bounty hunter
        ENTITYinfo.flags = pack_16_flags(ENTITYflags);
        ENTITY::pack(ENTITYinfo, &mut pdaENTITY.try_borrow_mut_data()?)?;

        Ok(())
    }
}

