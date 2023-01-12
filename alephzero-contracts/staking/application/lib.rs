//
// INTERLOCK NETWORK - GENERIC STAKING AND REWARDS APPLICATION CONTRACT
//

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod application {

    use ilockmvp::ILOCKmvpRef;
    use ilockmvp::ilockmvp::OtherError;
    use ink_prelude::vec::Vec;

    // this is the number designating application type's
    // port (the contract hash, owner, cap, tax, paid, collected, etc)
    pub const PORT: u16 = 0;

    #[ink(storage)]
    pub struct Application {

        token_instance: ILOCKmvpRef,
        operator: AccountId,
    }

    impl Application {

        /// . create new staking application contract linked to token contract
        #[ink(constructor)]
        pub fn new_application(
            token_address: AccountId,
        ) -> Self {
            
            // create a reference to the deployed token contract
            let token_instance: ILOCKmvpRef = ink_env::call::FromAccountId::from_account_id(token_address);
            let operator: AccountId = Self::env().caller();

            Self { token_instance, operator }
        }

        /// . register this application contract with the token contract
        /// . only operator may call
        #[ink(message)]
        pub fn create_socket(
            &mut self
        ) -> Result<(), OtherError> {

            // make sure caller is operator
            if self.env().caller() != self.operator {

                return Err(OtherError::CallerNotOperator);
            }

            self.token_instance.create_socket(self.env().caller(), PORT)
        }

        /// . make call to registed rewards socket
        /// . only operator may call
        #[ink(message)]
        pub fn call_socket(
            &mut self,
            address: AccountId,
            amount: Balance,
            data: Vec<u8>,                  // <--! data vector to pass custom information to token
            ) -> Result<(), OtherError> {   //      contract logic

            // make sure caller is operator
            if self.env().caller() != self.operator {

                return Err(OtherError::CallerNotOperator);
            }

            // < do stuff here, then reward user >

            self.token_instance.call_socket(address, amount, data)
        }

        /// . change application operator
        /// . only operator may call
        #[ink(message)]
        pub fn change_operator(
            &mut self,
            newoperator: AccountId,
        ) -> Result<(), OtherError> {

            // make sure caller is operator
            if self.env().caller() != self.operator {

                return Err(OtherError::CallerNotOperator);
            }

            self.operator = newoperator;

            Ok(())
        }
    }
}
