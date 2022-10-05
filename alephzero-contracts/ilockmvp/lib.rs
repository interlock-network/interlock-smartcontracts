//
// INTERLOCK NETWORK MVP SMART CONTRACTS
//  - PSP22 TOKEN
//  - REWARDS
//
// !!!!! INCOMPLETE AND UNAUDITED, WARNING !!!!!
//
// This is a standard ERC20-style token contract
// with provisions for enforcing a token distribution
// vesting schedule, and for rewarding interlockers for
// browsing the internet with the Interlock browser extension.


#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use self::ilocktoken::{
    ILOCKtoken,
    ILOCKtokenRef,
};

#[openbrush::contract]
pub mod ilocktoken {

    use ink_lang::{
        codegen::{EmitEvent, Env},
        reflect::ContractEventBase,
    };
    use ink_prelude::{
        format,
        string::{String, ToString},
    };
    use ink_storage::{
        Mapping,
        traits::{
            PackedLayout,
            SpreadLayout,
            SpreadAllocate,
        },
    };
    use openbrush::{
        contracts::psp22::*,
        traits::Storage,
    };

//// constants /////////////////////////////////////////////////////////////

    /// . magic numbers
    pub const ID_LENGTH: usize = 32;                                // 32B account id
    pub const POOL_COUNT: usize = 12;                               // number of stakeholder pools
    pub const ONE_MONTH: u128 = 2592000;                            // seconds in 30 days

    /// . token data
    pub const TOKEN_CAP: u128 = 1_000_000_000;                      // 10^9
    pub const DECIMALS_POWER10: u128 = 1_000_000_000_000_000_000;   // 10^18
    pub const SUPPLY_CAP: u128 = TOKEN_CAP * DECIMALS_POWER10;      // 10^27
    pub const TOKEN_NAME: &str = "Interlock Network";
    pub const TOKEN_DECIMALS: u8 = 18;
    pub const TOKEN_SYMBOL: &str = "ILOCK";

    /// . pool data
    pub const POOL_NAMES: [&str; POOL_COUNT] = [
                    "early_backers+venture_capital",
                    "presale_1",
                    "presale_2",
                    "presale_3",
                    "team+founders",
                    "outlier_ventures",
                    "advisors",
                    "rewards",
                    "foundation",
                    "partners",
                    "whitelist",
                    "public_sale",
                ];
    pub const POOL_TOKENS: [u128; POOL_COUNT] = [
                    20_000_000,
                    48_622_222,
                    66_666_667,
                    40_000_000,
                    200_000_000,
                    40_000_000,
                    25_000_000,
                    285_000_000,
                    172_711_111,
                    37_000_000,
                    15_000_000,
                    50_000_000,
                ];
    pub const POOL_VESTS: [u8; POOL_COUNT] = [
                    24,
                    18,
                    15,
                    12,
                    36,
                    24,
                    24,
                    1,
                    84,
                    1,
                    48,
                    1,
                ];
    pub const POOL_CLIFFS: [u8; POOL_COUNT] = [
                    1,
                    1,
                    1,
                    1,
                    6,
                    1,
                    1,
                    0,
                    1,
                    0,
                    0,
                    0,
                ];

//// structured data /////////////////////////////////////////////////////////////

    /// . StakeholderData struct contains all pertinent information for each stakeholder
    ///   (Besides balance and allowance mappings)
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct StakeholderData {
        paid: Balance,
        share: Balance,
        pool: u8,
    }

    /// . ILOCKtoken struct contains overall storage data for contract
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ILOCKtoken {
        #[storage_field]
        psp22: psp22::Data,
        owner: AccountId,
        stakeholderdata: Mapping<AccountId, StakeholderData>,
        rewardeduser: Mapping<AccountId, Balance>,
        rewardedtotal: Balance,
        rewardspoolbalance: Balance,
        monthspassed: u8,
        nextpayout: u128,
        circulatingsupply: Balance,
    }

//// PSP22 events /////////////////////////////////////////////////////////////

    /// . specify transfer event
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        amount: Balance,
    }

    /// . specify approve event
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        spender: Option<AccountId>,
        amount: Balance,
    }

    /// . specify reward event
    #[ink(event)]
    pub struct Reward {
        #[ink(topic)]
        to: Option<AccountId>,
        amount: Balance,
    }

    /// . Other contract error types
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum OtherError {
        /// Returned if caller is not contract owner
        CallerNotOwner,
        /// Returned if stakeholder share is entirely paid out
        StakeholderSharePaid,
        /// Returned if the stakeholder doesn't exist
        StakeholderNotFound,
        /// Returned if stakeholder has not yet passed cliff
        CliffNotPassed,
        /// Returned if it is too soon to payout for month
        PayoutTooEarly,
        /// Returned if reward is too large
        RewardTooLarge,
    }

    impl Into<PSP22Error> for OtherError {
        fn into(self) -> PSP22Error {
            PSP22Error::Custom(format!("{:?}", self))
        }
    }

    pub type PSP22Result<T> = core::result::Result<T, PSP22Error>;

    /// . OtherError result type.
    pub type ResultOther<T> = core::result::Result<T, OtherError>;

    pub type Event = <ILOCKtoken as ContractEventBase>::Type;

