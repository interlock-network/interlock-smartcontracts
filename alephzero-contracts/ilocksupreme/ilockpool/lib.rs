// INTERLOCK NETWORK
//
// blairmunroakusa@0936Mon.12Sep22.anch.AK:rc
//
// !!!!! INCOMPLETE AND FLAWED, WARNING !!!!!
//
// NOTE: This contract exists to form token pools.
// Each pool contract must be contructed individually (and
// not from the token contract). This is for the same reason that
// ilockrewards delegator must use ilockrewardsdata to store state.

// NOTE: To enable unsigned integer division, overflow_checks
// has been turned 'off' in Cargo.toml file.

#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::ilockpool::{
    ILOCKpool,
    ILOCKpoolRef,
};

use ink_lang as ink;

#[ink::contract]
pub mod ilockpool {

    use ink_lang::utils::initialize_contract;
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::string::String;

    #[derive(SpreadAllocate)]
    #[ink(storage)]
    pub struct ILOCKpool {
        poolNumber: u8,
        poolName: String,
    }

    impl ILOCKpool {

        /// constructor that initializes contract
        #[ink(constructor)]
        pub fn new_ilockpool(number: u8, name: String) -> Self {

            // create contract
            initialize_contract(|contract: &mut Self| {

                contract.poolNumber = number;
                contract.poolName = name;
            })
        } 

        /// get basic pool information (name and number)
        #[ink(message)]
        pub fn poolinfo(&self) -> (u8, String) {
            (self.poolNumber, self.poolName.clone())
        }
    }
}
