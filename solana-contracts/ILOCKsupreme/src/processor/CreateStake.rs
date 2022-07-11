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
        let pdaACCOUNT = next_account_info(account_info_iter)?;
        let pdaSTAKE = next_account_info(account_info_iter)?;
        let rent = next_account_info(account_info_iter)?;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // get user account info
        let mut ACCOUNTinfo = ACCOUNT::unpack_unchecked(&pdaACCOUNT.try_borrow_data()?)?;

        // check that owner is *actually* owner
        if ACCOUNTinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        // calculate rent if we want to create new account
        let rentSTAKE = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_STAKE.into());

        // get user account info
        let GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;

        // create pdaGLOBAL
        invoke_signed(
        &system_instruction::create_account(
            &GLOBALinfo.owner.key,
            &pdaSTAKE.key,
            rentSTAKE,
            SIZE_STAKE.into(),
            &program_id
        ),
        &[
            GLOBALinfo.owner.clone(),
            pdaSTAKE.clone()
        ],
        &[&[&seedSTAKE, &[bumpSTAKE]]]
        )?;
        msg!("Successfully created pdaSTAKE account");
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

        // credit ACCOUNT
        ACCOUNTinfo.balance -= amount;
        ACCOUNT::pack(ACCOUNTinfo, &mut pdaACCOUNT.try_borrow_mut_data()?)?;

        Ok(())
    }
}

