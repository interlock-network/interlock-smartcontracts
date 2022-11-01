#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod user {

    use ilocktoken::ILOCKtokenRef;

    pub const PORT: u16 = 0;

    #[ink(storage)]
    pub struct User {

        token_instance: ILOCKtokenRef,
    }

    impl User {

        #[ink(constructor)]
        pub fn new(
            token_address: AccountId,
        ) -> Self {
            
            let token_instance: ILOCKtokenRef = ink_env::call::FromAccountId::from_account_id(token_address);

            Self { token_instance }
        }

        #[ink(message)]
        pub fn register(&mut self) -> Option<()>  {

            self.token_instance.create_socket(self.env().caller(), PORT)
        }

        #[ink(message)]
        pub fn user_do_something(&self) -> Balance {

            self.token_instance.cap()
        }
    }
}