/////// init /////////////////////////////////////////////////////////////

    impl PSP22 for ILOCKtoken {}

    impl Internal for ILOCKtoken {
        fn _emit_transfer_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _amount: Balance,
        ) {
            ILOCKtoken::emit_event(
                self.env(),
                Event::Transfer(Transfer {
                    from: _from,
                    to: _to,
                    amount: _amount,
                }),
            );
        }

        fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance) {
            ILOCKtoken::emit_event(
                self.env(),
                Event::Approval(Approval {
                    owner: Some(_owner),
                    spender: Some(_spender),
                    amount: _amount,
                }),
            );
        }
    }

    impl ILOCKtoken {

        /// . constructor to initialize contract
        /// . note: pool contracts must be created prior to construction (for args)
        #[ink(constructor)]
        // takes in array of pool addresses generated earlier, pre token contract constructor
        pub fn new_token(
        ) -> Self {

            // create contract
            ink_lang::codegen::initialize_contract(|contract: &mut Self| {

                // define owner as caller
                let caller = Self::env().caller();

                // set initial data
                contract.monthspassed = 0;
                contract.nextpayout = Self::env().block_timestamp() as u128 + ONE_MONTH;
                contract.owner = caller;
                contract.rewardedtotal = 0;

                // mint with openbrush:
                contract._mint(caller, SUPPLY_CAP)
                        .expect("Failed to mint the initial supply");


                // emit Transfer event
                Self::env().emit_event(Transfer {
                    from: Some(ink_env::AccountId::from([0_u8; ID_LENGTH])),
                    to: Some(Self::env().account_id()),
                    amount: SUPPLY_CAP,
                });

                // emit Approval event
                Self::env().emit_event(Approval {
                    owner: Some(Self::env().account_id()),
                    spender: Some(caller),
                    amount: SUPPLY_CAP,
                });

                // reflect initial circulation
                // ...these may be inappropriate, as we may increment circulation
                // every time we pay a whitelister, or every time somebody buys tokens during
                // public sale...

                // TODO: handle the error cases here
                // whitelist
                contract.increment_circulation(POOL_TOKENS[10] * DECIMALS_POWER10);
                // public sale
                contract.increment_circulation(POOL_TOKENS[11] * DECIMALS_POWER10);
                contract.rewardspoolbalance = POOL_TOKENS[7] * DECIMALS_POWER10;

            })
        }

        pub fn emit_event<EE: EmitEvent<Self>>(emitter: EE, event: Event) {
            emitter.emit_event(event);
        }

/////// modifiers ///////////////////////////////////////////////////////////

        /// . make sure caller is owner
        pub fn not_owner(
            &self,
        ) -> bool {
            self.env().caller() != self.owner
        }

        /// . make sure transfer amount is available
        pub fn not_enough(
            &self,
            holder: AccountId,
            amount: Balance,
        ) -> bool {
            self.balance_of(holder) < amount
        }

        /// . make sure allowance is sufficient
        pub fn not_allowed(
            &self,
            holder: AccountId,
            spender: AccountId,
            amount: Balance,
        ) -> bool {
            self.allowance(holder, spender) < amount
        }

        /// . make sure account is not zero account
        pub fn is_zero(
            &self,
            account: AccountId,
        ) -> bool {
            account == ink_env::AccountId::from([0_u8; ID_LENGTH])
        }

/////// PSP22 getters ///////////////////////////////////////////////////////////

        /// . token name getter
        #[ink(message)]
        pub fn name(
            &self,
        ) -> Option<String> {

            Some(TOKEN_NAME.to_string())
        }

        /// . token symbol getter
        #[ink(message)]
        pub fn symbol(
            &self,
        ) -> Option<String> {

            Some(TOKEN_SYMBOL.to_string())
        }

        /// . token decimal count getter
        #[ink(message)]
        pub fn decimals(
            &self,
        ) -> u8 {

            TOKEN_DECIMALS
        }

        /// . total circulating supply getter
        #[ink(message)]
        pub fn total_supply(
            &self,
        ) -> Balance {

            self.circulatingsupply
        }


/////// timing /////////////////////////////////////////////////////////////

        /// . function to check if enough time has passed to collect next payout
        /// . this function ensures Interlock cannot rush the vesting schedule
        #[ink(message)]
        pub fn check_time(
            &mut self,
        ) -> ResultOther<()> {

            // test to see if current time falls beyond time for next payout
            if self.env().block_timestamp() as u128 > self.nextpayout {

                // update time variables
                self.nextpayout += ONE_MONTH;
                self.monthspassed += 1;

                ()
            }

            // too early
            return Err(OtherError::PayoutTooEarly)
        }

/////// registration  /////////////////////////////////////////////////////////////

        /// . function that registers a stakeholder's wallet and vesting info
        /// . used to calculate monthly payouts and track net paid
        /// . stakeholder data also used for stakeholder to verify their place in vesting schedule
        #[ink(message)]
        pub fn register_stakeholder(
            &mut self,
            stakeholder: AccountId,
            share: Balance,
            pool: u8,
        ) -> ResultOther<()> {

            // only owner can call
            if self.not_owner() {
                return Err(OtherError::CallerNotOwner)
            }

            // create stakeholder struct
            let this_stakeholder = StakeholderData {
                paid: 0,
                share: share,
                pool: pool,
            };

            // insert stakeholder struct into mapping
            self.stakeholderdata.insert(stakeholder, &this_stakeholder);

            Ok(())
        }


/////// token distribution /////////////////////////////////////////////////////////////

        /// . function to transfer the token share a stakeholder is currently entitled to
        /// . this is called once per stakeholder by Interlock, Interlock paying fees
        /// . pools are guaranteed to have enough tokens for all stakeholders
        #[ink(message)]
        pub fn distribute_tokens(
            &mut self,
            stakeholder: AccountId,
        ) -> PSP22Result<()> {

            // make sure caller is owner
            if self.not_owner() {
                return Err(OtherError::CallerNotOwner.into());
            }

            // get data structs
            let mut this_stakeholder = match self.stakeholderdata.get(stakeholder) {
                Some(s) => s,
                None => { return Err(OtherError::StakeholderNotFound.into()) },
            };
            let pool = this_stakeholder.pool;

            // require cliff to have been surpassed
            if self.monthspassed < POOL_CLIFFS[pool as usize] {
                return Err(OtherError::CliffNotPassed.into());
            }

            // require share has not been completely paid out
            if this_stakeholder.paid == this_stakeholder.share {
                return Err(OtherError::StakeholderSharePaid.into())
            }

            // calculate the payout owed
            let mut payout: Balance = this_stakeholder.share / POOL_VESTS[pool as usize] as Balance;

            // if this is final payment, add token remainder to payout
            // (this is to compensate for floor division that calculates payamount)
            if this_stakeholder.share - this_stakeholder.paid - payout <
                this_stakeholder.share / POOL_VESTS[pool as usize] as Balance {

                // add remainder
                payout += this_stakeholder.share % POOL_VESTS[pool as usize] as Balance;
            }

            // now transfer tokens
            let _ = self.transfer(stakeholder, payout, Default::default())?;

            // update circulating supply
            let _ = self.increment_circulation(payout)?;

            // finally update stakeholder data struct state
            this_stakeholder.paid += payout;
            self.stakeholderdata.insert(stakeholder, &this_stakeholder);

            Ok(())
        }

/////// stakeholder data ////////////////////////////////////////////////////////////

        /// . function that returns a stakeholder's payout data
        /// . this will allow stakeholders to verify their stake from explorer if so motivated
        /// . returns tuple (paidout, payremaining, payamount, poolnumber)
        #[ink(message)]
        pub fn stakeholder_data(
            &self,
            stakeholder: AccountId,
        ) -> (Balance, Balance, Balance, u8) {

            // get pool and stakeholder data structs first
            let this_stakeholder = self.stakeholderdata.get(stakeholder).unwrap();
            let pool = this_stakeholder.pool;

            // how much has stakeholder already claimed?
            let paidout: Balance = this_stakeholder.paid;

            // how much does stakeholder have yet to collect?
            let payremaining: Balance = this_stakeholder.share - this_stakeholder.paid;

            // how much does stakeholder get each month?
            let payamount: Balance = this_stakeholder.share / POOL_VESTS[pool as usize] as Balance;

            return (
                paidout,
                payremaining,
                payamount,
                pool,
            )
        }

/////// pool data ////////////////////////////////////////////////////////////

        /// . function that returns pool data
        /// . this will allow observers to verify vesting parameters for each pool (esp. theirs)
        /// . observers may verify pool data from explorer if so motivated
        /// . pool numbers range from 0-11
        /// . returns (name, tokens, vests, cliff)
        #[ink(message)]
        pub fn pool_data(
            &self,
            pool: u8,
        ) -> (String, u128, u8, u8) {
        
            // just grab up and send it out
            return (
                POOL_NAMES[pool as usize].to_string(),
                POOL_TOKENS[pool as usize],
                POOL_VESTS[pool as usize],
                POOL_CLIFFS[pool as usize],
            )
        }

//// rewarding  //////////////////////////////////////////////////////////////////////

        /// . reward the user for browsing
        #[ink(message)]
        pub fn reward_user(&mut self, reward: Balance, user: AccountId) -> PSP22Result<Balance> {

            // make sure caller is owner
            if self.not_owner() {
                return Err(OtherError::CallerNotOwner.into())
            }

            // update total amount rewarded to user
            self.rewardedtotal += reward;

            // update token circulation
            let _ = self.increment_circulation(reward)?;

            // update rewards pool balance
            self.rewardspoolbalance -= reward;

            // transfer reward tokens from rewards pool to user
            self.transfer(user, reward, Default::default())?;

            let rewardedusertotal: Balance = match self.rewardeduser.get(user) {
                Some(u) => u,
                None => {
                    return Err(PSP22Error::Custom(format!("User {:?} not found", user)))
                },
            };
            self.rewardeduser.insert(user, &(rewardedusertotal + reward));

            // emit Reward event
            self.env().emit_event(Reward {
                to: Some(user),
                amount: reward,
            });

            // this returns user total reward amount for extension display purposes
            Ok(rewardedusertotal + reward)
        }

        /// . get amount rewarded to user to date
        #[ink(message)]
        pub fn rewarded_user_total(&self, user: AccountId) -> PSP22Result<Balance> {

            match self.rewardeduser.get(user) {
                Some(t) => Ok(t),
                None => Err(PSP22Error::Custom(format!("User {:?} not found", user))),
            }
        }

        /// . get total amount rewarded to date
        #[ink(message)]
        pub fn rewarded_total(&self) -> Balance {

            self.rewardedtotal
        }

        /// . get current balance of rewards pool
        #[ink(message)]
        pub fn rewards_pool_balance(&self) -> Balance {

            self.rewardspoolbalance
        }

//// misc  //////////////////////////////////////////////////////////////////////
        
        /// . function to get the number of months passed for contract
        #[ink(message)]
        pub fn months_passed(
            &self,
        ) -> u8 {
            self.monthspassed
        }

        /// . function to increment circulatingsupply after reward issue or stakeholder payment
        #[ink(message)]
        pub fn increment_circulation(
            &mut self,
            amount: u128,
        ) -> PSP22Result<()> {

            if self.not_owner() {
                return Err(OtherError::CallerNotOwner.into())
            }

            self.circulatingsupply += amount;
            Ok(())
        }

        /// . function to decrement circulatingsupply after burn or reward reclaim
        #[ink(message)]
        pub fn decrement_circulation(
            &mut self,
            amount: u128,
        ) -> PSP22Result<()> {

            if self.not_owner() {
                return Err(OtherError::CallerNotOwner.into())
            }

            self.circulatingsupply -= amount;
            Ok(())
        }

        /// . function to increment monthspassed for testing
        #[ink(message)]
        pub fn TESTING_increment_month(
            &mut self,
        ) -> bool {

            self.monthspassed += 1;
            true
        }

        /// . function to change contract owners
        #[ink(message)]
        pub fn change_owner(
            &mut self,
            newowner: AccountId,
        ) -> PSP22Result<()> {

            if self.not_owner() {
                return Err(OtherError::CallerNotOwner.into())
            }

            self.owner = newowner;

            Ok(())
        }

        /// . function to disown contract
        /// . upgrade and disown if Interlock Foundation goes under
        #[ink(message)]
        pub fn disown(
            &mut self,
        ) -> PSP22Result<()> {

            if self.not_owner() {
                return Err(OtherError::CallerNotOwner.into())
            }

            self.owner = ink_env::AccountId::from([0_u8; ID_LENGTH]);

            Ok(())
        }

        /// . burn function to permanently remove tokens from circulation / supply
        #[ink(message)]
        pub fn burn(
            &mut self,
            donor: AccountId,
            amount: Balance,
        ) -> PSP22Result<()> {

            // make sure owner is caller
            if self.not_owner() {
                return Err(OtherError::CallerNotOwner.into())
            }

            // burn the tokens
            let _ = self._burn_from(donor, amount)?;
            self.decrement_circulation(amount)?;

            // emit transfer event
            self.env().emit_event(Transfer {
                from: Some(donor),
                to: Some(ink_env::AccountId::from([0_u8; ID_LENGTH])),
                amount: amount,
            });

            Ok(())
        }

        /// . modifies the code which is used to execute calls to this contract address
        /// . this upgrades the token contract logic while using old state
        #[ink(message)]
        pub fn set_code(
            &mut self,
            code_hash: [u8; 32]
        ) -> PSP22Result<()> {

            // make sure caller is owner
            if self.not_owner() {
                return Err(OtherError::CallerNotOwner.into())
            }

            // takes code hash of updates contract and modifies preexisting logic to match
            ink_env::set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!(
                    "Failed to `set_code_hash` to {:?} due to {:?}",
                    code_hash, err
                )
            });

            Ok(())
        }
    }

