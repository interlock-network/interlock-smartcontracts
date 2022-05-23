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
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
        sysvar::{
            rent::Rent,
            Sysvar,
        },
        msg,
    };

use bit_vec::BitVec;

use crate::{
        processor::run::Processor,
        utils::utils::*,
        state::{
            GLOBAL::*,
        },
    };

// for this instruction, the expected accounts are
//
// 0, owner pubkey, is signer
// 1, GLOBAL pda
// 2, system rent account

impl Processor {

    pub fn process_program_init(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bumpGLOBAL: u8,
        seedGLOBAL: Vec<u8>,
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let pdaGLOBAL = next_account_info(account_info_iter)?
        let rent = next_account_info(account_info_iter)?;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // calculate rent if we want to create new account
        let rentGLOBAL = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_GLOBAL.into());

        // get GLOBAL account info
        let mut GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;
        let flags = unpack_flags(GLOBALinfo.flags);

        // revert if global account already created
        if flags[0] {
            return Err(ContractError::GlobalAreadyExistsError.into());
        }

        // create pdaGLOBAL
        invoke_signed(
        &system_instruction::create_account(
            &owner.key,
            &pdaGLOBAL.key,
            rentGLOBAL,
            SIZE_GLOBAL.into(),
            &program_id
        ),
        &[
            owner.clone(),
            pdaGLOBAL.clone()
        ],
        &[&[&seedGLOBAL, &[bumpGLOBAL]]]
        )?;
        msg!("Successfully created pdaGLOBAL");


        // set flag, #1 == global account initialized
        let mut flags = BitVec::from_elem(32, false);
        flags.set(1, true);

        // populate and pack GLOBAL account info
        GLOBALinfo.flags = pack_flags(flags);
        GLOBALinfo.owner = *owner.key;
        GLOBAL::pack(GLOBALinfo, &mut first.try_borrow_mut_data()?)?;

        Ok(())
    }
}

