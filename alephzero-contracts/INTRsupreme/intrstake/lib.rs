// INTERLOCK NETWORK
//
// blairmunroakusa@0742Tue.28Jun22.anch.AK:south
//
// THIS IS A PROTOTYPE STAKING CONTRACT
// USING INK! FRAMEWORK


#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::intrstake::{
    INTRstake,
    INTRstakeRef,
};

use ink_lang as ink;

#[ink::contract]
pub mod intrstake {

    use intrtoken::INTRtokenRef;
    use stakedata::StakeDataRef;

    use ink_lang::utils::initialize_contract;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;

    /// defines contract storage


    #[ink(storage)]
    pub struct INTRstake {
        intrtoken: INTRtokenRef,
        stakedata: StakeDataRef,
    }

    impl INTRstake {
        
        /// Constructor that initializes staking contract
        #[ink(constructor)]
        pub fn new(
            intrtoken: INTRtokenRef,
            stakedata: StakeDataRef
        ) -> Self {

            Self {
                intrtoken,
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
            self.intrtoken.transfer_from(self.env().caller(), self.env().account_id(), amount);

            // emit Stake event
            self.intrtoken.emit_stake(
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
            self.intrtoken.transfer_from(self.env().caller(), self.env().account_id(), amount);

            // emit Stake event
            self.intrtoken.emit_stake(
                staker,
                hash,
                amount,
                );

            true
        }
    }
}
