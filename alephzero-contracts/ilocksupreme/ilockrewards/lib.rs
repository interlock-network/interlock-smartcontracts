//
// INTERLOCK NETWORK - MVP REWARDS CONTRACT
//
// !!!!! INCOMPLETE AND UNAUDITED, WARNING !!!!!
//
// This is simple contract that links the MVP browser extension
// with the ERC20 ILOCK token contract. This contract is responsible
// for doling out rewards as users browse the internet.


#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]

use ink_lang as ink;

#[ink::contract]
mod ilockrewards {

    use ilocktoken::ILOCKtokenRef;
    use ilockrewardsdata::ILOCKrewardsDataRef;
    use ink_env::call::FromAccountId;

    pub const ID_LENGTH: usize = 32;

    #[ink(storage)]
    pub struct ILOCKrewards {
        contract_ilocktoken: ILOCKtokenRef,
        contract_ilockrewardsdata: ILOCKrewardsDataRef,
        owner: AccountId,
        maxreward: Balance,
    }

    /// . error types
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum RewardsError {
        /// Returned if caller is not contract owner
        CallerNotOwner,
        /// Returned if specified reward recipient is zero address
        ZeroRecipientAddress,
        /// Returned if specified reward is too larger
        RewardTooLarge,
    }

    /// . RewardsError result type.
    pub type ResultRewards<T> = core::result::Result<T, RewardsError>;

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
                owner: Self::env().caller(),
                maxreward: 0,   // set this prior to construction
            }
        }

        /// check owner modifier
        pub fn not_owner(
            &self
        ) -> bool {

            self.env().caller() != self.owner
        }

        /// check user is not zero address
        pub fn is_zero(
            &self,
            address: AccountId,
        ) -> bool {

            address == ink_env::AccountId::from([0_u8; ID_LENGTH])
        }

        /// change max reward amount
        #[ink(message)]
        pub fn set_maxreward(
            &mut self,
            amount: Balance,
        ) -> ResultRewards<()> {
           
            // make sure caller is owner
            if self.not_owner() {
                return Err(RewardsError::CallerNotOwner)
            }

            // set
            self.maxreward = amount;

            Ok(())
        }

        /// get total amount rewarded to date
        #[ink(message)]
        pub fn total_rewarded(&self) -> Balance {
            self.contract_ilockrewardsdata.rewarded_total()
        }

        /// get amount rewarded to user to date
        #[ink(message)]
        pub fn total_rewarded_user(&self, user: AccountId) -> Balance {
            self.contract_ilockrewardsdata.rewarded_user(user)
        }

        /// reward the user for browsing
        #[ink(message)]
        pub fn reward_user(&mut self, reward: Balance, user: AccountId) -> ResultRewards<Balance> {

            // make sure caller is owner
            if self.not_owner() {
                return Err(RewardsError::CallerNotOwner)
            }

            // make sure reward recipient is not zero account
            if self.is_zero(user) {
                return Err(RewardsError::ZeroRecipientAddress)
            }

            // make sure reward isn't too large (protect reward pool in event server is compromised
            if reward > self.maxreward {
                return Err(RewardsError::RewardTooLarge)
            }

            // get total amount rewarded overall and to user so far
            let mut totalUserRewarded: Balance = self.contract_ilockrewardsdata.rewarded_user(user);

            // update total amount rewarded to user
            totalUserRewarded += reward;

            // update token circulation
            self.contract_ilocktoken.increment_circulation(reward);

            // get rewards pool address
            let rewards_pool = self.contract_ilocktoken.rewards_pool();

            // transfer reward tokens from rewards pool to user
            self.contract_ilocktoken.transfer_from(rewards_pool, user, reward).unwrap();

            // update state for user and total amounts rewarded
            self.contract_ilockrewardsdata.mut_rewarded_user(user, reward);
            self.contract_ilockrewardsdata.mut_rewarded_total(reward);

            // emit Reward event
            self.contract_ilockrewardsdata.emit_event_reward(user, reward);

            // this returns user total reward amount for extension display purposes
            Ok(totalUserRewarded)
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
                
            let contract1: AccountId = AccountId::from([0x11; ID_LENGTH]);
            let contract2: AccountId = AccountId::from([0x12; ID_LENGTH]);

            let ILOCKrewards = ILOCKrewards::new_ILOCKrewards(contract1, contract2);

            assert_eq!(ILOCKrewards.owner, ILOCKrewards.env().caller());
            assert_eq!(ILOCKrewards.maxreward, 0);
        }

        // I DO NOT KNOW HOW TO PERFORM A UNIT TEST FOR DELEGATOR THAT CALLS OTHER CONTRACTS...
        // ...PERHAPS THIS IS WHAT THEY MEAN BY 'INTEGRATION TESTS'
        
        // I'LL JUST TEST THE DELEGATED METHODS IN ILOCKREWARDSDATA CONTRACT
    }
}

