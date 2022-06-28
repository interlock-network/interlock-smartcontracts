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
mod intrstake {

    use intrtoken::INTRtokenRef;
    use stakedata::StakeDataRef;

    use ink_lang::utils::initialize_contract;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;

    /// defines contract storage

    #[ink(storage)]
    pub struct stake_data {
        rewards_available: Mapping<AccountId, u32>,
        url_hashes: Mapping<(Hash, AccountId), u32>,
        url_stakes: Mapping<(AccountId, Hash), u32>,
    }

    #[ink(storage)]
    pub struct INTRstake {
        intrtoken: INTRtokenRef,
        stakedata: StakeDataRef,
    }

    /// specify stake event
    #[ink(event)]
    pub struct Stake {
        #[ink(topic)]
        staker: Option<AccountId>,
        #[ink(topic)]
        hash: Option<Hash>,
        amount: u32,
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
        pub fn stake_url(&mut self, staker: AccountId, hash: Hash, amount: u32) -> bool {
/*
            // add account's stake to hash's stake record
            self.url_hashes.insert((&hash, &staker), &amount);

            // add hash to account's stake record
            self.url_stakes.insert((&staker, &hash), &amount);

            // add zero award balance for account id
           /* self.rewards_available.insert(
                &staker,
                (self.rewards_available.get(&staker) {
                    Some(value) => value,
                    None => 0,
                } + amount)
            );*/


            // emit Stake event
            Self::env().emit_event(Stake {
                staker: Some(staker),
                hash: Some(hash),
                amount: amount,
            });
*/
            true
        }
    }
}
