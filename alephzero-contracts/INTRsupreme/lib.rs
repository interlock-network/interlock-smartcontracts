#![cfg_attr(not(feature = "std"), no_std)]


use ink_lang as ink;

#[ink::contract]
mod intrsupreme {
    use intrtoken::INTRtokenRef;
    use ink_storage::traits::{
        PackedLayout,
        SpreadLayout,
    };

    #[ink(storage)]
    pub struct INTRsupreme {
        intrtoken: INTRtokenRef,
    }

    #[derive(
        Debug,
        Copy,
        Clone,
        PartialEq,
        Eq,
        scale::Encode,
        scale::Decode,
        SpreadLayout,
        PackedLayout,
    )]
    #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub enum Which {
        Adder,
        Subber,
    }


    impl INTRsupreme {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new_supreme(
            init_value: u32,
            version: u32,
            token_code_hash: Hash,
        ) -> Self {
            let total_balance = Self::env().balance();
            let salt = version.to_le_bytes();
            let intrtoken = INTRtokenRef::new_token(init_value)
                .endowment(total_balance/2)
                .code_hash(token_code_hash)
                .salt_bytes(salt)
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!(
                        "Failed to instantiate token contract: {:?}", error)
                });
            Self {
                intrtoken,
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            self.intrtoken.total_supply()
        }



    }


}
