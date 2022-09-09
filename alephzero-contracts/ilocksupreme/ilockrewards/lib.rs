// INTERLOCK NETWORK - MVP rewards contract
//
// This is simple contract that links the MVP browser extension
// with the ERC20 ILOCK token contract. This contract is responsible
// for doling out rewards as users browse the internet.
//
// blairmunroakusa@1719Thu.08Sep22.anch.AK:south

// !!!!! INCOMPLETE AND FLAWED, WARNING !!!!!

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

//pub use crate::ilockrewards::ILOCKrewards;
use ink_lang as ink;

#[ink::contract]
mod ilockrewards {

    use ilocktoken::ilocktoken::ILOCKtokenRef;
    use ink_env::call::FromAccountId;

    #[ink(storage)]
    pub struct ILOCKrewards {
        ilocktoken_contract: ILOCKtokenRef,
        rewardedTotal: u128,
        rewardedPastMonth: u128,
        rewardedPastWeek: u128,
        rewardedPastDay: u128,
    }

    impl ILOCKrewards {

        /// create rewards contract and link to ilocktoken contract
        #[ink(constructor)]
        pub fn new_ILOCKrewards(
            ilocktoken_address: AccountId,
        ) -> Self {
            
            let ilocktoken_contract: ILOCKtokenRef = FromAccountId::from_account_id(ilocktoken_address);

            Self {
                ilocktoken_contract,
                rewardedTotal: 0,
                rewardedPastMonth: 0,
                rewardedPastWeek: 0,
                rewardedPastDay: 0,
            }
        }

        #[ink(message)]
        pub fn dummyfunction(&self) -> bool {
            true
        }
    }
}
