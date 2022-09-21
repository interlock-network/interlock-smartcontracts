/////////////////////////////////////////////////////////////////
//
// INTERLOCK NETWORK
//
// !!!!! INCOMPLETE AND FLAWED, WARNING !!!!!
//
// NOTE: To enable unsigned integer division, overflow_checks
// has been turned 'off' in Cargo.toml file.
// However, this was for ink 3.0.1. This may be rectified in 3.3.1
//
/////////////////////////////////////////////////////////////////

#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use self::ilocktoken::{
    ILOCKtoken,
    ILOCKtokenRef,
};

use ink_lang as ink;

#[ink::contract]
pub mod ilocktoken {

    use ink_lang::utils::initialize_contract;
    use ink_prelude::string::String;
    use ink_prelude::string::ToString;
    use ink_storage::{
        Mapping,
        traits::{
            PackedLayout,
            SpreadLayout,
            SpreadAllocate,
        },
    };

//// constants /////////////////////////////////////////////////////////////

    /// . magic numbers
    pub const ID_LENGTH: usize = 32;                                // 32B account id
    pub const POOL_COUNT: usize = 12;                               // number of stakeholder pools
    pub const MEMBER_COUNT: usize = 1000;                           // number of vesting stakeholders
    pub const ONE_MONTH: u128 = 2592000;                                // seconds in 30 days

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

    /// . PoolData struct contains all pertinant information about the various token pools
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct PoolData {
        name: String,
        tokens: u128,
        vests: u8,
        cliff: u8,
    }

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
    #[derive(SpreadAllocate)]
    #[ink(storage)]
    pub struct ILOCKtoken {
        owner: AccountId,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
        stakeholderdata: Mapping<AccountId, StakeholderData>,
        pooldata: Mapping<AccountId, PoolData>,
        pools: [AccountId; POOL_COUNT], // this is pattern to iterate through pooldata mapping
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

//// PSP22 errors /////////////////////////////////////////////////////////////

    /// . PSP22 error types, per standard
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum PSP22Error {
        /// Custom error
        Custom(String),
        /// Returned if not enough balance to fulfill a request
        InsufficientBalance,
        /// Returned if not enough allowance to fulfill a request
        InsufficientAllowance,
        /// Returned if recipient is zero address
        ZeroRecipientAddress,
        /// Returned if sender is zero address
        ZeroSenderAddress,
        /// Returned if receiving contract does not accept tokens
        SafeTransferCheckFailed(String),
    }

    // NEED TO FIGURE OUT HOW TO IMPLEMENT RECEIVER

    /// . PSP22 receiver error type, per standard
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum PSP22ReceiverError {
        /// Returned if a transfer is rejected.
        TransferRejected(String),
    }

    /// . Other contract error types
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum OtherError {
        /// Returned if caller is not contract owner
        CallerNotOwner,
        /// Returned if stakeholder share is entirely paid out
        StakeholderSharePaid,
        /// Returned if stakeholder has not yet passed cliff
        CliffNotPassed,
        /// Returned if it is too soon to payout for month
        PayoutTooEarly,
    }

    // NEED TO FIND OUT IF THESE CUSTOM RESULT TYPES RETURN Result<T, xxxx> OR IF ResultXxxx<T>
    // ...IF LATTER, MAY NOT SATISFY SPS22 STANDARD INTERFACE

    /// . PSP22Error result type.
    pub type ResultPSP22<T> = core::result::Result<T, PSP22Error>;

    /// . PSP22ReceiverError result type.
    pub type ResultPSP22Receiver<T> = core::result::Result<T, PSP22ReceiverError>;

    /// . OtherError result type.
    pub type ResultOther<T> = core::result::Result<T, OtherError>;

/////// init /////////////////////////////////////////////////////////////

    impl ILOCKtoken {

