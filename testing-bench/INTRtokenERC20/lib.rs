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


    use ink_lang::utils::initialize_contract;
    use ink_prelude::string::String;
    use ink_prelude::string::ToString;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct IntRtokenErc20 {
        /// Stores a single `bool` value on the storage.
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: u32,
        balances: Mapping<AccountId, u32>,
        allowances: Mapping<(AccountId, AccountId), u32>,
    }


    /// Specify ERC-20 error type.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Return if the balance cannot fulfill a request.
        InsufficientBalance,
    }

    /// Specify the ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    /// transfer event
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        amount: u32,
    }

    /// approve event
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        amount: u32,
    }


    impl IntRtokenErc20 {


        /// Constructor that initializes the `bool` value to the given `init_value`
        #[ink(constructor)]
        pub fn new_token(supply: u32) -> Self {
            initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.balances.insert(&caller, &supply);
                contract.total_supply = supply;
                Self::env().emit_event(Transfer {
                    from: None,
                    to: Some(caller),
                    amount: supply,
                });
                contract.name = "test".to_string();
                contract.symbol = "INTR".to_string();
                contract.decimals = 18;
            })
        }

        /// token decimal count getter
        #[ink(message)]
        pub fn name(&self) -> String {
            self.name.clone()
        }

        /// token decimal count getter
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.symbol.clone()
        }

        /// token decimal count getter
        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            self.decimals
        }

        /// total supply getter
        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            self.total_supply
        }

        /// account balance getter
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> u32 {
            match self.balances.get(&account) {
                Some(value) => value,
                None => 0,
            }
        }

        /// account allowance getter
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> u32 {
            match self.allowances.get((&owner, &spender)) {
                Some(value) => value,
                None => 0,
            }
        }
        
        /// transfer method
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

        /// approve method
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, amount: u32) -> bool {
            let owner = self.env().caller();
            // here, make sure owner and spender addresses are not 0 address
            self.allowances.insert((&owner, &spender), &amount);
            self.env().emit_event(Approval {
                owner,
                spender,
                amount,
            });
            true
        }

        /// transfer from method
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: u32,
        ) -> Result<()> {
            let from_balance = self.balance_of(from);
            if from_balance < amount {
                return Err(Error::InsufficientBalance)
            }
            self.balances.insert(from, &(from_balance - amount));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + amount));
            Ok(())
        }

    }

/////////////////////////////////////////////////////////////////////////


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
