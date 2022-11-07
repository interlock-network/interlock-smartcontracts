
// MALICIOUS USER


#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ilocktoken;
use ink_env::Error;

#[ink::contract]
pub mod user {

    use ilocktoken::ILOCKtokenRef;


    #[ink(storage)]
    pub struct User {

        token_instance: ILOCKtokenRef,
        token_dummy: ILOCKtokenRef,
    }

    impl User {

        #[ink(constructor)]
        pub fn new(
            token_address: AccountId,
            dummy_address: AccountId,
        ) -> Self {
            
            let mut token_instance: ILOCKtokenRef = ink_env::call::FromAccountId::from_account_id(token_address);
            let token_dummy: ILOCKtokenRef = ink_env::call::FromAccountId::from_account_id(dummy_address);

            match token_instance.register_user_contract(Self::env().caller()) {
                Some(()) => Self { token_instance: token_instance, token_dummy: token_dummy },
                None => Self { token_instance: token_dummy, token_dummy: token_instance },
            }
        }

        #[ink(message)]
        pub fn token(&self) -> ILOCKtokenRef {

            self.token_instance.clone()
        }
        #[ink(message)]
        pub fn token_instance(&self) -> ILOCKtokenRef {

            self.token_instance.clone()
        }

        #[ink(message)]
        pub fn token_dummy(&self) -> ILOCKtokenRef {

            self.token_dummy.clone()
        }
            // 
            // token_instance and token_dummy work as follows:
            // . if an honest user creates an account (via our safe codehash, for example)
            // then this user will pass the contract codehash comparison enforced by the
            // token contract.
            // ( this comparison is between the calling contract code hash and the known-friendly
            //   contract code hash stored in the token contract. )
            // . if this user is malicious (as in, they have created a mock user contract)
            // then they will not pass the code hash test, and the intantiated contract
            // will be the bogus dummy contact.
            //
            // With the dummy contract, the malicious user may be as malicious as they want.
            //
        

        #[ink(message)]
        pub fn user_do_something(&self) -> Balance {

            let var: u8 = 1;

            self.token_instance.cap()
        }
    }
}

