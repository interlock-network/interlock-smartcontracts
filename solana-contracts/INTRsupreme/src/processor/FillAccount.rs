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

    pub fn process_fill_account(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {


        // BEFORE SENDING THIS IX, MVP INITIATED ETHEREUM WORMHOLE INTR TRANSFER
        // AND -- SERVER (OR MVP CLIENT) PUSHED VALIDATED MESSAGE TO SOLANA


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

        // get GLOBAL account info
        let mut ACCOUNTinfo = ACCOUNT::unpack_unchecked(&pdaACCOUNT.try_borrow_data()?)?;

        // check that owner is *actually* owner
        if ACCOUNTinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        // HERE, CALL WORMHOLE TOKEN BRIDGE PROGRAM, PERHAPS A TINY INTERLOCK PROGRAM THAT DOES
        // THIS, TO MINT SLP TOKEN
        // 
        // SOMEHOW GET THE AMOUNT BACK TO THIS PROGRAM IX
        // ( NOT SURE HOW TO DO THIS )

        // HERE, UPDATE ACCOUNT BALANCE AND LET SPL TOKEN JUST SIT THERE 
        //

        ACCOUNTinfo.balance = 0;
        ACCOUNT::pack(ACCOUNTinfo, &mut pdaACCOUNT.try_borrow_mut_data()?)?;

        Ok(())
    }
}

