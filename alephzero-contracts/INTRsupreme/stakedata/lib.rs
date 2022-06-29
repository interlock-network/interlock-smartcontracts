// INTERLOCK NETWORK
//
// blairmunroakusa@0742Tue.28Jun22.anch.AK:south
//
// THIS IS A PROTOTYPE STAKING CONTRACT
// USING INK! FRAMEWORK



// !!!!! INCOMPLETE AND FLAWED, WARNING !!!!!




#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::stakedata::{
    StakeData,
    StakeDataRef,
};

use ink_lang as ink;

#[ink::contract]
pub mod stakedata {

    use ink_lang::utils::initialize_contract;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;

    /// defines contract storage

    #[derive(SpreadAllocate)]
    #[ink(storage)]
    pub struct StakeData {
        rewards_available: Mapping<AccountId, u32>,
        url_hashes: Mapping<(Hash, AccountId), u32>,
        url_stakes: Mapping<(AccountId, Hash), u32>,
    }
    
    impl StakeData {
        
        /// Constructor that initializes staking contract
        #[ink(constructor)]
        pub fn new() -> Self {
            // create contract
            initialize_contract(|contract: &mut Self| {

                // define owner as caller
                let caller = Self::env().caller();

                contract.rewards_available.insert(&caller, &0);
            })
        }

        /// stake getter
        #[ink(message)]
        pub fn get_stake(&self, staker: AccountId, hash: Hash) -> u32 {
            self.url_stakes.get(&staker, &hash)
        }

        /// add or change hash account amount info
        #[ink(message)]
        pub fn update_hash(&mut self, hash: Hash, account: AccountId, amount: u32) -> bool {
            self.url_hashes.insert((&hash, &account), &amount);
            true
        }

        /// add or change account stake info
        #[ink(message)]
        pub fn update_stake(&mut self, account: AccountId, hash: Hash, amount: u32) -> bool {
            self.url_stakes.insert((&account, &hash), &amount);
            true
        }

        /// add to account's available rewards
        #[ink(message)]
        pub fn reward(&mut self, account: AccountId, amount: u32) -> bool {
            let prior = self.rewards_available.get(&account).unwrap_or(0);
            self.rewards_available.insert(&account, &(&prior + &amount));
            true
        }

        /// claim account's available rewards
        #[ink(message)]
        pub fn claim(&mut self, account: AccountId, amount: u32) -> bool {
            self.rewards_available.insert(&account, &0);
            true
        }
    }
}