        /// . constructor to initialize contract
        /// . note: pool contracts must be created prior to construction (for args)
        #[ink(constructor)]
        // takes in array of pool addresses generated earlier, pre token contract constructor
        pub fn new_token(
            pools: [AccountId; POOL_COUNT],
        ) -> Self {

            // create contract
            initialize_contract(|contract: &mut Self| {

                // define owner as caller
                let caller = Self::env().caller();

                // assign pool data
                for pool in 0..POOL_COUNT {
                    contract.pools[pool] = pools[pool];

                    // define pooldata struct for this pool
                    let this_pool = PoolData {
                        name: POOL_NAMES[pool].to_string(),
                        tokens: POOL_TOKENS[pool] * DECIMALS_POWER10,
                        vests: POOL_VESTS[pool],
                        cliff: POOL_CLIFFS[pool],
                    };

                    // push current pool into pooldata map
                    contract.pooldata.insert(pools[pool], &this_pool);

                    // charge pool with tokens
                    contract.balances.insert(pools[pool], &this_pool.tokens);

                    // give owner control over pool
                    contract.allowances.insert((pools[pool], caller), &this_pool.tokens);

                    // emit mint Transfer event
                    Self::env().emit_event(Transfer {
                        from: None,
                        to: Some(pools[pool]),
                        amount: this_pool.tokens,
                    });

                    // emit mint Approval event
                    Self::env().emit_event(Approval {
                        owner: Some(pools[pool]),
                        spender: Some(caller),
                        amount: this_pool.tokens,
                    });

                }

                // set initial data
                contract.monthspassed = 0;
                contract.nextpayout = Self::env().block_timestamp() as u128 + ONE_MONTH;
                contract.owner = caller;
                
                // reflect initial circulation
                // ...these may be inappropriate, as we may increment circulation
                // every time we pay a whitelister, or every time somebody buys tokens during
                // public sale...

                // whitelist
                contract.increment_circulation(POOL_TOKENS[10] * DECIMALS_POWER10);
                // public sale
                contract.increment_circulation(POOL_TOKENS[11] * DECIMALS_POWER10);

            })
        }

/////// modifiers ///////////////////////////////////////////////////////////

        /// . make sure caller is owner
        /// . returns true if caller is owner
        pub fn not_owner(
            &self,
        ) -> bool {
            self.env().caller() != self.owner
        }

        /// . make sure transfer amount is available
        /// . returns true if token holder has enough
        pub fn not_enough(
            &self,
            holder: AccountId,
            amount: Balance,
        ) -> bool {
            self.balances.get(holder).unwrap() < amount
        }

        /// . make sure allowance is sufficient
        /// . returns true if token spender has sufficient allowance
        pub fn not_allowed(
            &self,
            holder: AccountId,
            spender: AccountId,
            amount: Balance,
        ) -> bool {
            self.allowance(holder, spender) < amount
        }

        /// . make sure account is not zero account
        /// . returns true if not zero account
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

        /// . account balance getter
        #[ink(message)]
        pub fn balance_of(
            &self,
            account: AccountId,
        ) -> Balance {

            self.balances.get(account).unwrap_or(0)
        }

        /// . account allowance getter
        #[ink(message)]
        pub fn allowance(
            &self,
            owner: AccountId,
            spender: AccountId,
        ) -> Balance {

            self.allowances.get((owner, spender)).unwrap_or(0)
        }
        
/////// PSP22 doers /////////////////////////////////////////////////////////////

        /// . transfer method
        #[ink(message)]
        pub fn transfer(
            &mut self,
            recipient: AccountId,
            amount: Balance,
        ) -> ResultPSP22<()> {

            // get caller information
            let sender = self.env().caller();
            let sender_balance = self.balance_of(sender);

            // make sure balance is adequate
            if self.not_enough(sender, amount) {
                return Err(PSP22Error::InsufficientBalance)
            }

            // make sure no zero address
            if self.is_zero(recipient) {
                return Err(PSP22Error::ZeroRecipientAddress)
            }
            if self.is_zero(sender) {
                return Err(PSP22Error::ZeroSenderAddress)
            }

            // update balances
            let recipient_balance = self.balance_of(recipient);
            self.balances.insert(sender, &(sender_balance - amount));
            self.balances.insert(recipient, &(recipient_balance + amount));

            // emit Transfer event
            self.env().emit_event(Transfer {
                from: Some(sender),
                to: Some(recipient),
                amount: amount,
            });

            Ok(())
        }

