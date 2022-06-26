#![cfg_attr(not(feature = "std"), no_std)]


use ink_lang as ink;

#[ink::contract]
mod INTRsupreme {
    use INTRtokenERC20::INTRTOKENERC20Ref;
    use ink_storage::traits::{
        PackedLayout,
        SpreadLayout,
    };
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
        INTRTOKENERC20,
    }

    #[ink(storage)]
    pub struct INTRsupreme {
        which: Which,
        INTRtokenERC20: INTRTOKENERC20Ref,
    }

    #[ink(storage)]
    pub struct INTRSUPREME {
        /// store global values here
        init_value u32,
        token_code_hash: Hash,
    }

    impl INTRSUPREME {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new_supreme() -> Self {
            let total_balance = Self::env().balance();
            let salt = version.to_le_bytes();
            let token = INTRTOKENERC20Ref::new_token(init_value)
                .endowment(total_balance/2)
                .code_hash(token_code_hash)
                .salt_bytes(salt)
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!(
                        "Failed to instantiate token contract: {:?}" error)
                });
            Self { init_value  }
        }



    }


}
