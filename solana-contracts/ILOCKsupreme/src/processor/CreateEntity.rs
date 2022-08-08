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
        //error::error::ContractError::GlobalAlreadyExistsError,
        processor::run::Processor,
        utils::utils::*,
        error::error::ContractError::*,
        state::{
            GLOBAL::*,
            USER::*,
            STAKE::*,
            ENTITY::*,
        },
    };

// for this instruction, the expected accounts are:

impl Processor {

    pub fn process_create_entity(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        bumpSTAKE: u8,
        seedSTAKE: Vec<u8>,
        bumpENTITY: u8,
        seedENTITY: Vec<u8>, //ENTITY seed will be derived from 
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
        let hash = next_account_info(account_info_iter)?; // delete.. baked into PDA. [?]
        let rent = next_account_info(account_info_iter)?;
        let clock = next_account_info(account_info_iter)?;

        // get GLOBAL data
        let GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;

        // get USER data
        let mut USERinfo = USER::unpack_unchecked(&pdaUSER.try_borrow_data()?)?;
        let USERflags = unpack_16_flags(USERinfo.flags);

        // convert serialized valence from u8 into boolean
        let valence_bool: bool;
        if valence == 0 { valence_bool = false } else { valence_bool = true }

        // get current time
        let timestamp = Clock::from_account_info(&clock)?.unix_timestamp;

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // check that owner is *actually* owner
        if USERinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        // make sure USER has balance for stake amount
        if USERinfo.balance < amount {
            return Err(InsufficientBalanceError.into())
        }

        // calculate rent and create pda ENTITY
        let rentENTITY = Rent::from_account_info(rent)?
            .minimum_balance(SIZE_ENTITY.into());
        invoke_signed(
        &system_instruction::create_account(
            &owner.key,
            &pdaENTITY.key,
            rentENTITY,
            SIZE_ENTITY.into(),
            &program_id,
        ),
        &[
            owner.clone(),
            pdaENTITY.clone(),
        ],
        &[&[&seedENTITY, &[bumpENTITY]]]
        )?;
        msg!("Successfully created pdaENTITY");


        // initialize ENTITY data
        let mut ENTITYinfo = ENTITY::unpack_unchecked(&pdaENTITY.try_borrow_data()?)?;

        // init flags
        let mut ENTITYflags = BitVec::from_elem(16, false);
            // false                            // 0: account type is ENTITY == 010
            ENTITYflags.set(1, true);           // 1: account type is ENTITY == 010
            // false                            // 2: account type is ENTITY == 010
            // false                            // 3: stake total minimum threshold triggered
            // false                            // 4: time total minimum threshold triggered
            // false                            // 5: staker number total minumum threshold triggered
            // false                            // 6: entity settled status
            // false                            // 7: entity settling status
            // false                            // 8: entity valence
            // false                            // 9: entity determination
            if USERflags[3] {
                ENTITYflags.set(10, true);      // 10: entity claimed by bounty hunter
            }
            // false                            // 11: bounty hunter rewarded

        // calculate rent and create pda STAKE
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
        msg!("Successfully created pdaSTAKE");

        // cover rent costs by transferring lamp to owner
        **pdaGLOBAL.try_borrow_mut_lamports()? -= rentSTAKE;
        **owner.try_borrow_mut_lamports()? += rentSTAKE;

        **pdaGLOBAL.try_borrow_mut_lamports()? -= rentENTITY;
        **owner.try_borrow_mut_lamports()? += rentENTITY;

        // initialize STAKE data
        let mut STAKEinfo = STAKE::unpack_unchecked(&pdaSTAKE.try_borrow_data()?)?;

        // init flags
        let mut STAKEflags = BitVec::from_elem(16, false);
            // false                            // 0: account type is STAKE == 001
            // false                            // 1: account type is STAKE == 001
            STAKEflags.set(2, true);            // 2: account type is STAKE == 001
            STAKEflags.set(3, valence_bool);    // 3: STAKE valence, high == good

        // update STAKE
        STAKEinfo.flags = pack_16_flags(STAKEflags);
        STAKEinfo.entity = *hash.key;
        STAKEinfo.amount = amount;
        STAKEinfo.timestamp = timestamp;
        STAKE::pack(STAKEinfo, &mut pdaSTAKE.try_borrow_mut_data()?)?;

        // if entity creator is a bounty hunter, declare them the owner
        // if entity created just from regulare security staker, entity is owned by global
        if USERflags[3] && USERinfo.owner == *owner.key {
            ENTITYinfo.hunter = *pdaUSER.key;
        } else {
            ENTITYinfo.hunter = GLOBALinfo.owner;
        }

        // set ENTITY valence and time
        ENTITYflags.set(8, valence_bool);
        ENTITYinfo.timestamp = timestamp;
        ENTITYinfo.stakers = 1;
        if valence_bool { ENTITYinfo.stakepos += amount } else { ENTITYinfo.stakeneg += amount }
        ENTITYinfo.flags = pack_16_flags(ENTITYflags);
        ENTITY::pack(ENTITYinfo, &mut pdaENTITY.try_borrow_mut_data()?)?;

        // update USER
        USERinfo.balance -= amount;
        USERinfo.count += 1;
        USER::pack(USERinfo, &mut pdaUSER.try_borrow_mut_data()?)?;

        Ok(())
    }
}