        /// . approve method
        #[ink(message)]
        pub fn approve(
            &mut self,
            spender: AccountId,
            amount: Balance,
        ) -> ResultPSP22<()> {

            // get caller information
            let owner = self.env().caller();

            // make sure no zero address
            if self.is_zero(spender) {
                return Err(PSP22Error::ZeroRecipientAddress)
            }
            if self.is_zero(owner) {
                return Err(PSP22Error::ZeroSenderAddress)
            }

            // add/update approval amount
            self.allowances.insert((owner, spender), &amount);

            // emit Approval event
            self.env().emit_event(Approval {
                owner: Some(owner),
                spender: Some(spender),
                amount: amount,
            });

            Ok(())
        }

        /// . transfer from method
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: Balance,
        ) -> ResultPSP22<()> {

            // get caller information
            let caller = self.env().caller();

            // make sure balance is adequate
            if self.not_enough(from, amount) {
                return Err(PSP22Error::InsufficientBalance)
            }

            // make sure allowance is adequate
            if self.not_allowed(from, caller, amount) {
                return Err(PSP22Error::InsufficientAllowance)
            }

            // make sure no zero address
            if self.is_zero(from) {
                return Err(PSP22Error::ZeroSenderAddress)
            }
            if self.is_zero(to) {
                return Err(PSP22Error::ZeroRecipientAddress)
            }

            // get owner balance
            let from_balance = self.balance_of(from);

            // update balances
            self.balances.insert(from, &(from_balance - amount));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + amount));

            // update allowances
            let caller_allowance = self.allowance(from, caller);
            self.allowances.insert((from, caller), &(caller_allowance - amount));

            // emit Approval event
            self.env().emit_event(Approval {
                owner: Some(from),
                spender: Some(caller),
                amount: caller_allowance - amount,
            });

            // emit Transfer event
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                amount: amount,
            });

            Ok(())
        }

        /// . increase allowance method
        /// . this is to mitigate frontrunning
        #[ink(message)]
        pub fn increase_allowance(
            &mut self,
            spender: AccountId,
            delta: Balance,
        ) -> ResultPSP22<()> {

            // get caller information
            let owner = self.env().caller();

            // make sure no zero address
            if self.is_zero(spender) {
                return Err(PSP22Error::ZeroRecipientAddress)
            }
            if self.is_zero(owner) {
                return Err(PSP22Error::ZeroSenderAddress)
            }

            // get prior allowance
            let allowance: Balance = self.allowances.get((owner, spender)).unwrap();

            // add/update approval amount
            self.allowances.insert((owner, spender), &(allowance + delta));

            // emit Approval event
            self.env().emit_event(Approval {
                owner: Some(owner),
                spender: Some(spender),
                amount: allowance + delta,
            });

            Ok(())
        }
        
        /// . decrease allowance method
        /// . this is to mitigate frontrunning
        #[ink(message)]
        pub fn decrease_allowance(
            &mut self,
            spender: AccountId,
            delta: Balance,
        ) -> ResultPSP22<()> {

            // get caller information
            let owner = self.env().caller();

            // make sure no zero address
            if self.is_zero(spender) {
                return Err(PSP22Error::ZeroRecipientAddress)
            }
            if self.is_zero(owner) {
                return Err(PSP22Error::ZeroSenderAddress)
            }

            // make sure spender has enough allowance to decrease by delta
            if self.not_allowed(owner, spender, delta) {
                return Err(PSP22Error::InsufficientAllowance)
            }
            // if insufficient, should allowance go to zero instead of returning?

            // get prior allowance
            let allowance: Balance = self.allowances.get((owner, spender)).unwrap();

            // add/update approval amount
            self.allowances.insert((owner, spender), &(allowance - delta));

            // emit Approval event
            self.env().emit_event(Approval {
                owner: Some(owner),
                spender: Some(spender),
                amount: allowance - delta,
            });

            Ok(())
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
        ) -> ResultOther<()> {

            // make sure caller is owner
            if self.not_owner() {
                return Err(OtherError::CallerNotOwner)
            }

            // get data structs
            let mut this_stakeholder = self.stakeholderdata.get(stakeholder).unwrap();
            let this_pool = self.pooldata.get(self.pools[this_stakeholder.pool as usize]).unwrap();

            // require cliff to have been surpassed
            if self.monthspassed < this_pool.cliff {
                return Err(OtherError::CliffNotPassed)
            }

            // require share has not been completely paid out
            if this_stakeholder.paid == this_stakeholder.share {
                return Err(OtherError::StakeholderSharePaid)
            }

            // calculate the payout owed
            let mut payout: Balance = this_stakeholder.share / this_pool.vests as Balance;

            // if this is final payment, add token remainder to payout
            // (this is to compensate for floor division that calculates payamount)
            if this_stakeholder.share - this_stakeholder.paid - payout < this_stakeholder.share / this_pool.vests as Balance {

                // add remainder
                payout += this_stakeholder.share % this_pool.vests as Balance;
            }

            // now transfer tokens
            let balance_sender = self.balance_of(self.pools[this_stakeholder.pool as usize]);
            let balance_recipient = self.balance_of(stakeholder);
            self.balances.insert(self.pools[this_stakeholder.pool as usize], &(balance_sender - payout));
            self.balances.insert(stakeholder, &(balance_recipient + payout));

            // emit transfer event
            self.env().emit_event(Transfer {
                from: Some(self.pools[this_stakeholder.pool as usize]),
                to: Some(stakeholder),
                amount: payout,
            });

            // update circulating supply
            self.increment_circulation(payout);

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
            let this_pool = self.pooldata.get(self.pools[this_stakeholder.pool as usize]).unwrap();

            // how much has stakeholder already claimed?
            let paidout: Balance = this_stakeholder.paid;

            // how much does stakeholder have yet to collect?
            let payremaining: Balance = this_stakeholder.share - this_stakeholder.paid;

            // how much does stakeholder get each month?
            let payamount: Balance = this_stakeholder.share / this_pool.vests as Balance;

            return (
                paidout,
                payremaining,
                payamount,
                this_stakeholder.pool,
            )
        }

