/****************************************************************
 * ILOCKsupreme Solana Contract
 ****************************************************************/

#![allow(non_snake_case)]
use solana_program::{
        account_info::{
            next_account_info,
            AccountInfo
        },
        clock::Clock,
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
        processor::run::Processor,
        utils::utils::*,
        error::error::ContractError::*,
        state::{
            USER::*,
            STAKE::*,
            GLOBAL::*,
            ENTITY::*,
        },
    };

// for this instruction, the expected accounts are:

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
        let pdaUSER = next_account_info(account_info_iter)?;
        let pdaSTAKE = next_account_info(account_info_iter)?;
        let pdaENTITY = next_account_info(account_info_iter)?;
        let rent = next_account_info(account_info_iter)?;
        let hash = next_account_info(account_info_iter)?;
        let clock = next_account_info(account_info_iter)?;

        // get GLOBAL
        let GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;

        // get USER
        let mut USERinfo = USER::unpack_unchecked(&pdaUSER.try_borrow_data()?)?;

        // get ENTITY
        let mut ENTITYinfo = ENTITY::unpack_unchecked(&pdaENTITY.try_borrow_data()?)?;
        let mut ENTITYflags = unpack_16_flags(ENTITYinfo.flags);

        // get current time
        let timestamp = Clock::from_account_info(&clock)?.unix_timestamp;

        // computer time delta
        let timedelta = timestamp - ENTITYinfo.timestamp;        
        
        // convert serialized valence from u8 into boolean
        let valence_bool: bool;
        if valence == 0 { valence_bool = false } else { valence_bool = true }

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        
        // make sure USER has balance for stake amount
        if USERinfo.balance < amount {
            return Err(InsufficientBalanceError.into())
        }
        
        // check that owner is *actually* owner
        if USERinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        // make sure STAKE amount meets minimum
        if (amount as u32) < GLOBALinfo.values[5] {
            return Err(MinimumStakeNotMetError.into());
        }
        
        // make sure ENTITY is not settling
        if ENTITYflags[7] {
            return Err(EntitySettlingError.into());
        }

        // if unclaimed, enforce staking against entity valence
        if !ENTITYflags[10] && ENTITYflags[8] == valence_bool {
            return Err(WrongStakeValenceError.into());
        }

        // is staker number over threshold?
        if ENTITYinfo.stakers as u32 == GLOBALinfo.values[9] {
            // make sure entity is marked 'settling' then repack
            ENTITYflags.set(7, true);
            ENTITYflags.set(5, true);
            ENTITYinfo.flags = pack_16_flags(ENTITYflags);
            ENTITY::pack(ENTITYinfo, &mut pdaENTITY.try_borrow_mut_data()?)?;

            return Err(StakerCountThresholdPassedError.into());
        }

        // is delta over threshold?
        if timedelta as u32 > GLOBALinfo.values[2] {
            // make sure entity is marked 'settling' then repack
            ENTITYflags.set(7, true);
            ENTITYflags.set(4, true);
            ENTITYinfo.flags = pack_16_flags(ENTITYflags);
            ENTITY::pack(ENTITYinfo, &mut pdaENTITY.try_borrow_mut_data()?)?;

            return Err(TimeThresholdPassedError.into());

        }

        // is entity over total stake threshold?
        if (ENTITYinfo.stakepos + ENTITYinfo.stakeneg) > GLOBALinfo.values[0] as u128 {
            // make sure entity is marked 'settling' then repack
            ENTITYflags.set(7, true);
            ENTITYflags.set(3, true);
            ENTITYinfo.flags = pack_16_flags(ENTITYflags);
            ENTITY::pack(ENTITYinfo, &mut pdaENTITY.try_borrow_mut_data()?)?;
            
            return Err(TotalStakeThresholdPassedError.into());
        }

        // is entity over (+) threshold?
        if ENTITYinfo.stakepos > GLOBALinfo.values[7] as u128 && valence_bool {
            return Err(PositiveStakeThresholdPassedError.into());
        }

        // is entity over (-) threshold?
        if ENTITYinfo.stakeneg > GLOBALinfo.values[8] as u128 && !valence_bool {
            return Err(NegativeStakeThresholdPassedError.into());
        }

        // calculate rent and create pda STAKE account
        let rentSTAKE = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_STAKE.into());
        invoke_signed(
        &system_instruction::create_account(
            &owner.key,
            &pdaSTAKE.key,
            rentSTAKE,
            SIZE_STAKE.into(),
            &program_id
        ),
        &[
            owner.clone(),
            pdaSTAKE.clone(),
        ],
        &[&[&seedSTAKE, &[bumpSTAKE]]]
        )?;
        msg!("Successfully created pdaSTAKE account");

        // cover rent costs by transferring lamp to owner
        **pdaGLOBAL.try_borrow_mut_lamports()? -= rentSTAKE;
        **owner.try_borrow_mut_lamports()? += rentSTAKE;
        
        // get unititialized STAKE data
        let mut STAKEinfo = STAKE::unpack_unchecked(&pdaSTAKE.try_borrow_data()?)?;

        // init flags
        let mut STAKEflags = BitVec::from_elem(16, false);
            // false                            // 1: account type is STAKE == 001
            // false                            // 2: account type is STAKE == 001
            STAKEflags.set(2, true);            // 3: account type is STAKE == 001
            STAKEflags.set(3, valence_bool);    // 4: STAKE valence, high == good

        // update STAKE
        STAKEinfo.flags = pack_16_flags(STAKEflags);    
        STAKEinfo.entity = *hash.key;       // URL hash is STAKE entity identifier
        STAKEinfo.amount = amount;          // stake amount set accordingly
        STAKEinfo.timestamp = timestamp;    // time of STAKE creation (now)
        STAKE::pack(STAKEinfo, &mut pdaSTAKE.try_borrow_mut_data()?)?;

        // update USER
        USERinfo.balance -= amount;         // deduct stake amount from USER token balance
        USERinfo.count += 1;                // increment number of stakes for USER
        USER::pack(USERinfo, &mut pdaUSER.try_borrow_mut_data()?)?;

        // update ENTITY
        if valence_bool { ENTITYinfo.stakepos += amount } else { ENTITYinfo.stakeneg += amount }
        ENTITYinfo.stakers += 1;            // increment number of stakes for ENTITY
        ENTITY::pack(ENTITYinfo, &mut pdaENTITY.try_borrow_mut_data()?)?;

        Ok(())
    }
}

