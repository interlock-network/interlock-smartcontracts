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
            USER::*,
        },
    };

// for this instruction, the expected accounts are:

impl Processor {

    pub fn process_create_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bumpUSER: u8,
        seedUSER: Vec<u8>,
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let pdaGLOBAL = next_account_info(account_info_iter)?;
        let pdaUSER = next_account_info(account_info_iter)?;
        let rent = next_account_info(account_info_iter)?;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // calculate rent if we want to create new account
        let rentUSER = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_USER.into());

        // create pdaUSER
        invoke_signed(
        &system_instruction::create_account(
            &pdaGLOBAL.key,
            &pdaUSER.key,
            rentUSER,
            SIZE_USER.into(),
            &program_id,
        ),
        &[
            pdaGLOBAL.clone(),
            pdaUSER.clone(),
        ],
        &[&[&seedUSER, &[bumpUSER]]]
        )?;
        msg!("Successfully created pdaUSER");
// need to determine if create_account reverts if account already exists
        
        // get unititialized USER data
        let mut USERinfo = USER::unpack_unchecked(&pdaUSER.try_borrow_data()?)?;
        
        // init flags
        let flags = BitVec::from_elem(16, false);

            // account type is USER = 000
            // flags[0] = false;
            // flags[1] = false;
            // flags[2] = false;
            // is bounty hunter? init false
            // flags[3] = false;
            // is connected to Ethereum? init false
            // flags[4] = false;

        // populate and pack USER info
        USERinfo.flags = pack_16_flags(flags);
        USERinfo.owner = *owner.key;
        USERinfo.balance = 0;
        USER::pack(USERinfo, &mut pdaUSER.try_borrow_mut_data()?)?;

        Ok(())
    }
}

