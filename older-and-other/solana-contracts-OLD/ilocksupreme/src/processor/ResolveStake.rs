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

    pub fn process_resolve_stake(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        seedSTAKE:    Vec<u8>,
    ) -> ProgramResult {

        // it is customary to iterate through accounts like so
        let account_info_iter = &mut accounts.iter();
        let owner = next_account_info(account_info_iter)?;
        let pdaGLOBAL = next_account_info(account_info_iter)?;
        let pdaUSER = next_account_info(account_info_iter)?;
        let pdaSTAKE = next_account_info(account_info_iter)?;
        let pdaENTITY = next_account_info(account_info_iter)?;

        // get GLOBAL data
        let mut GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;
        
        // get USER info
        let mut USERinfo = USER::unpack_unchecked(&pdaUSER.try_borrow_data()?)?;

        // get STAKE  data
        let mut STAKEinfo = STAKE::unpack_unchecked(&pdaSTAKE.try_borrow_data()?)?;
        let mut STAKEflags = unpack_16_flags(STAKEinfo.flags);

        // get ENTITY info
        let ENTITYinfo = ENTITY::unpack_unchecked(&pdaENTITY.try_borrow_data()?)?;
        let ENTITYflags = unpack_16_flags(ENTITYinfo.flags);

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // make sure entity is settled
        if !ENTITYflags[6] {
            return Err(EntityNotSettledError.into());
        }

        // check that owner is *actually* owner
        if USERinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        // verify STAKE is USER's
        let pdaUSERstring = pdaUSER.key.to_string();
        let (pdaSTAKEcheck, _) = Pubkey::find_program_address(&[&seedSTAKE], &program_id);
        if &seedSTAKE[0..(PUBKEY_LEN - U16_LEN)] !=
            pdaUSERstring[0..(PUBKEY_LEN - U16_LEN)].as_bytes() ||  // STAKE seed contains pdaUSER address
            pdaSTAKEcheck != *pdaSTAKE.key {                        // address generated from seed matches STAKE
            return Err(NotUserStakeError.into());
        }
     
        // compute continuous exponential return
        //
        //
        // FORMULA: Return(t) = Stake * exp(rate * t)
        //
        // We approximate this by taking the first
        // four terms of the Taylor Series, where,
        //
        // exp(x) = (x^0/0!) + (x^1/1!) + (x^2/2!) + (x^3/3!) + ...
        //        = 1 + x + x^2/2 + x^3/6 + ...
        //
        // Because Solana does not deal with floating points,
        // we need to express the interest rate as
        // exp(time*rate/ratefactor), where rate factor is the integer
        // that scales the rate
        //
        // For example, a rate of 3.55% will be expressed as 355/10_000,
        // where rate = 355 and ratefactor = 10_000
        //
        // The new approximation looks like this
        // exp(x/y) = (x^0/(0!*y^0)) + (x^1/(1!*y^1)) + (x^2/(2!*y^2)) + (x^3/(3!*y^3)) + ...
        //        = 1 + x/y + x^2/(2*y^2) + x^3/(6*y^3) + ...
        //
        // In above, x is 'exp_numerator', while y is 'exp_denominator'
        
        let time = ENTITYinfo.timestamp - STAKEinfo.timestamp;      // time elapsed in seconds
        let timefactor = GLOBALinfo.values[11];                     // time adjustment factor
        let rate = GLOBALinfo.values[3];                            // interest rate
        let ratefactor = GLOBALinfo.values[10];                     // rate adjustment factor

        let exp_numerator = time as u128 * rate as u128;               // continuously compounding exponential factor's numerator
        let exp_denominator = timefactor as u128 * ratefactor as u128; // continuously compounding exponential factor's numerator

        // compute terms in approximation
        // note the STAKEinfo.amount must be applied within brackets in numerator to make sure
        // result is not a decimal value (no floating point with solana)
        let term0 = STAKEinfo.amount;
        let term1 = (exp_numerator*STAKEinfo.amount)/
                     exp_denominator;
        let term2 = (exp_numerator*exp_numerator*STAKEinfo.amount)/
                    (2*exp_denominator*exp_denominator);
        let term3 = (exp_numerator*exp_numerator*exp_numerator*STAKEinfo.amount)/
                    (6*exp_denominator*exp_denominator*exp_denominator);

        // ( bear in mind, term1 >> term2 >> term3 )

        let stake_payout = term0 + term1 + term2 + term3; // computer final continuously compounding approximation
        let stake_yield = stake_payout - STAKEinfo.amount;           // compute final stake yield to be paid out

        // pay reward and return stake principal
        let stake_reward = STAKEinfo.amount * GLOBALinfo.values[9] as u128;

        // if stake matches determination
        if STAKEflags[3] == ENTITYflags[9] {

            // transfer reward stake and stake_yield to USER
            USERinfo.balance += stake_reward + STAKEinfo.amount + stake_yield;
            USERinfo.rewards += stake_reward;
            USERinfo.success += 1;
            GLOBALinfo.pool -= stake_reward + stake_yield;
            STAKEinfo.amount = 0;

        // if stake does not match determination
        } else {

            // transfer stake_yield only to USER and slash reward and principal
            USERinfo.balance += stake_yield;
            USERinfo.fail += 1;
            GLOBALinfo.pool += STAKEinfo.amount - stake_yield;
            STAKEinfo.amount = 0;
        }

        // set STAKE to 'resolved'
        STAKEflags.set(4, true);

        // update all
        STAKEinfo.flags = pack_16_flags(STAKEflags);
        STAKE::pack(STAKEinfo, &mut pdaSTAKE.try_borrow_mut_data()?)?;
        USER::pack(USERinfo, &mut pdaUSER.try_borrow_mut_data()?)?;
        GLOBAL::pack(GLOBALinfo, &mut pdaGLOBAL.try_borrow_mut_data()?)?;

        Ok(())
    }
}

