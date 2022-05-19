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
            FIRST::*,
            SECOND::*,
        },
    };

// for this instruction, the expected accounts are
//
// 0, operator pubkey
// 1, system rent account
// 2, pubkey for account type FIRST
// 3, pubkey for account type SEECOND

impl Processor {

    pub fn process_instruction_one(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _dataNumberA: u8,
        dataStringA: Vec<u8>,
        _dataNumberB: u64,
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let operator = next_account_info(account_info_iter)?;
        let rent = next_account_info(account_info_iter)?;
        let first = next_account_info(account_info_iter)?;
        let second = next_account_info(account_info_iter)?;

        // customary to check to make sure tx operator is signer
        if !operator.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // calculate rent if we want to create new account
        let _rentfirst = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_FIRST.into());
        let _rentsecond = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_SECOND.into());
       
        // create a FIRST account
        // omitted
        //
        // create a SECOND account
        // omitted
        //
        // do some stuff with data variables
        //
        // then learn how to pack and unpack accounts below
        
        // log confirmation
        msg!("I AM INSTRUCTION NUMBER ONE!");

        // get FIRST account info
        let mut FIRSTinfo = FIRST::unpack_unchecked(&first.try_borrow_data()?)?;

        // set flags
        let mut flags = BitVec::from_elem(16, false);
        flags.set(0, false); // some random flag values
        flags.set(1, false);
        flags.set(2, true);
        flags.set(3, false);
        flags.set(4, true);

        // populate and pack FIRST account info
        FIRSTinfo.flags = pack_flags(flags);
        FIRSTinfo.operator = *operator.key;
        FIRSTinfo.balance = 0;
        FIRSTinfo.stringy = pack_stringy(dataStringA);
        FIRST::pack(FIRSTinfo, &mut first.try_borrow_mut_data()?)?;

        // get SECOND account info
        let mut SECONDinfo = SECOND::unpack_unchecked(&second.try_borrow_data()?)?;

        // populate and pack SECOND account info
        SECONDinfo.operator = *operator.key;
        SECONDinfo.balance = 0;
        SECONDinfo.stringy = pack_stringy(vec!());
        SECOND::pack(SECONDinfo, &mut second.try_borrow_mut_data()?)?;

        Ok(())
    }
}