//// tests //////////////////////////////////////////////////////////////////////

// . To view debug prints and assertion failures run test via:
// cargo nightly+ test -- --nocapture
// . To view debug for specific method run test via:
// cargo nightly+ test <test_function_here> -- --nocapture

    #[cfg(test)]
    mod tests {

        use super::*;
        use ink_lang as ink;
        use ink_env::Clear;
        use ink_env::topics::PrefixedValue;
        use ink_lang::codegen::Env;

        type Event = <ILOCKtoken as ::ink_lang::reflect::ContractEventBase>::Type;

        /// . test if the default constructor does its job
        #[ink::test]
        fn constructor_works() {

            let ILOCKtokenPSP22 = ILOCKtoken::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // the rest
            assert_eq!(ILOCKtokenPSP22.owner, accounts.alice);
            assert_eq!(ILOCKtokenPSP22.monthspassed, 0);
            assert_eq!(ILOCKtokenPSP22.nextpayout, ILOCKtokenPSP22.env().block_timestamp() as u128 + ONE_MONTH);
        }

        /// . test if name getter does its job
        #[ink::test]
        fn name_works() {

            let ILOCKtokenPSP22 = ILOCKtoken::new_token();
            assert_eq!(ILOCKtokenPSP22.name(), Some("Interlock Network".to_string()));
        }

        /// . test if symbol getter does its job
        #[ink::test]
        fn symbol_works() {

            let ILOCKtokenPSP22 = ILOCKtoken::new_token();
            assert_eq!(ILOCKtokenPSP22.symbol(), Some("ILOCK".to_string()));
        }
        
        /// . test if decimals getter does its job
        #[ink::test]
        fn decimals_works() {

            let ILOCKtokenPSP22 = ILOCKtoken::new_token();
            assert_eq!(ILOCKtokenPSP22.decimals(), 18);
        }

        /// . test if total supply getter does its job
        #[ink::test]
        fn totalsupply_works() {

            let ILOCKtokenPSP22 = ILOCKtoken::new_token();
            assert_eq!(ILOCKtokenPSP22.total_supply(), 65_000_000 * DECIMALS_POWER10);
        }

        /// . test if balance getter does its job
        #[ink::test]
        fn balance_of_works() {

            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // charge alice's account
            ILOCKtokenPSP22.psp22.balances.insert(&accounts.alice, &100);

            assert_eq!(ILOCKtokenPSP22.balance_of(accounts.alice), 100);
        }

        /// . test if allowance getter does its job
        #[ink::test]
        fn allowance_works() {

            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice has not yet approved Bob
            assert_eq!(ILOCKtokenPSP22.allowance(accounts.alice, accounts.bob), 0);

            // Alice approves Bob for tokens
            assert_eq!(ILOCKtokenPSP22.approve(accounts.bob, 10), Ok(()));

            // Bob's new allowance reflects this approval
            assert_eq!(ILOCKtokenPSP22.allowance(accounts.alice, accounts.bob), 10);
        }

        /// . test if the transfer doer does its job
        #[ink::test]
        fn transfer_works() {

            // construct contract and initialize accounts
            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // charge alice's account
            ILOCKtokenPSP22.psp22.balances.insert(&accounts.alice, &100);

            // alice transfers tokens to bob
            assert_eq!(ILOCKtokenPSP22.transfer(accounts.bob, 10, Default::default()), Ok(()));

            // Alice balance reflects transfer
            assert_eq!(ILOCKtokenPSP22.balance_of(accounts.alice), 90);

            // Bob balance reflects transfer
            assert_eq!(ILOCKtokenPSP22.balance_of(accounts.bob), 10);

            // Alice attempts transfer too large
            assert_eq!(ILOCKtokenPSP22.transfer(accounts.bob, 100, Default::default()), Err(PSP22Error::InsufficientBalance));

            // check all events that happened during the previous calls
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);

            // check the transfer event relating to the actual trasfer
            assert_transfer_event(
                &emitted_events[2],
                Some(AccountId::from([0x01; ID_LENGTH])),
                Some(AccountId::from([0x02; ID_LENGTH])),
                10,
            );
        }

        /// . test if the approve does does its job
        #[ink::test]
        fn approve_works() {

            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice approves bob to spend tokens
            assert_eq!(ILOCKtokenPSP22.approve(accounts.bob, 10), Ok(()));

            // Bob is approved to spend tokens owned by Alice
            assert_eq!(ILOCKtokenPSP22.allowance(accounts.alice, accounts.bob), 10);

            // check all events that happened during previous calls
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);

            // check the approval event relating to the actual approval
            assert_approval_event(
                &emitted_events[2],
                Some(AccountId::from([0x01; ID_LENGTH])),
                Some(AccountId::from([0x02; ID_LENGTH])),
                10,
            );
        }

        /// . test if the transfer-from doer does its job
        #[ink::test]
        fn transfer_from_works() {

            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // charge alice's account
            ILOCKtokenPSP22.psp22.balances.insert(&accounts.alice, &100);

            // Alice approves Bob for token transfers on her behalf
            assert_eq!(ILOCKtokenPSP22.approve(accounts.bob, 10), Ok(()));

            // set the contract owner as callee and Bob as caller
            let contract = ink_env::account_id::<ink_env::DefaultEnvironment>();
            ink_env::test::set_callee::<ink_env::DefaultEnvironment>(contract);
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.bob);

            // Check Bob's allowance
            assert_eq!(ILOCKtokenPSP22.allowance(accounts.alice, accounts.bob), 10);

            // and Bob is caller now
            assert_eq!(ILOCKtokenPSP22.env().caller(), accounts.bob);

            // Bob transfers tokens from Alice to Eve
            assert_eq!(ILOCKtokenPSP22.transfer_from(accounts.alice, accounts.eve, 10, Default::default()), Ok(()));

            // Eve received the tokens
            assert_eq!(ILOCKtokenPSP22.balance_of(accounts.eve), 10);

            // Bob attempts a transferfrom too large
            assert_eq!(ILOCKtokenPSP22.transfer_from(accounts.alice, accounts.eve, 100, Default::default()),
                        Err(PSP22Error::InsufficientAllowance));

            // check all events that happened during the previous callsd
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 5);

            // check that Transfer event was emitted        
            assert_transfer_event(
                &emitted_events[4],
                Some(AccountId::from([0x01; ID_LENGTH])),
                Some(AccountId::from([0x05; ID_LENGTH])),
                10,
            );
        }

        /// . test if increase allowance does does its job
        #[ink::test]
        fn increase_allowance_works() {

            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice approves bob to spend tokens
            assert_eq!(ILOCKtokenPSP22.approve(accounts.bob, 10), Ok(()));

            // Bob is approved to spend tokens owned by Alice
            assert_eq!(ILOCKtokenPSP22.allowance(accounts.alice, accounts.bob), 10);

            // Alice increases Bobs allowance
            assert_eq!(ILOCKtokenPSP22.increase_allowance(accounts.bob, 10), Ok(()));

            // Bob is approved to spend extra tokens owned by Alice
            assert_eq!(ILOCKtokenPSP22.allowance(accounts.alice, accounts.bob), 20);
        }

        /// . test if decrease allowance does does its job
        #[ink::test]
        fn decrease_allowance_works() {

            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice approves bob to spend tokens
            assert_eq!(ILOCKtokenPSP22.approve(accounts.bob, 10), Ok(()));

            // Bob is approved to spend tokens owned by Alice
            assert_eq!(ILOCKtokenPSP22.allowance(accounts.alice, accounts.bob), 10);

            // Alice increases Bobs allowance
            assert_eq!(ILOCKtokenPSP22.decrease_allowance(accounts.bob, 5), Ok(()));

            // Bob is approved to spend extra tokens owned by Alice
            assert_eq!(ILOCKtokenPSP22.allowance(accounts.alice, accounts.bob), 5);
        }

        /// . test if wallet registration function works as intended 
        #[ink::test]
        fn register_stakeholder_works() {

            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // bob's stakeholder data
            let share: Balance = 1_000_000;
            let pool: u8 = 3;

            // call registration function
            ILOCKtokenPSP22.register_stakeholder(accounts.bob, share, pool).unwrap();

            // verify registration stuck
            let this_stakeholder = ILOCKtokenPSP22.stakeholderdata.get(accounts.bob).unwrap();
            assert_eq!(this_stakeholder.paid, 0);
            assert_eq!(this_stakeholder.share, share);
            assert_eq!(this_stakeholder.pool, pool);
        }
       
        /// . test if the approve does does its job
        #[ink::test]
        fn distribute_tokens_works() {

            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // bob's stakeholder data
            let share: Balance = 1_000_000;

            let mut pool = 4;

            // register bob, 6 month cliff, 36 vests (pool 4)
            ILOCKtokenPSP22.register_stakeholder(accounts.bob, share, pool).unwrap();

            // debug println header (if no capture is on)
            ink_env::debug_println!("POOL 4, 36 MONTH VESTING PERIOD, 6 MONTH CLIFF");
            // run distribution over 44 months (6 + 36 + 2)
            for _month in 0..44 {

                // get bob his monthly tokens
                ILOCKtokenPSP22.distribute_tokens(accounts.bob).ok();

                // print everything and check balances at each iteration
                let this_stakeholder: StakeholderData = ILOCKtokenPSP22.stakeholderdata.get(accounts.bob).unwrap();
                ink_env::debug_println!("month: {:?}\tpaid: {:?}", ILOCKtokenPSP22.monthspassed, this_stakeholder.paid);
                assert_eq!(ILOCKtokenPSP22.balance_of(accounts.bob), this_stakeholder.paid);

                // make time go on
                ILOCKtokenPSP22.TESTING_increment_month();
            }

            // reset time
            ILOCKtokenPSP22.monthspassed = 0;

            pool = 1;

            // register bob, 1 month cliff, 18 vests (pool 1)
            ILOCKtokenPSP22.register_stakeholder(accounts.bob, share, pool).unwrap();
            ILOCKtokenPSP22.psp22.balances.insert(&accounts.bob, &0);

            // debug println header (if no capture is on)
            ink_env::debug_println!("POOL 1, 18 MONTH VESTING PERIOD, 1 MONTH CLIFF");
            // run distribution over 44 months (1 + 18 + 2)
            for _month in 0..21 {

                // get bob his monthly tokens
                //ILOCKtokenPSP22.distribute_tokens(accounts.bob).ok();

                // print everything and check balances at each iteration
                let this_stakeholder: StakeholderData = ILOCKtokenPSP22.stakeholderdata.get(accounts.bob).unwrap();
                ink_env::debug_println!("month: {:?}\tpaid: {:?}", ILOCKtokenPSP22.monthspassed, this_stakeholder.paid);
                assert_eq!(ILOCKtokenPSP22.balance_of(accounts.bob), this_stakeholder.paid);

                // make time go on
                ILOCKtokenPSP22.TESTING_increment_month();
            }

            // reset time
            ILOCKtokenPSP22.monthspassed = 0;

            pool = 10;

            // register bob, 0 month cliff, 48 vests (pool 10)
            ILOCKtokenPSP22.register_stakeholder(accounts.bob, share, pool).unwrap();
            ILOCKtokenPSP22.psp22.balances.insert(&accounts.bob, &0);

            // debug println header (if no capture is on)
            ink_env::debug_println!("POOL 10, 48 MONTH VESTING PERIOD, 0 MONTH CLIFF");
            // run distribution over 44 months (0 + 48 + 2)
            for _month in 0..50 {

                // get bob his monthly tokens
                ILOCKtokenPSP22.distribute_tokens(accounts.bob).ok();

                // print everything and check balances at each iteration
                let this_stakeholder: StakeholderData = ILOCKtokenPSP22.stakeholderdata.get(accounts.bob).unwrap();
                ink_env::debug_println!("month: {:?}\tpaid: {:?}", ILOCKtokenPSP22.monthspassed, this_stakeholder.paid);
                assert_eq!(ILOCKtokenPSP22.balance_of(accounts.bob), this_stakeholder.paid);

                // make time go on
                ILOCKtokenPSP22.TESTING_increment_month();
            }
        }

        /// . test if pool data getter does its job
        #[ink::test]
        fn pool_data_works() {

            let ILOCKtokenPSP22 = ILOCKtoken::new_token();
            assert_eq!(ILOCKtokenPSP22.pool_data(1), (POOL_NAMES[1].to_string(),
                                                      POOL_TOKENS[1],
                                                      POOL_VESTS[1],
                                                      POOL_CLIFFS[1]));
        }

        /// . test if months passed getter does its job
        #[ink::test]
        fn months_passed_works() {

            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();
            ILOCKtokenPSP22.monthspassed = 99;
            assert_eq!(ILOCKtokenPSP22.months_passed(), 99);
        }

        /// . test if circulation incrementor does its job
        #[ink::test]
        fn increment_circulation_works() {

            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();

            ILOCKtokenPSP22.increment_circulation(100);
            assert_eq!(ILOCKtokenPSP22.total_supply(), 65_000_000 * DECIMALS_POWER10 + 100);
        }

        /// . test if circulation decrementor does its job
        #[ink::test]
        fn decrement_circulation_works() {

            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();

            ILOCKtokenPSP22.decrement_circulation(100);
            assert_eq!(ILOCKtokenPSP22.total_supply(), 65_000_000 * DECIMALS_POWER10 - 100);
        }

        /// . test if change owner does its job
        #[ink::test]
        fn change_owner_works() {

            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            assert_eq!(accounts.alice, ILOCKtokenPSP22.owner);
            ILOCKtokenPSP22.change_owner(accounts.bob).unwrap();
            assert_eq!(accounts.bob, ILOCKtokenPSP22.owner);
        }

        /// . test if disowner does its job
        #[ink::test]
        fn disown_works() {

            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            assert_eq!(accounts.alice, ILOCKtokenPSP22.owner);
            ILOCKtokenPSP22.disown().unwrap();
            assert_eq!(AccountId::from([0_u8; ID_LENGTH]), ILOCKtokenPSP22.owner);
        }

        /// . test if burn does its job
        #[ink::test]
        fn burn_works() {

            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // charge alice's account
            ILOCKtokenPSP22.psp22.balances.insert(&accounts.alice, &100);

            // alice has her tokens burned by contract owner (herself in this case)
            ILOCKtokenPSP22.burn(accounts.alice, 100).unwrap();

            assert_eq!(ILOCKtokenPSP22.balance_of(accounts.alice), 0);
            assert_eq!(ILOCKtokenPSP22.total_supply(), 65_000_000 * DECIMALS_POWER10 - 100);
        }

