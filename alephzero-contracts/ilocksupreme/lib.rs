// INTERLOCK NETWORK
//
// blairmunroakusa@0653Tue.28Jun22.anch.AK:br
//
// THIS IS A TEST SUPREME NODE CONTRACT TO
// INTEGRATE TOKEN AND STAKING CONTRACTS
// USING INK! FRAMEWORK



// !!!!! INCOMPLETE AND FLAWED, WARNING !!!!!

// NOTES: Because I cannot find a way to instantiate a collection of contracts
// like below, then only call methods from this or that contract without accessing
// different storage, this contract represents the access point to all methods
// living within child contracts. Pretty straight forward.


#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ilocksupreme {

    use ilocktoken::ILOCKtokenRef;
    use ilockstake::ILOCKstakeRef;
    use stakedata::StakeDataRef;
    
    use ink_storage::traits::{
        PackedLayout,
        SpreadLayout,
    };
    use ink_prelude::string::String;

    #[ink(storage)]
    pub struct ILOCKsupreme {
        ilocktoken: ILOCKtokenRef,
        ilockstake: ILOCKstakeRef,
        stakedata: StakeDataRef,
    }

    impl ILOCKsupreme {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new_supreme(
            init_value: u32,
            version: u32,
            token_code_hash: Hash,
            stake_code_hash: Hash,
            stakedata_code_hash: Hash,
        ) -> Self {
            let total_balance = Self::env().balance();
            let salt = version.to_le_bytes();
            let ilocktoken = ILOCKtokenRef::new_token(init_value)
                .endowment(total_balance/4)
                .code_hash(token_code_hash)
                .salt_bytes(salt)
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!(
                        "Failed to instantiate token contract: {:?}", error)
                });
            let stakedata = StakeDataRef::new()
                .endowment(total_balance/4)
                .code_hash(stakedata_code_hash)
                .salt_bytes(salt)
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!(
                        "Failed to instantiate stakedata contract: {:?}", error)
                });
            let ilockstake = ILOCKstakeRef::new(ilocktoken.clone(), stakedata.clone())
                .endowment(total_balance/4)
                .code_hash(stake_code_hash)
                .salt_bytes(salt)
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!(
                        "Failed to instantiate stake contract: {:?}", error)
                });


            Self {
                ilocktoken,
                ilockstake,
                stakedata,
            }
        }


// ERC20 methods
        
        /// name getter
        #[ink(message)]
        pub fn name(&self) -> String {
            self.ilocktoken.name()
        }

        /// symbol getter
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.ilocktoken.symbol()
        }

        /// decimals getter
        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            self.ilocktoken.decimals()
        }

        /// total supply getter
        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            self.ilocktoken.total_supply()
        }

        /// account balance getter
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> u32 {
            self.ilocktoken.balance_of(account)
        }

        /// account allowance getter
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> u32 {
            self.ilocktoken.allowance(owner, spender)
        }

        /// transfer token
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: u32) -> bool {
            self.ilocktoken.transfer(to, amount)
        }

        /// transfer token from, to
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: u32) -> bool {
            self.ilocktoken.transfer_from(from, to, amount)
        }

        /// approve token spending
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, amount: u32) -> bool {
            self.ilocktoken.approve(spender, amount)
        }

    }


}
