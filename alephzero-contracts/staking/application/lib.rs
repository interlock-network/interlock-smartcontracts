#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod application {

    use ilockmvp::ILOCKmvpRef;
    use ilockmvp::ilockmvp::OtherError;
    use ink_prelude::{
        string::String,
        vec::Vec,
    };

    // this is the application port (the contract hash and owner, cap, tax, etc)
    pub const PORT: u16 = 0;

    #[ink(storage)]
    pub struct Application {

        token_instance: ILOCKmvpRef,
        operator: AccountId,
    }

    impl Application {

        #[ink(constructor)]
        pub fn new_application(
            token_address: AccountId,
        ) -> Self {
            
            let token_instance: ILOCKmvpRef = ink_env::call::FromAccountId::from_account_id(token_address);
            let operator: AccountId = Self::env().caller();

            Self { token_instance, operator }
        }

        /// . register this application contract with the token contract
        /// . only operator may call
        #[ink(message)]
        pub fn register(&mut self) -> Result<(), OtherError> {

            

            self.token_instance.create_socket(self.env().caller(), PORT)
        }

        #[ink(message)]
        pub fn call_socket(
            &mut self,
            address: AccountId,
            amount: Balance,
            data: Vec<u8>,
        ) -> Result<(), OtherError> {

            // do stuff here, then reward user

            self.token_instance.call_socket(address, amount, data)
        }

        #[ink(message)]
        pub fn pool_balance(
            &self,
            pool: u8,
        ) -> (String, Balance) {

            self.token_instance.pool_balance(pool)
        }
    }
}