/////// testing helpers  //////////////////////////////////////////////////////////////////////

        /// . check that a transfer event is good
        fn assert_transfer_event(
            event: &ink_env::test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_amount: u128,
        ) {

            // decode Event object
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");

            // check event is expected transfer event
            if let Event::Transfer(Transfer { from, to, amount }) = decoded_event {

                // make sure accounts match what we expect
                assert_eq!(from, expected_from, "encountered invalid Transfer.from");
                assert_eq!(to, expected_to, "encountered invalid Transfer.to");
                assert_eq!(amount, expected_amount, "encountered invalid Trasfer.amount");
            } else {
                panic!("encountered unexpected event kind: expected a Transfer event")
            }

            // define expected topics for Transfer event
            let expected_topics = vec![
                encoded_into_hash(&PrefixedValue {
                    value: b"ILOCKtoken::Transfer",
                    prefix: b"",
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"ILOCKtoken::Transfer::from",
                    value: &expected_from,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"ILOCKtoken::Transfer::to",
                    value: &expected_to,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"ILOCKtoken::Transfer::amount",
                    value: &expected_amount,
                }),
            ];

            // get actual topics for event
            let topics = event.topics.clone();

            // check that actual topics match expected topics
            for (n, (actual_topic, expected_topic)) in
                topics.iter().zip(expected_topics).enumerate()
            {
                let mut topic_hash = Hash::clear();
                let len = actual_topic.len();
                topic_hash.as_mut()[0..len].copy_from_slice(&actual_topic[0..len]);
                assert_eq!(topic_hash, expected_topic, "encountered invalid topic at {}", n);
            }
        }

        /// . check that an approval event is good
        fn assert_approval_event(
            event: &ink_env::test::EmittedEvent,
            expected_owner: Option<AccountId>,
            expected_spender: Option<AccountId>,
            expected_amount: u128,
        ) {

            // decode Event object
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");

            // check event is expected approval event
            if let Event::Approval(Approval { owner, spender, amount }) = decoded_event {
                assert_eq!(owner, expected_owner, "encountered invalid Approval.owner");
                assert_eq!(spender, expected_spender, "encountered invalid Approval.spender");
                assert_eq!(amount, expected_amount, "encountered invalid Approval.amount");
            } else {
                panic!("encountered unexpected event kind: expected a Approval event")
            }

            // define expected topics for Approval event
            let expected_topics = vec![
                encoded_into_hash(&PrefixedValue {
                    value: b"ILOCKtoken::Approval",
                    prefix: b"",
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"ILOCKtoken::Approval::owner",
                    value: &expected_owner,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"ILOCKtoken::Approval::spender",
                    value: &expected_spender,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"ILOCKtoken::Approval::amount",
                    value: &expected_amount,
                }),
            ];

            // get actual topics for event
            let topics = event.topics.clone();

            // check that actual topics match expected topics
            for (n, (actual_topic, expected_topic)) in
                topics.iter().zip(expected_topics).enumerate()
            {
                let mut topic_hash = Hash::clear();
                let len = actual_topic.len();
                topic_hash.as_mut()[0..len].copy_from_slice(&actual_topic[0..len]);
                assert_eq!(topic_hash, expected_topic, "encountered invalid topic at {}", n);
            }
        }

        /// . this is a painful hashing function for use in event assert functions
        fn encoded_into_hash<T>(entity: &T) -> Hash
        where
            T: scale::Encode,
        {
            use ink_env::{
                hash::{
                    Blake2x256,
                    CryptoHash,
                    HashOutput,
                },
                Clear,
            };
            let mut result = Hash::clear();
            let len_result = result.as_ref().len();
            let encoded = entity.encode();
            let len_encoded = encoded.len();
            if len_encoded <= len_result {
                result.as_mut()[..len_encoded].copy_from_slice(&encoded);
                return result
            }
            let mut hash_output =
                <<Blake2x256 as HashOutput>::Type as Default>::default();
            <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
            let copy_len = core::cmp::min(hash_output.len(), len_result);
            result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
            result
        }
    }

}
