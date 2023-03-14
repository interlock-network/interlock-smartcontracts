//!
//! # INTERLOCK NETWORK - GENERIC PORT/SOCKET APPLICATION TEMPLATE CONTRACT
//!
//!
//! #### To ensure build with cargo-contract version 2.0.0, run:
//!
//! -     cargo install cargo-contract --force --version 2.0.0
//!
//! #### To build, run:
//!
//! -     cargo +nightly contract build
//!
//! #### To build docs, run:
//!
//! -     cargo +nightly doc --no-deps --document-private-items --open
//!
//! #### To reroute docs in Github, run:
//!
//! -     echo "<meta http-equiv=\"refresh\" content=\"0; url=application\">" >
//! -     target/doc/index.html;
//! -     cp -r target/doc ./docs
//!

#![doc(
    html_logo_url = "https://uploads-ssl.webflow.com/6293b370c2da3eda80121e92/6293d7cffa42ae33001294d1_interlock-visual-hero.png",
    html_favicon_url = "https://uploads-ssl.webflow.com/6293b370c2da3eda80121e92/6293d7cffa42ae33001294d1_interlock-visual-hero.png",
)]

#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
pub mod application {

    use ilockmvp::ILOCKmvpRef;
    use ilockmvp::ilockmvp::OtherError;
    use ink::prelude::vec::Vec;

    // this is the number designating application type's
    // port (the contract hash, owner, cap, tax, paid, collected, etc)
    pub const PORT: u16 = 0;

    #[ink(storage)]
    pub struct Application {

        token_instance: ILOCKmvpRef,
        operator: AccountId,
    }

    impl Application {

        /// - Create new staking application contract linked to token contract.
        #[ink(constructor)]
        pub fn new_application(
            token_address: AccountId,
        ) -> Self {
            
            // create a reference to the deployed token contract
            let token_instance: ILOCKmvpRef = ink::env::call::FromAccountId::from_account_id(token_address);
            let operator: AccountId = Self::env().caller();

            Self { token_instance, operator }
        }

        /// - Register this application contract with the ILOCK PSP22 token contract.
        /// - Only operator may call.
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

        /// - Make call to registed ILOCK PSP22 token contract socket.
        /// - Only operator may call.
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

        /// - Change application operator.
        /// - Only operator may call.
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
