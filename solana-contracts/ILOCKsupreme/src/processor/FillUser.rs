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
        state::{
            USER::*,
        },
    };

// for this instruction, the expected accounts are
//
// 0, owner pubkey, is signer
// 1, GLOBAL pda
// 2, system rent account
// 3, register USER pda

impl Processor {

    pub fn process_fill_user(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u128,
    ) -> ProgramResult {


        // BEFORE SENDING THIS IX, MVP INITIATED ETHEREUM WORMHOLE ILOCK TRANSFER
        // AND -- SERVER (OR MVP CLIENT) PUSHED VALIDATED MESSAGE TO SOLANA


        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let pdaUSER = next_account_info(account_info_iter)?;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // get GLOBAL account info
        let mut USERinfo = USER::unpack_unchecked(&pdaUSER.try_borrow_data()?)?;

        // check that owner is *actually* owner
        if USERinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        USERinfo.balance += amount;

        // HERE, CALL WORMHOLE TOKEN BRIDGE PROGRAM, PERHAPS A TINY INTERLOCK PROGRAM THAT DOES
        // THIS, TO MINT SLP TOKEN
        // 
        // SOMEHOW GET THE AMOUNT BACK TO THIS PROGRAM IX
        // ( NOT SURE HOW TO DO THIS )

        // HERE, UPDATE USER BALANCE AND LET SPL TOKEN JUST SIT THERE 
        //

        USER::pack(USERinfo, &mut pdaUSER.try_borrow_mut_data()?)?;

        Ok(())
    }
}

