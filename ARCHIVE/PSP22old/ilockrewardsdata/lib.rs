// INTERLOCK NETWORK - DATA FOR MVP REWARDS CONTRACT
//
// !!!!! INCOMPLETE AND UNAUDITED, WARNING !!!!!
/
// NOTE: This contract exists because I cannot figure out how to
// create a delegator contract that also contains Mapping state. (The 'initialize_contract'
// method is not defined for any type ```ThisContractRef```.) So creating a struct
// with both Ref types and regular types makes it (possibly) impossible to initialize
// delegator contract.
// ...also, event type implementations in delegator conflict with event implementations
// in PSP22 token contract, so implementing the 'Reward' event here and exposing it
// to the delegator is the only way I can find to emit Reward event from reward_user method
// in delegator.

#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::ilockrewardsdata::{
    ILOCKrewardsData,
    ILOCKrewardsDataRef,
};

use ink_lang as ink;

#[ink::contract]
pub mod ilockrewardsdata {

    use ink_lang::utils::initialize_contract;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;

    /// defines contract storage
    #[derive(SpreadAllocate)]
    #[ink(storage)]
    pub struct ILOCKrewardsData {
        rewardedtotal: u128,
        rewardeduser: Mapping<AccountId, u128>,
        owner: AccountId,
    }

    /// . specify reward event
    #[ink(event)]
    pub struct Reward {
        #[ink(topic)]
        to: Option<AccountId>,
        amount: Balance,
    }

    /// . error types
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum RewardsdataError {
        /// Returned if caller is not contract owner
        CallerNotOwner,
        /// Returned if specified reward recipient is zero address
        ZeroRecipientAddress,
        /// Returned if specified reward is too larger
        RewardTooLarge,
    }

    /// . RewardsError result type.
    pub type ResultRewardsdata<T> = core::result::Result<T, RewardsdataError>;

    impl ILOCKrewardsData {

        /// constructor that initializes contract
        #[ink(constructor)]
        pub fn new_ilockrewardsdata() -> Self {

            // create contract
            initialize_contract(|contract: &mut Self| {

                // define owner as caller
                let caller = Self::env().caller();

                // initialize
                contract.rewardedtotal = 0;
                contract.rewardeduser.insert(&caller, &0);
                contract.owner = caller;

            })
        }

        // IF env().caller() IN CASE OF CROSS CONTRACT CALL IS CALLING CONTRACT
        // ADDRESS AND NOT ORIGINATING CALLER (FROM ILOCKREWARDS CONTRACT) THEN WE NEED TO
        // CREATE A SETTER METHOD HERE TO SET OWNER AS ILOCKREWARDS CONTRACT ADDRESS.
        //
        // IF .env().caller IS ILOCKREWARDS CONTRACT ADDRESS, WE ONLY NEED NOT_OWNER GUARD, PLUS
        // VARIABLE AND SETTER METHOD TO TAKE CONTRACT ACCOUNTID. IF NOT, WE NEED TO IMPLEMENT ALL
        // GUARDS AGAIN (NOT_OWNER, IS_ZERO, MAXREWARD).

        /// check owner modifier
        pub fn not_owner(
            &self
        ) -> bool {

            self.env().caller() != self.owner
        }

        /// get rewarded total
        #[ink(message)]
        pub fn rewarded_total(&self) -> Balance {

            self.rewardedtotal
        }

        /// set rewarded total
        #[ink(message)]
        pub fn mut_rewarded_total(&mut self, reward: Balance) -> bool {

            // make sure caller is owner
            if self.not_owner() {
                return false
            }

            // increment reward total
            self.rewardedtotal += reward;

            true
        }

        /// get user rewards
        #[ink(message)]
        pub fn rewarded_user(&self, user: AccountId) -> Balance {

            match self.rewardeduser.get(&user) {
                Some(value) => value,
                None => 0,
            }
        }

        /// set user rewards
        #[ink(message)]
        pub fn mut_rewarded_user(&mut self, user: AccountId, reward: Balance) -> bool {

            // make sure caller is owner
            if self.not_owner() {
                return false
            }

            // get prior total reward
            let prior_total: Balance = self.rewardeduser.get(user).unwrap();

            // update total rewarded to user
            self.rewardeduser.insert(&user, &(reward + prior_total));

            // update total rewarded
            self.mut_rewarded_total(reward + self.rewardedtotal);

            true
        }

        /// emit Reward event
        #[ink(message)]
        pub fn emit_event_reward(
            &self,
            user: AccountId,
            reward: Balance,
        ) {
           
            // emit
            self.env().emit_event(Reward {
                to: Some(user),
                amount: reward,
            });

            ()
        }


    }

//// tests //////////////////////////////////////////////////////////////////////

//// To view debug prints and assertion failures run test via
//// cargo nightly+ test -- --nocapture

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        use ink_lang::codegen::Env;

        /// test if the default constructor does its job
        #[ink::test]
        fn constructor_works() {

            // instantiate contract account
            let ILOCKrewardsData = ILOCKrewardsData::new_ilockrewardsdata();

            assert_eq!(ILOCKrewardsData.owner, ILOCKrewardsData.env().caller());
            assert_eq!(ILOCKrewardsData.rewardedtotal, 0);
            assert_eq!(ILOCKrewardsData.rewardeduser.get(ILOCKrewardsData.env().caller()).unwrap(), 0);
        }

        /// test that total rewards getter does its job
        #[ink::test]
        fn rewarded_total_works() {

            // instantiate contract account
            let mut ILOCKrewardsData = ILOCKrewardsData::new_ilockrewardsdata();

            // set rewardedtotal
            ILOCKrewardsData.rewardedtotal = 100;

            assert_eq!(ILOCKrewardsData.rewarded_total(), 100);
        }

        /// test that total rewards incrementer does its job
        #[ink::test]
        fn mut_rewarded_total_works() {

            // instantiate contract account
            let mut ILOCKrewardsData = ILOCKrewardsData::new_ilockrewardsdata();

            // set rewardedtotal
            ILOCKrewardsData.rewardedtotal = 100;

            // mutate alice's reward total (increment only)
            ILOCKrewardsData.mut_rewarded_total(50);

            assert_eq!(ILOCKrewardsData.rewarded_total(), 150);
        }

        /// test that user rewards getter does its job
        #[ink::test]
        fn rewarded_user_works() {

            // instantiate contract account
            let mut ILOCKrewardsData = ILOCKrewardsData::new_ilockrewardsdata();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();


            // set rewardeduser
            ILOCKrewardsData.rewardeduser.insert(accounts.alice, &100);

            assert_eq!(ILOCKrewardsData.rewarded_user(accounts.alice), 100);
        }

        /// test that user rewards incrementer does its job
        #[ink::test]
        fn mut_rewarded_user_works() {

            // instantiate contract account
            let mut ILOCKrewardsData = ILOCKrewardsData::new_ilockrewardsdata();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();


            // set rewardedtotal
            ILOCKrewardsData.rewardeduser.insert(accounts.alice, &100);

            // mutate alice's reward total (increment only)
            ILOCKrewardsData.mut_rewarded_user(accounts.alice, 50);

            assert_eq!(ILOCKrewardsData.rewarded_user(accounts.alice), 150);
        }
    }
}
