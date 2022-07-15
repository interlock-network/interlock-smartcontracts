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
        clock::Clock,
    };

use crate::{
        error::error::ContractError::*,
        processor::run::Processor,
        utils::utils::*,
        state::{
            GLOBAL::*,
            STAKE::*,
            ENTITY::*,
            USER::*,
        },
    };

// for this instruction, the expected accounts are:

impl Processor {

    pub fn process_close_stake(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let pdaGLOBAL = next_account_info(account_info_iter)?;
        let pdaUSER = next_account_info(account_info_iter)?;
        let pdaSTAKE = next_account_info(account_info_iter)?;
        let pdaENTITY = next_account_info(account_info_iter)?;
        let clock = next_account_info(account_info_iter)?;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // get ENTITY info
        let ENTITYinfo = ENTITY::unpack_unchecked(&pdaENTITY.try_borrow_data()?)?;

        // unpack ENTITY flags
        let ENTITYflags = unpack_16_flags(ENTITYinfo.flags);

        // make sure entity is settled
        if !ENTITYflags[6] {
            return Err(EntityNotSettledError.into());
        }

        // get GLOBAL data
        let mut GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;
        
        // get USER info
        let mut USERinfo = USER::unpack_unchecked(&pdaUSER.try_borrow_data()?)?;

        // check that owner is *actually* owner
        if USERinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        // get STAKE  data
        let mut STAKEinfo = STAKE::unpack_unchecked(&pdaSTAKE.try_borrow_data()?)?;

        // unpack STAKE flags
        let mut STAKEflags = unpack_16_flags(STAKEinfo.flags);
        
        // compute time delta
        let timedelta = ENTITYinfo.timestamp - STAKEinfo.timestamp;

        // compute continuous exponential return
        //
        // FORMULA: Return(t) = Stake * exp(rate * t)
        //
        // We approximate this by taking the first
        // four terms of the Taylor Series, where,
        //
        // exp(x) = (x^0/0!) + (x^1/1!) + (x^2/2!) + (x^3/3!) + ...
        //        = 1 + x + x^2/2 + x^3/6 + ...
        //
        let rate = GLOBALinfo.values[3];
        let exponent = rate * timedelta;
        let payout = STAKEinfo.amount * (1 + exponent + (exponent*exponent)/2 + (exponent*exponent*exponent)/6);
        let yield = payout - STAKEinfo.amount;

        // pay reward and return stake principal
        let reward = STAKEinfo.amount * GLOBALinfo.values[9];

        // if stake matches determination
        if STAKEflags[3] == ENTITYflags[9] {

            // transfer reward stake and yield to USER
            USERinfo.balance += reward + STAKEinfo.amount + yield;
            USERinfo.rewards += reward;
            GLOBALinfo.pool -= reward + yield;
            STAKEinfo.amount = 0;

        } else {

            // transfer yield only to USER
            USERinfo.balance += yield;
            GLOBALinfo.pool += STAKEinfo.amount - yield;
            STAKEinfo.amount = 0;
        }

        // set STAKE to 'resolved'
        STAKEflags.set(4, true);

        // repack flags and STAKE info
        STAKEinfo.flags = pack_16_flags(STAKEflags);
        STAKE::pack(STAKEinfo, &mut pdaSTAKE.try_borrow_mut_data()?)?;

        Ok(())
    }
}

