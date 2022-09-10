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

    use ilocktoken::ILOCKtokenRef;
    use ilockrewardsdata::ILOCKrewardsDataRef;
    use ink_env::call::FromAccountId;

    #[ink(storage)]
    pub struct ILOCKrewards {
        ilocktoken_contract: ILOCKtokenRef,
        ilockrewardsdata_contract: ILOCKrewardsDataRef,

    }

    impl ILOCKrewards {

        /// create rewards contract and link to ilocktoken contract
        #[ink(constructor)]
        pub fn new_ILOCKrewards(
            ilocktoken_address: AccountId,
            ilockrewardsdata_address: AccountId,
        ) -> Self {

            let ilocktoken_contract: ILOCKtokenRef = FromAccountId::from_account_id(ilocktoken_address);
            let ilockrewardsdata_contract: ILOCKrewardsDataRef = FromAccountId::from_account_id(ilockrewardsdata_address);

            Self {
                ilocktoken_contract,
                ilockrewardsdata_contract,
            }
        }

        #[ink(message)]
        pub fn dummyfunction(&self) -> bool {
            ink_env::debug_println!("{:?}", self.ilockrewardsdata_contract.rewardFactor());
            true
        }
    }
}
