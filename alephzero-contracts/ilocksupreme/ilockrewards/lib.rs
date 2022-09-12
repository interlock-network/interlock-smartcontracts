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

use ink_lang as ink;

#[ink::contract]
mod ilockrewards {

    use ilocktoken::ILOCKtokenRef;
    use ilockrewardsdata::ILOCKrewardsDataRef;
    use ink_env::call::FromAccountId;

    #[ink(storage)]
    pub struct ILOCKrewards {
        contract_ilocktoken: ILOCKtokenRef,
        contract_ilockrewardsdata: ILOCKrewardsDataRef,

    }

    impl ILOCKrewards {

        /// create rewards contract and link to ilocktoken contract
        #[ink(constructor)]
        pub fn new_ILOCKrewards(
            address_ilocktoken: AccountId,
            address_ilockrewardsdata: AccountId,
        ) -> Self {

            // get contract handles
            let contract_ilocktoken: ILOCKtokenRef =
                FromAccountId::from_account_id(address_ilocktoken);

            let contract_ilockrewardsdata: ILOCKrewardsDataRef =
                FromAccountId::from_account_id(address_ilockrewardsdata);

            Self {
                contract_ilocktoken,
                contract_ilockrewardsdata,
            }
        }

        /// get total amount rewarded to date
        #[ink(message)]
        pub fn totalRewarded(&self) -> Balance {
            self.contract_ilockrewardsdata.rewardedTotal()
        }

        /// get amount rewarded to user to date
        #[ink(message)]
        pub fn totalRewardedUser(&self, user: AccountId) -> Balance {
            self.contract_ilockrewardsdata.rewardedUser(user)
        }

        /// reward the user for browsing
        #[ink(message)]
        pub fn rewardUser(&mut self, reward: Balance, user: AccountId) -> (Balance, Balance) {

            // get total amount rewarded to user so far
            let mut totalRewarded: Balance = self.contract_ilockrewardsdata.rewardedUser(user);

            // update total amount rewarded to user
            totalRewarded += reward;

            // update state for user and total amounts rewarded
            self.contract_ilockrewardsdata.mut_rewardedUser(user, reward);

            // this returns user total and reward amount for extension display purposes
            (totalRewarded, reward)
        }

    }
}
