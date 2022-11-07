#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod port {

    use ilocktoken::ILOCKtokenRef;
    use ilocktoken::ilocktoken::OtherError;

    pub const PORT: u16 = 0;

    #[ink(storage)]
    pub struct Port {

        token_instance: ILOCKtokenRef,
    }

    impl Port {

        #[ink(constructor)]
        pub fn new(
            token_address: AccountId,
        ) -> Self {
            
            let token_instance: ILOCKtokenRef = ink_env::call::FromAccountId::from_account_id(token_address);

            Self { token_instance }
        }

        #[ink(message)]
        pub fn register(&mut self) -> Result<(), OtherError> {

            self.token_instance.create_socket(self.env().caller(), PORT)
        }

        #[ink(message)]
        pub fn call_socket(
            &mut self,
            address: AccountId,
            amount: Balance
        ) -> Result<(), OtherError> {

            // do stuff here, then reward user

            self.token_instance.call_socket(address, amount)
        }
    }
}


