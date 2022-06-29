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
        },
    };

// for this instruction, the expected accounts are
//
// 0, owner pubkey, is signer
// 1, GLOBAL pda
// 2, system rent account
// 3, register ACCOUNT pda

impl Processor {

    pub fn process_create_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bumpACCOUNT: u8,
        seedACCOUNT: Vec<u8>,
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let pdaGLOBAL = next_account_info(account_info_iter)?;
        let rent = next_account_info(account_info_iter)?;
        let pdaACCOUNT = next_account_info(account_info_iter)?;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // calculate rent if we want to create new account
        let rentACCOUNT = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_ACCOUNT.into());

        // create pdaACCOUNT
        invoke_signed(
        &system_instruction::create_account(
            &owner.key,
            &pdaACCOUNT.key,
            rentACCOUNT,
            SIZE_ACCOUNT.into(),
            &program_id
        ),
        &[
            owner.clone(),
            pdaACCOUNT.clone()
        ],
        &[&[&seedACCOUNT, &[bumpACCOUNT]]]
        )?;
        msg!("Successfully created pdaACCOUNT");
// need to determine if create_account reverts if account already exists
        
        // get unititialized ACCOUNT data
        let mut ACCOUNTinfo = ACCOUNT::unpack_unchecked(&pdaACCOUNT.try_borrow_data()?)?;
        
        // init flags
        let flags = BitVec::from_elem(16, false);

        // populate and pack ACCOUNT info
        ACCOUNTinfo.flags = pack_16_flags(flags);
        ACCOUNTinfo.owner = *owner.key;
        ACCOUNTinfo.balance = 0;
        ACCOUNT::pack(ACCOUNTinfo, &mut pdaACCOUNT.try_borrow_mut_data()?)?;

        Ok(())
    }
}

