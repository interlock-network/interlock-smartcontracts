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
        pub fn total_rewarded(&self) -> u128 {
            self.contract_ilockrewardsdata.rewardedTotal()
        }

        /// get amount rewarded to user to date
        #[ink(message)]
        pub fn total_rewarded_user(&self, user: AccountId) -> u128 {
            self.contract_ilockrewardsdata.rewardedUser(user)
        }

        /// reward the user for browsing
        #[ink(message)]
        pub fn reward_user(&mut self, reward: Balance, user: AccountId) -> (u128, u128) {

            // get total amount rewarded overall and to user so far
            let mut totalUserRewarded: Balance = self.contract_ilockrewardsdata.rewardedUser(user);

            // update total amount rewarded to user
            totalUserRewarded += reward;

            // update token circulation
            self.contract_ilocktoken.increment_circulation(reward);

            // get rewards pool address
            let rewards_pool = self.contract_ilocktoken.rewards_pool();

            // transfer reward tokens from rewards pool to user
            self.contract_ilocktoken.transfer_from(rewards_pool, user, reward).unwrap();

            // update state for user and total amounts rewarded
            self.contract_ilockrewardsdata.mut_rewardedUser(user, reward);
            self.contract_ilockrewardsdata.mut_rewardedTotal(reward);

            // this returns user total and reward amount for extension display purposes
            (totalUserRewarded, reward)
        }
    }
}