/////// pool data ////////////////////////////////////////////////////////////

        /// . function that returns pool data
        /// . this will allow observers to verify vesting parameters for each pool (esp. theirs)
        /// . observers may verify pool data from explorer if so motivated
        /// . pool numbers range from 0-11
        #[ink(message)]
        pub fn pool_data(
            &self,
            pool: u8,
        ) -> PoolData {
        
            // just grab up and send it out
            return self.pooldata.get(self.pools[pool as usize]).unwrap()
        }

//// misc  //////////////////////////////////////////////////////////////////////
        
        /// . function to get the number of months passed for contract
        #[ink(message)]
        pub fn months_passed(
            &self,
        ) -> u8 {

            self.monthspassed
        }

        /// . function to provide rewards pool address to ilockrewards contract
        #[ink(message)]
        pub fn rewards_pool(
            &self,
        ) -> AccountId {

            self.pools[7]
        }

        /// . function to increment circulatingsupply after reward issue or stakeholder payment
        #[ink(message)]
        pub fn increment_circulation(
            &mut self,
            amount: u128,
        ) -> bool {

            if self.not_owner() {
                return false
            }

            self.circulatingsupply += amount;
            true
        }

        /// . function to decrement circulatingsupply after burn or reward reclaim
        #[ink(message)]
        pub fn decrement_circulation(
            &mut self,
            amount: u128,
        ) -> bool {

            if self.not_owner() {
                return false
            }

            self.circulatingsupply -= amount;
            true
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
        ) -> ResultOther<()> {

            if self.not_owner() {
                return Err(OtherError::CallerNotOwner)
            }

            self.owner = newowner;

            Ok(())
        }

        /// . function to disown contract
        /// . upgrade and disown if Interlock Foundation goes under
        #[ink(message)]
        pub fn disown(
            &mut self,
        ) -> ResultOther<()> {

            if self.not_owner() {
                return Err(OtherError::CallerNotOwner)
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
        ) -> ResultOther<()> {

            // make sure owner is caller
            if self.not_owner() {
                return Err(OtherError::CallerNotOwner)
            }

            // burn the tokens
            let donor_balance: Balance = self.balances.get(donor).unwrap();
            self.balances.insert(donor, &(donor_balance - amount));

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
        ) -> ResultOther<()> {

            // make sure caller is owner
            if self.not_owner() {
                return Err(OtherError::CallerNotOwner)
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

//// To view debug prints and assertion failures run test via
//// cargo nightly+ test -- --nocapture

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        use ink_env::Clear;
        use ink_env::topics::PrefixedValue;
        use ink_lang::codegen::Env;

        type Event = <ILOCKtoken as ::ink_lang::reflect::ContractEventBase>::Type;



        /// test if the default constructor does its job
        #[ink::test]
        fn constructor_works() {

        let TEST_POOLS: [AccountId; POOL_COUNT] = [
                AccountId::from([0x11; ID_LENGTH]),
                AccountId::from([0x12; ID_LENGTH]),
                AccountId::from([0x13; ID_LENGTH]),
                AccountId::from([0x14; ID_LENGTH]),
                AccountId::from([0x15; ID_LENGTH]),
                AccountId::from([0x16; ID_LENGTH]),
                AccountId::from([0x17; ID_LENGTH]),
                AccountId::from([0x18; ID_LENGTH]),
                AccountId::from([0x19; ID_LENGTH]),
                AccountId::from([0x1a; ID_LENGTH]),
                AccountId::from([0x1b; ID_LENGTH]),
                AccountId::from([0x1c; ID_LENGTH]),
            ];

            let ILOCKtokenPSP22 = ILOCKtoken::new_token(TEST_POOLS);

            // check events
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(24, emitted_events.len());

            assert_eq!(ILOCKtokenPSP22.owner, ILOCKtokenPSP22.env().caller());
            assert_eq!(ILOCKtokenPSP22.balance_of(ILOCKtokenPSP22.env().account_id()), 0);

            let test_pool6: PoolData = ILOCKtokenPSP22.pooldata.get(TEST_POOLS[6]).unwrap();
            assert_eq!(test_pool6.name, "advisors");
            assert_eq!(test_pool6.tokens, 25_000_000 * DECIMALS_POWER10);
            assert_eq!(test_pool6.vests, 24);
            assert_eq!(test_pool6.cliff, 1);

            assert_eq!(ILOCKtokenPSP22.monthspassed, 0);
        }

        /// test if name getter does its job
        #[ink::test]
        fn name_works() {

        let TEST_POOLS: [AccountId; POOL_COUNT] = [
                AccountId::from([0x11; ID_LENGTH]),
                AccountId::from([0x12; ID_LENGTH]),
                AccountId::from([0x13; ID_LENGTH]),
                AccountId::from([0x14; ID_LENGTH]),
                AccountId::from([0x15; ID_LENGTH]),
                AccountId::from([0x16; ID_LENGTH]),
                AccountId::from([0x17; ID_LENGTH]),
                AccountId::from([0x18; ID_LENGTH]),
                AccountId::from([0x19; ID_LENGTH]),
                AccountId::from([0x1a; ID_LENGTH]),
                AccountId::from([0x1b; ID_LENGTH]),
                AccountId::from([0x1c; ID_LENGTH]),
            ];

            let ILOCKtokenPSP22 = ILOCKtoken::new_token(TEST_POOLS);
            assert_eq!(ILOCKtokenPSP22.name(), Some("Interlock Network".to_string()));
        }

        /// test if symbol getter does its job
        #[ink::test]
        fn symbol_works() {

        let TEST_POOLS: [AccountId; POOL_COUNT] = [
                AccountId::from([0x11; ID_LENGTH]),
                AccountId::from([0x12; ID_LENGTH]),
                AccountId::from([0x13; ID_LENGTH]),
                AccountId::from([0x14; ID_LENGTH]),
                AccountId::from([0x15; ID_LENGTH]),
                AccountId::from([0x16; ID_LENGTH]),
                AccountId::from([0x17; ID_LENGTH]),
                AccountId::from([0x18; ID_LENGTH]),
                AccountId::from([0x19; ID_LENGTH]),
                AccountId::from([0x1a; ID_LENGTH]),
                AccountId::from([0x1b; ID_LENGTH]),
                AccountId::from([0x1c; ID_LENGTH]),
            ];


            let ILOCKtokenPSP22 = ILOCKtoken::new_token(TEST_POOLS);
            assert_eq!(ILOCKtokenPSP22.symbol(), Some("ILOCK".to_string()));
        }
        
        /// test if decimals getter does its job
        #[ink::test]
        fn decimals_works() {

        let TEST_POOLS: [AccountId; POOL_COUNT] = [
                AccountId::from([0x11; ID_LENGTH]),
                AccountId::from([0x12; ID_LENGTH]),
                AccountId::from([0x13; ID_LENGTH]),
                AccountId::from([0x14; ID_LENGTH]),
                AccountId::from([0x15; ID_LENGTH]),
                AccountId::from([0x16; ID_LENGTH]),
                AccountId::from([0x17; ID_LENGTH]),
                AccountId::from([0x18; ID_LENGTH]),
                AccountId::from([0x19; ID_LENGTH]),
                AccountId::from([0x1a; ID_LENGTH]),
                AccountId::from([0x1b; ID_LENGTH]),
                AccountId::from([0x1c; ID_LENGTH]),
            ];


            let ILOCKtokenPSP22 = ILOCKtoken::new_token(TEST_POOLS);
            assert_eq!(ILOCKtokenPSP22.decimals(), 18);
        }

        /// test if total supply getter does its job
        #[ink::test]
        fn totalsupply_works() {

        let TEST_POOLS: [AccountId; POOL_COUNT] = [
                AccountId::from([0x11; ID_LENGTH]),
                AccountId::from([0x12; ID_LENGTH]),
                AccountId::from([0x13; ID_LENGTH]),
                AccountId::from([0x14; ID_LENGTH]),
                AccountId::from([0x15; ID_LENGTH]),
                AccountId::from([0x16; ID_LENGTH]),
                AccountId::from([0x17; ID_LENGTH]),
                AccountId::from([0x18; ID_LENGTH]),
                AccountId::from([0x19; ID_LENGTH]),
                AccountId::from([0x1a; ID_LENGTH]),
                AccountId::from([0x1b; ID_LENGTH]),
                AccountId::from([0x1c; ID_LENGTH]),
            ];


            let ILOCKtokenPSP22 = ILOCKtoken::new_token(TEST_POOLS);
            assert_eq!(ILOCKtokenPSP22.total_supply(), 65_000_000 * DECIMALS_POWER10);
        }

        /// test if balance getter does its job
        #[ink::test]
        fn balance_of_works() {

        let TEST_POOLS: [AccountId; POOL_COUNT] = [
                AccountId::from([0x11; ID_LENGTH]),
                AccountId::from([0x12; ID_LENGTH]),
                AccountId::from([0x13; ID_LENGTH]),
                AccountId::from([0x14; ID_LENGTH]),
                AccountId::from([0x15; ID_LENGTH]),
                AccountId::from([0x16; ID_LENGTH]),
                AccountId::from([0x17; ID_LENGTH]),
                AccountId::from([0x18; ID_LENGTH]),
                AccountId::from([0x19; ID_LENGTH]),
                AccountId::from([0x1a; ID_LENGTH]),
                AccountId::from([0x1b; ID_LENGTH]),
                AccountId::from([0x1c; ID_LENGTH]),
            ];


            // construct contract and initialize accounts
            let ILOCKtokenPSP22 = ILOCKtoken::new_token(TEST_POOLS);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice owns all the tokens on contract instantiation
            assert_eq!(ILOCKtokenPSP22.balance_of(ILOCKtokenPSP22.pools[6]), 25_000_000 * DECIMALS_POWER10);

            // Bob does not own tokens
            assert_eq!(ILOCKtokenPSP22.balance_of(accounts.bob), 0);
        }

        /// test if allowance getter does its job
        #[ink::test]
        fn allowance_works() {

        let TEST_POOLS: [AccountId; POOL_COUNT] = [
                AccountId::from([0x11; ID_LENGTH]),
                AccountId::from([0x12; ID_LENGTH]),
                AccountId::from([0x13; ID_LENGTH]),
                AccountId::from([0x14; ID_LENGTH]),
                AccountId::from([0x15; ID_LENGTH]),
                AccountId::from([0x16; ID_LENGTH]),
                AccountId::from([0x17; ID_LENGTH]),
                AccountId::from([0x18; ID_LENGTH]),
                AccountId::from([0x19; ID_LENGTH]),
                AccountId::from([0x1a; ID_LENGTH]),
                AccountId::from([0x1b; ID_LENGTH]),
                AccountId::from([0x1c; ID_LENGTH]),
            ];


            // construct contract and initialize accounts
            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token(TEST_POOLS);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice has not yet approved Bob
            assert_eq!(ILOCKtokenPSP22.allowance(accounts.alice, accounts.bob), 0);

            // Alice approves Bob for tokens
            assert_eq!(ILOCKtokenPSP22.approve(accounts.bob, 10), Ok(()));

            // Bob's new allowance reflects this approval
            assert_eq!(ILOCKtokenPSP22.allowance(accounts.alice, accounts.bob), 10);
        }

        /// test if the transfer doer does its job
        #[ink::test]
        fn transfer_works() {

        let TEST_POOLS: [AccountId; POOL_COUNT] = [
                AccountId::from([0x11; ID_LENGTH]),
                AccountId::from([0x12; ID_LENGTH]),
                AccountId::from([0x13; ID_LENGTH]),
                AccountId::from([0x14; ID_LENGTH]),
                AccountId::from([0x15; ID_LENGTH]),
                AccountId::from([0x16; ID_LENGTH]),
                AccountId::from([0x17; ID_LENGTH]),
                AccountId::from([0x18; ID_LENGTH]),
                AccountId::from([0x19; ID_LENGTH]),
                AccountId::from([0x1a; ID_LENGTH]),
                AccountId::from([0x1b; ID_LENGTH]),
                AccountId::from([0x1c; ID_LENGTH]),
            ];


            // construct contract and initialize accounts
            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token(TEST_POOLS);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // charge alice's account
            ILOCKtokenPSP22.balances.insert(accounts.alice, &100);

            // alice transfers tokens to bob
            assert_eq!(ILOCKtokenPSP22.transfer(accounts.bob, 10), Ok(()));

            // alice new balance
            assert_eq!(ILOCKtokenPSP22.balance_of(accounts.alice), 90);

            // Alice balance reflects transfer
            assert_eq!(ILOCKtokenPSP22.balance_of(accounts.alice), 90);

            // Bob balance reflects transfer
            assert_eq!(ILOCKtokenPSP22.balance_of(accounts.bob), 10);

            // Alice attempts transfer too large
            assert_eq!(ILOCKtokenPSP22.transfer(accounts.bob, SUPPLY_CAP), Err(PSP22Error::InsufficientBalance));

            // check all events that happened during the previous calls
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 25);

            // check the transfer event relating to the actual trasfer
            assert_transfer_event(
                &emitted_events[24],
                Some(AccountId::from([0x01; ID_LENGTH])),
                Some(AccountId::from([0x02; ID_LENGTH])),
                10,
            );
        }

        /// test if the approve does does its job
        #[ink::test]
        fn approve_works() {

        let TEST_POOLS: [AccountId; POOL_COUNT] = [
                AccountId::from([0x11; ID_LENGTH]),
                AccountId::from([0x12; ID_LENGTH]),
                AccountId::from([0x13; ID_LENGTH]),
                AccountId::from([0x14; ID_LENGTH]),
                AccountId::from([0x15; ID_LENGTH]),
                AccountId::from([0x16; ID_LENGTH]),
                AccountId::from([0x17; ID_LENGTH]),
                AccountId::from([0x18; ID_LENGTH]),
                AccountId::from([0x19; ID_LENGTH]),
                AccountId::from([0x1a; ID_LENGTH]),
                AccountId::from([0x1b; ID_LENGTH]),
                AccountId::from([0x1c; ID_LENGTH]),
            ];


            // construct contract and initialize accounts
            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token(TEST_POOLS);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice approves bob to spend tokens
            assert_eq!(ILOCKtokenPSP22.approve(accounts.bob, 10), Ok(()));

            // Bob is approved to spend tokens owned by Alice
            assert_eq!(ILOCKtokenPSP22.allowance(accounts.alice, accounts.bob), 10);

            // check all events that happened during previous calls
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 25);

            // check the approval event relating to the actual approval
            assert_approval_event(
                &emitted_events[24],
                Some(AccountId::from([0x01; ID_LENGTH])),
                Some(AccountId::from([0x02; ID_LENGTH])),
                10,
            );
        }

        /// test if the transfer-from doer does its job
        #[ink::test]
        fn transfer_from_works() {

        let TEST_POOLS: [AccountId; POOL_COUNT] = [
                AccountId::from([0x11; ID_LENGTH]),
                AccountId::from([0x12; ID_LENGTH]),
                AccountId::from([0x13; ID_LENGTH]),
                AccountId::from([0x14; ID_LENGTH]),
                AccountId::from([0x15; ID_LENGTH]),
                AccountId::from([0x16; ID_LENGTH]),
                AccountId::from([0x17; ID_LENGTH]),
                AccountId::from([0x18; ID_LENGTH]),
                AccountId::from([0x19; ID_LENGTH]),
                AccountId::from([0x1a; ID_LENGTH]),
                AccountId::from([0x1b; ID_LENGTH]),
                AccountId::from([0x1c; ID_LENGTH]),
            ];


            // construct contract and initialize accounts
            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token(TEST_POOLS);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // charge alice's account
            ILOCKtokenPSP22.balances.insert(accounts.alice, &100);

            // Alice approves Bob for token transfers on her behalf
            assert_eq!(ILOCKtokenPSP22.approve(accounts.bob, 10), Ok(()));

            // set the contract as callee and Bob as caller
            let contract = ink_env::account_id::<ink_env::DefaultEnvironment>();
            ink_env::test::set_callee::<ink_env::DefaultEnvironment>(contract);
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.bob);

            // Check Bob's allowance
            assert_eq!(ILOCKtokenPSP22.allowance(accounts.alice, accounts.bob), 10);

            assert_eq!(ILOCKtokenPSP22.env().caller(), accounts.bob);

            // Bob transfers tokens from Alice to Eve
            assert_eq!(ILOCKtokenPSP22.transfer_from(accounts.alice, accounts.eve, 10), Ok(()));

            // Eve received the tokens
            assert_eq!(ILOCKtokenPSP22.balance_of(accounts.eve), 10);

            // Bob attempts a transferfrom too large
            assert_eq!(ILOCKtokenPSP22.transfer_from(accounts.alice, accounts.eve, 100),
                        Err(PSP22Error::InsufficientBalance));

            // check all events that happened during the previous callsd
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 27);

            // check that Transfer event was emitted        
            assert_transfer_event(
                &emitted_events[26],
                Some(AccountId::from([0x01; ID_LENGTH])),
                Some(AccountId::from([0x05; ID_LENGTH])),
                10,
            );
        }

        /// test if wallet registration function works as intended 
        #[ink::test]
        fn register_stakeholder_works() {

        let TEST_POOLS: [AccountId; POOL_COUNT] = [
                AccountId::from([0x11; ID_LENGTH]),
                AccountId::from([0x12; ID_LENGTH]),
                AccountId::from([0x13; ID_LENGTH]),
                AccountId::from([0x14; ID_LENGTH]),
                AccountId::from([0x15; ID_LENGTH]),
                AccountId::from([0x16; ID_LENGTH]),
                AccountId::from([0x17; ID_LENGTH]),
                AccountId::from([0x18; ID_LENGTH]),
                AccountId::from([0x19; ID_LENGTH]),
                AccountId::from([0x1a; ID_LENGTH]),
                AccountId::from([0x1b; ID_LENGTH]),
                AccountId::from([0x1c; ID_LENGTH]),
            ];


            // construct contract and initialize accounts
            let mut ILOCKtokenPSP22 = ILOCKtoken::new_token(TEST_POOLS);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // bob's stakeholder data
            let share: u128 = 1_000_000;
            let pool: u8 = 3;

            // call registration function
            ILOCKtokenPSP22.register_stakeholder(accounts.bob, share, pool).unwrap();

            // verify registration stuck
            let this_stakeholder = ILOCKtokenPSP22.stakeholderdata.get(accounts.bob).unwrap();
            assert_eq!(this_stakeholder.paid, 0);
            assert_eq!(this_stakeholder.share, share);
            assert_eq!(this_stakeholder.pool, pool);

        }
       
/////// testing helpers  //////////////////////////////////////////////////////////////////////

        /// check that a transfer event is good
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

        /// check that an approval event is good
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

        /// this is a painful hashing function for use in event assert functions
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
