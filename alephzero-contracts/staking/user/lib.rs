#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ilocktoken;

#[ink::contract]
pub mod user {

    use ilocktoken::ILOCKtokenRef;


    #[ink(storage)]
    pub struct User {

        token_instance: ILOCKtokenRef,
    }

    impl User {

        #[ink(constructor)]
        pub fn new(
            token_address: AccountId,
        ) -> Self {
            
            let mut token_instance: ILOCKtokenRef = ink_env::call::FromAccountId::from_account_id(token_address);

            token_instance.register_user_contract(Self::env().caller());

            Self {
                token_instance,
            }
        }

        #[ink(message)]
        pub fn user_do_something(&self) -> Balance {
            self.token_instance.cap()
        }
    }
}

