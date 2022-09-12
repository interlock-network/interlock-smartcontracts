// INTERLOCK NETWORK
//
// blairmunroakusa@1531Fri.09Sep22.anch.AK:south

// !!!!! INCOMPLETE AND FLAWED, WARNING !!!!!
//
// NOTE: This contract exists because I cannot figure out how to
// create a delegator contract that also contains state. (The 'initialize_contract'
// method is not defined for any type ```ThisContractRef```.) So creating a struct
// with both Ref types and regular types makes it (possibly) impossible to initialize
// delegator contract.

#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::ilockrewardsdata::{
    ILOCKrewardsData,
    ILOCKrewardsDataRef,
};

use ink_lang as ink;

#[ink::contract]
pub mod ilockrewardsdata {

    use ink_lang::utils::initialize_contract;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;

    /// defines contract storage
    #[derive(SpreadAllocate)]
    #[ink(storage)]
    pub struct ILOCKrewardsData {
        rewardedTotal: Balance,
        rewardedUser: Mapping<AccountId, Balance>,
        rewardFactor: u32,
    }

    impl ILOCKrewardsData {

        /// constructor that initializes contract
        #[ink(constructor)]
        pub fn new_ilockrewardsdata() -> Self {

            // create contract
            initialize_contract(|contract: &mut Self| {

                // define owner as caller
                let caller = Self::env().caller();

                // initialize
                contract.rewardedTotal = 0;
                contract.rewardedUser.insert(&caller, &0);
                contract.rewardFactor = 0; // << This determines what the reward will be

            })
        }

        /// get rewarded total
        #[ink(message)]
        pub fn rewardedTotal(&self) -> Balance {

            self.rewardedTotal
        }

        /// set rewarded total
        #[ink(message)]
        pub fn mut_rewardedTotal(&mut self, total: Balance) -> bool {

            self.rewardedTotal = total;

            true
        }

        /// get user rewards
        #[ink(message)]
        pub fn rewardedUser(&self, user: AccountId) -> Balance {

            match self.rewardedUser.get(&user) {
                Some(value) => value,
                None => 0,
            }
        }

        /// set user rewards
        #[ink(message)]
        pub fn mut_rewardedUser(&mut self, user: AccountId, reward: Balance) -> bool {

            // update total rewarded to user
            self.rewardedUser.insert(&user, &reward);

            // update total rewarded
            self.mut_rewardedTotal(reward + self.rewardedTotal());

            true
        }

        /// get reward factor
        #[ink(message)]
        pub fn rewardFactor(&self) -> u32 {

            self.rewardFactor
        }

        /// set reward factor
        #[ink(message)]
        pub fn mut_rewardFactor(&mut self, factor: u32) -> bool {

            self.rewardFactor = factor;
            
            true
        }
        
    }
}
