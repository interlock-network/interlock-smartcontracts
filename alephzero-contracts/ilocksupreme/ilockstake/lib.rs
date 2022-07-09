// INTERLOCK NETWORK
//
// blairmunroakusa@0742Tue.28Jun22.anch.AK:south
//
// THIS IS A PROTOTYPE STAKING CONTRACT
// USING INK! FRAMEWORK



// !!!!! INCOMPLETE AND FLAWED, WARNING !!!!!

// NOTES: this contract bridges the data and methods of the token contract and the stake data
// contract. Again, an obnoxious problem is the event emit issue, hack being to declare events
// elsewhere and create emit methods. I still need to implmement stake claim and reward functions,
// but honestly this cannot happen until I figure out the staker contract issue outlined in
// stakerdata contract NOTES.


#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::ilockstake::{
    ILOCKstake,
    ILOCKstakeRef,
};

use ink_lang as ink;

#[ink::contract]
pub mod ilockstake {

    use ilocktoken::ILOCKtokenRef;
    use stakedata::StakeDataRef;

    use ink_lang::utils::initialize_contract;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;

    /// defines contract storage


    #[ink(storage)]
    pub struct ILOCKstake {
        ilocktoken: ILOCKtokenRef,
        stakedata: StakeDataRef,
    }

    impl ILOCKstake {
        
        /// Constructor that initializes staking contract
        #[ink(constructor)]
        pub fn new(
            ilocktoken: ILOCKtokenRef,
            stakedata: StakeDataRef,
        ) -> Self {

            Self {
                ilocktoken,
                stakedata,
            }
        }

        #[ink(message)]
        pub fn get_stake(&self, staker: AccountId, hash: Hash) -> u32 {
            
            self.stakedata.get_stake(staker, hash)
        }

        #[ink(message)]
        pub fn stake_url(&mut self, staker: AccountId, hash: Hash, amount: u32) -> bool {

            self.stakedata.update_stake(staker, hash, amount);
            self.stakedata.update_hash(hash, staker, amount);
            self.ilocktoken.transfer_from(self.env().caller(), self.env().account_id(), amount);

            // emit Stake event
            self.ilocktoken.emit_stake(
                staker,
                hash,
                amount,
                );

            true
        }

        #[ink(message)]
        pub fn claim_stake(&mut self, staker: AccountId, hash: Hash, amount: u32) -> bool {

            self.stakedata.update_stake(staker, hash, amount);
            self.stakedata.update_hash(hash, staker, amount);
            self.ilocktoken.transfer_from(self.env().caller(), self.env().account_id(), amount);

            // emit Stake event
            self.ilocktoken.emit_stake(
                staker,
                hash,
                amount,
                );

            true
        }
    }
}
