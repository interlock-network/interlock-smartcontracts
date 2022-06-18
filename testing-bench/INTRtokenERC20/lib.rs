// INTERLOCK NETWORK
//
// blairmunroakusa@1019Fri.17Jun22.anch.AK:br
//
// THIS IS A TEST ERC20 CONTRACT TO DETERMINE HOW
// EASILY ONE CAN SPIN UP A TOKEN ON ALEPH ZERO
// USING INK! FRAMEWORK


#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;


#[ink::contract]
mod INTRtokenERC20 {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.

    use ink_storage::traits::SpreadAllocate;
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct IntRtokenErc20 {
        /// Stores a single `bool` value on the storage.
        total_supply: u32,
        balances: Mapping<AccountId, u32>,
    }

    use ink_lang::utils::initialize_contract;
    use ink_storage::Mapping;
    impl IntRtokenErc20 {

        /// Constructor that initializes the `bool` value to the given `init_value`.

        #[ink(constructor)]
        pub fn new_token(supply: u32) -> Self {
            initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.balances.insert(&caller, &supply);
                contract.total_supply = supply;
            })
        }

        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> u32 {
            match self.balances.get(&account) {
                Some(value) => value,
                None => 0,
            }
        }

        #[ink(message)]
        pub fn transfer(&mut self, recipient: AccountId, amount: u32) -> bool {
            let sender = self.env().caller();
            let sender_balance = self.balance_of(sender);
            if sender_balance < amount {
                ink_env::debug_println!("Insufficient balance");
                return false
            }
            let recipient_balance = self.balance_of(recipient);
            self.balances.insert(sender, &(&sender_balance - amount));
            self.balances.insert(recipient, &(&recipient_balance + amount));
            true
        }

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let INTRtokenERC20 = IntRtokenErc20::default();
            assert_eq!(INTRtokenERC20.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut INTRtokenERC20 = IntRtokenErc20::new(false);
            assert_eq!(INTRtokenERC20.get(), false);
            INTRtokenERC20.flip();
            assert_eq!(INTRtokenERC20.get(), true);
        }
    }
}
