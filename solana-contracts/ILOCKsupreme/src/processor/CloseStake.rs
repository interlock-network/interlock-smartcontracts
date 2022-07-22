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

    pub fn process_close_stake(
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
        let pdaSTAKEend = next_account_info(account_info_iter)?;
        let pdaENTITY = next_account_info(account_info_iter)?;

        // get GLOBAL
        let mut GLOBALinfo = GLOBAL::unpack_unchecked(&pdaGLOBAL.try_borrow_data()?)?;

        // get USER
        let mut USERinfo = USER::unpack_unchecked(&pdaUSER.try_borrow_data()?)?;

        // get STAKE  data
        let mut STAKEinfo = STAKE::unpack_unchecked(&pdaSTAKE.try_borrow_data()?)?;
        let STAKEflags = unpack_16_flags(STAKEinfo.flags);

        // get STAKE at end of USER account stake index
        let endSTAKEinfo = STAKE::unpack_unchecked(&pdaSTAKEend.try_borrow_data()?)?;

        // get ENTITY
        let mut ENTITYinfo = ENTITY::unpack_unchecked(&pdaENTITY.try_borrow_data()?)?;
        let mut ENTITYflags = unpack_16_flags(ENTITYinfo.flags);

        // check to make sure tx sender is signer
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // make sure entity is settled
        if !ENTITYflags[6] {
            return Err(EntityNotSettledError.into());
        }

        // check that owner is *actually* owner
        if USERinfo.owner != *owner.key && GLOBALinfo.owner != *owner.key {
            return Err(OwnerImposterError.into());
        }

        // make sure stake is resolved first
        // ...before closing, yields or penalitizies must be processed
        if !STAKEflags[4] {
            return Err(StakeNotResolvedError.into());
        }

        // verify STAKE is USER's
        let pdaUSERstring = pdaUSER.key.to_string();
        let (pdaSTAKEcheck, _) = Pubkey::find_program_address(&[&seedSTAKE], &program_id);
        if &seedSTAKE[0..(PUBKEY_LEN - U16_LEN)] !=
            pdaUSERstring[0..(PUBKEY_LEN - U16_LEN)].as_bytes() ||  // STAKE seed contains pdaUSER address
            pdaSTAKEcheck != *pdaSTAKE.key {                        // address generated from seed matches STAKE
            return Err(NotUserStakeError.into());
        }

        // check and reward bounty hunter
        if ENTITYflags[10]  &&                      // entity is claimed by bounty hunter 
            !ENTITYflags[11] &&                     // bounty not yet rewarded
            ENTITYinfo.hunter == *pdaUSER.key &&    // USER is ENTITY hunter
            STAKEflags[3] == ENTITYflags[9] {       // entity determination matches stake valence

            // reward and pay out bounty hunter
            let reward = GLOBALinfo.values[0] * GLOBALinfo.values[1];
                // values[0] is entity total stake threshold
                // values[1] is bounty hunter reward threshold percentage
            USERinfo.rewards += reward as u128;
            USERinfo.balance += reward as u128;
            GLOBALinfo.pool -= reward as u128;
            GLOBAL::pack(GLOBALinfo, &mut pdaGLOBAL.try_borrow_mut_data()?)?;

            // bounty rewarded
            ENTITYflags.set(11, true);
        }

        // transfer rent lamps back into GLOBAL pool
        let pdaGLOBALlamp = pdaGLOBAL.lamports();
        **pdaGLOBAL.lamports.borrow_mut() = pdaGLOBALlamp
            .checked_add(pdaSTAKEend.lamports())
            .unwrap();
        **pdaSTAKEend.lamports.borrow_mut() = 0;
        let mut pdaSTAKEdata = pdaSTAKEend.data.borrow_mut();
        pdaSTAKEdata.fill(0);

        // update USER
        USERinfo.count -= 1;
        USER::pack(USERinfo, &mut pdaUSER.try_borrow_mut_data()?)?;

        // rearrange stake accounts to make index sequential
        // then, update STAKE
        STAKEinfo.entity = endSTAKEinfo.entity;
        STAKEinfo.amount = endSTAKEinfo.amount;
        STAKE::pack(STAKEinfo, &mut pdaSTAKE.try_borrow_mut_data()?)?;

        // update ENTITY
        ENTITYinfo.flags = pack_16_flags(ENTITYflags);
        ENTITY::pack(ENTITYinfo, &mut pdaENTITY.try_borrow_mut_data()?)?;

        Ok(())
    }
}

