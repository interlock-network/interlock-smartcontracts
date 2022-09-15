/////////////////////////////////////////////////////////////////
//
// INTERLOCK NETWORK
//
// blairmunroakusa@0903Fri.09Sep22.anch.AK:br
//
// !!!!! INCOMPLETE AND FLAWED, WARNING !!!!!
//
// NOTE: To enable unsigned integer division, overflow_checks
// has been turned 'off' in Cargo.toml file.
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

//// state /////////////////////////////////////////////////////////////

    /// PoolData struct contains all pertinant information about the various token pools
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct PoolData {
        name: String,
        tokens: u128,
        vests: u128,
        cliff: u128,
    }

    /// MemberData struct contains all pertinent information for each member
    /// (Besides balance and allowance mappings)
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct MemberData {
        owes: u128,
        paid: u128,
        share: u128,
        pool: u128,
        payouts: u128,
    }

    /// ILOCKtoken struct contains overall storage data for contract
    #[derive(SpreadAllocate)]
    #[ink(storage)]
    pub struct ILOCKtoken {
        owner: AccountId,
        name: String,
        symbol: String,
        decimals: u8,
        decimalfactor: u128,
        totalsupply: u128,
        balances: Mapping<AccountId, u128>,
        allowances: Mapping<(AccountId, AccountId), u128>,
        memberdata: Mapping<AccountId, MemberData>,
        pooldata: Mapping<AccountId, PoolData>,
        pools: [AccountId; 12], // this is pattern to iterate through pooldata mapping
        poolcount: u8,
        monthspassed: u128,
        nextpayout: u128,
        onemonth: u128,
        TGEtriggered: bool,
    }


//// ERC20 events /////////////////////////////////////////////////////////////

    /// specify transfer event
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        amount: u128,
    }

    /// specify approve event
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        spender: Option<AccountId>,
        amount: u128,
    }

    impl ILOCKtoken {

/////// init /////////////////////////////////////////////////////////////

        /// constructor that initializes contract
        /// and simultaneously creates TGE
        /// (splitSupply() and triggerTGE())
        #[ink(constructor)]
        // takes in array of pool addresses generated earlier, pre token contract constructor
        pub fn new_token(
            pools: [AccountId; 12],
        ) -> Self {

            // create contract
            initialize_contract(|contract: &mut Self| {

                // define supply and decimals
                let token_total: u128 = 1_000_000_000;
                let decimal_total: u128 = 1_000_000_000_000_000_000;
                let supply: u128 = token_total * decimal_total;

                // define owner as caller
                let caller = Self::env().caller();
                contract.owner = caller;
                contract.totalsupply = supply;
                contract.decimalfactor = decimal_total;
                contract.balances.insert(Self::env().account_id(), &supply);
                contract.allowances.insert((Self::env().account_id(), Self::env().caller()), &supply);

                // emit mint Transfer event
                Self::env().emit_event(Transfer {
                    from: None,
                    to: Some(Self::env().account_id()),
                    amount: supply,
                });

                // emit mint Approval event
                Self::env().emit_event(Approval {
                    owner: Some(Self::env().account_id()),
                    spender: Some(Self::env().caller()),
                    amount: supply,
                });


                // pool data
                const POOLCOUNT: usize = 12;
                let pool_names: [&str; POOLCOUNT] = [
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
                let pool_tokens: [u128; POOLCOUNT] = [
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
                let pool_vests: [u128; POOLCOUNT] = [
                    24,
                    18,
                    15,
                    12,
                    36,
                    24,
                    24,
                    48,   // This is rewards pool. Should #vests be 1 instead? Probably for consistency's sake.
                    84,
                    1,
                    48,
                    1,
                ];
                let pool_cliffs: [u128; POOLCOUNT] = [
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

                // assign pool data
                for pool in 0..POOLCOUNT {
                    contract.pools[pool] = pools[pool];

                    // define pooldata struct for this pool
                    let this_pool = PoolData {
                        name: pool_names[pool].to_string(),
                        tokens: pool_tokens[pool] * decimal_total,
                        vests: pool_vests[pool],
                        cliff: pool_cliffs[pool],
                    };

                    // push current pool into pooldata map
                    contract.pooldata.insert(pools[pool], &this_pool);
                }

                // set metadata
                let month: u128 = 2592000; // derived from two unix timestamps 30 days apart
                contract.name = "Interlock Network".to_string();
                contract.symbol = "ILOCK".to_string();
                contract.decimals = 18;
                contract.poolcount = POOLCOUNT as u8;
                contract.monthspassed = 0;
                contract.onemonth = month;
                contract.nextpayout = Self::env().block_timestamp() as u128 + month;
                contract.TGEtriggered = false;

            })
        }

/////// distributing /////////////////////////////////////////////////////////////

        /// function to distribute tokens to respective pools
        #[ink(message)]
        pub fn distribute_pools(
            &mut self,
        ) -> bool {

            // this must only happen once
            if self.TGEtriggered {
                ink_env::debug_println!("TGE already triggered");
                return false
            }

            // iterate through all pools
            for pool in 0..self.poolcount as usize {

                // get pooldata struct for given pool
                let this_pool = self.pooldata.get(self.pools[pool]).unwrap();
                    
                // transfer tokens to pool
                self.transfer_from(
                    self.env().account_id(),
                    self.pools[pool],
                    this_pool.tokens,
                );

                // adjust owner's allowance
                self.allowances.insert(
                    (self.pools[pool], self.env().caller()), 
                    &this_pool.tokens,
                );

                // emit mint Approval event
                Self::env().emit_event(Approval {
                    owner: Some(self.pools[pool]),
                    spender: Some(self.env().caller()),
                    amount: this_pool.tokens,
                });
            }

            // TGE is now complete
            // this must only happen once
            self.TGEtriggered = true;
        
            true
        }

/////// timing /////////////////////////////////////////////////////////////

        /// function to check if enough time has passed to push next payout
        #[ink(message)]
        pub fn check_time(
            &mut self,
        ) -> bool {

            // test to see if current time falls beyond time for next payout
            if self.env().block_timestamp() as u128 > self.nextpayout {

                // update time variables
                self.nextpayout += self.onemonth;
                self.monthspassed += 1;

                return true
            }

            // too early
            false
        }

/////// registration  /////////////////////////////////////////////////////////////

        /// function that registers a member's wallet and vesting info
        #[ink(message)]
        pub fn register_member(
            &mut self,
            member: AccountId,
            owes: u128,
            share: u128,
            pool: u128,
        ) -> bool {

            // members must be added before TGE
            if self.TGEtriggered {
                ink_env::debug_println!("member data may only be added prior to TGE. TGE happened.");
                return false
            }

            // create member struct
            let this_member = MemberData {
                owes: owes,
                paid: 0,
                share: share,
                pool: pool,
                payouts: 0,
            };

            // insert member struct into mapping
            self.memberdata.insert(member, &this_member);

            true
        }


/////// claiming /////////////////////////////////////////////////////////////

        /// function for user to claim the token share they are currently entitled to
        #[ink(message)]
        pub fn claim_tokens(
            &mut self,
            claimant: AccountId,
        ) -> bool {

            // make sure TGE is complete
            if !self.TGEtriggered {
                ink_env::debug_println!("TGE has not yet occured");
                return false
            }

            // get data structs
            let mut this_member = self.memberdata.get(claimant).unwrap();
            let this_pool = self.pooldata.get(self.pools[this_member.pool as usize]).unwrap();

            // require share has not been completely paid out
            if this_member.paid == this_member.share {
                ink_env::debug_println!("claimant has already been paid out completely");
                return false
            }

            // require if investor, to pay due first
            if this_member.owes > 0 {
                ink_env::debug_println!("claimant is investor who still needs to pay dues");
                return false
            }

            // require number of payouts not to exceed number of vests
            if this_member.payouts >= this_pool.vests {
                ink_env::debug_println!("too many payouts, claimant has already been paid out");
                return false
            }

            // now make sure monthspassed is up to date
            self.check_time();

            // require cliff to have been surpassed
            if self.monthspassed < this_pool.cliff {
                ink_env::debug_println!("too soon, the cliff has not yet been surpassed");
                return false
            }

            // determine the number of payments claimant is entitled to
            let payments: u128;
            if this_pool.cliff + this_pool.vests <= self.monthspassed {
            // for first case, claimant waited until all payments are available

                // payments owed are payments remaining 
                payments = this_pool.vests - this_member.payouts;

            } else if self.monthspassed >= this_pool.cliff {
            // for second case, claimant did not wait and is claiming only a portion of payments
                
                // factor of one to line everything up right
                payments = 1 + self.monthspassed - this_member.payouts - this_pool.cliff;

            } else {
                payments = 0;
            }

            // now calculate the payout owed
            let mut payout: u128 = (this_member.share / this_pool.vests)*payments;

            // if this is final payment, add token remainder to payout
            if this_member.share - this_member.paid - payout < this_member.share/this_pool.vests {

                // add remainder
                payout += this_member.share % this_pool.vests;
            }

            // now transfer tokens
            let balance_sender = self.balance_of(self.pools[this_member.pool as usize]);
            let balance_recipient = self.balance_of(claimant);
            self.balances.insert(self.pools[this_member.pool as usize], &(balance_sender - payout));
            self.balances.insert(claimant, &(balance_recipient + payout));

            // emit transfer event
            Self::env().emit_event(Transfer {
                from: Some(self.pools[this_member.pool as usize]),
                to: Some(claimant),
                amount: payout,
            });

            // finally update member data struct state
            this_member.payouts += payments;
            this_member.paid += payout;
            self.memberdata.insert(claimant, &this_member);

            true
        }

/////// vesting ////////////////////////////////////////////////////////////

        /// function that returns a member's vesting status
        #[ink(message)]
        pub fn vesting_status(
            &self,
            vestee: AccountId,
        ) -> (u128, u128, u128, u128, u128) {

            // get pool and member data structs first
            let this_member = self.memberdata.get(vestee).unwrap();
            let this_pool = self.pooldata.get(self.pools[this_member.pool as usize]).unwrap();

            // first we need to compute how long until the next payout is available
            let timeleft: u128;
            if self.monthspassed >= this_pool.vests + this_pool.cliff {
            // what happens when time surpasses vests and cliff? vv

                // it's time for everything to payout, no more timer
                timeleft = 0;

            } else if self.monthspassed < this_pool.cliff {
            // what happens if cliff hasn't been surpassed yet?

                // timeleft is time til next month plus time left on cliff
                timeleft = (this_pool.cliff - self.monthspassed - 1) * self.onemonth +
                    self.nextpayout - self.env().block_timestamp() as u128;

            } else {
            // during vesting period, time left is always time until nextpayout
                
                timeleft = self.nextpayout - self.env().block_timestamp() as u128;
            }

            // how much does investor still owe in dues?
            let stillowes: u128 = this_member.owes;

            // how much has member already claimed?
            let paidout: u128 = this_member.paid;

            // how much does member have yet to collect?
            let payremaining: u128 = this_member.share - this_member.paid;

            // now compute the tokens available to claim at given moment
            let payavailable: u128;
            if self.monthspassed >= this_pool.cliff &&
                self.monthspassed < this_pool.cliff + this_pool.vests {
            // if months passed are inbetween cliff and end of vesting period
                
                payavailable = (1 + self.monthspassed - this_pool.cliff - this_member.payouts) *
                    (this_member.share / this_pool.vests);

            } else if self.monthspassed < this_pool.cliff {
            // until time passes cliff, no pay is available

                payavailable = 0;

            } else {
            // after passing cliff and vest, the rest of share is available

                payavailable = this_member.share - this_member.paid;
            }

            return (
                timeleft,
                stillowes,
                paidout,
                payremaining,
                payavailable,
            )
        }

/////// ERC20 getters ///////////////////////////////////////////////////////////

        /// token decimal count getter
        #[ink(message)]
        pub fn name(
            &self,
        ) -> String {

            self.name.clone()
        }

        /// token decimal count getter
        #[ink(message)]
        pub fn symbol(
            &self,
        ) -> String {

            self.symbol.clone()
        }

        /// token decimal count getter
        #[ink(message)]
        pub fn decimals(
            &self,
        ) -> u8 {

            self.decimals
        }

        /// total supply getter
        #[ink(message)]
        pub fn total_supply(
            &self,
        ) -> u128 {

            self.totalsupply
        }

        /// account balance getter
        #[ink(message)]
        pub fn balance_of(
            &self,
            account: AccountId,
        ) -> u128 {

            match self.balances.get(account) {
                Some(value) => value,
                None => 0,
            }
        }

        /// account allowance getter
        #[ink(message)]
        pub fn allowance(
            &self,
            owner: AccountId,
            spender: AccountId,
        ) -> u128 {

            match self.allowances.get((owner, spender)) {
                Some(value) => value,
                None => 0,
            }
        }
        
/////// ERC20 doers /////////////////////////////////////////////////////////////

        /// transfer method
        #[ink(message)]
        pub fn transfer(
            &mut self,
            recipient: AccountId,
            amount: u128,
        ) -> bool {

            // get caller information
            let sender = self.env().caller();
            let sender_balance = self.balance_of(sender);

            // make sure balance is adequate
            if sender_balance < amount {
                ink_env::debug_println!("Insufficient balance");
                return false
            }

            // update balances
            let recipient_balance = self.balance_of(recipient);
            self.balances.insert(sender, &(sender_balance - amount));
            self.balances.insert(recipient, &(recipient_balance + amount));

            // emit Transfer event
            Self::env().emit_event(Transfer {
                from: Some(sender),
                to: Some(recipient),
                amount: amount,
            });

            true
        }

        /// approve method
        #[ink(message)]
        pub fn approve(
            &mut self,
            spender: AccountId,
            amount: u128,
        ) -> bool {

            // get caller information
            let owner = self.env().caller();

            // add/update approval amount
            self.allowances.insert((owner, spender), &amount);

            // emit Approval event
            self.env().emit_event(Approval {
                owner: Some(owner),
                spender: Some(spender),
                amount: amount,
            });

            true
        }

        /// transfer from method
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: u128,
        ) -> bool {

            // get owner balance
            let from_balance = self.balance_of(from);

            // make sure balance is adequate
            if from_balance < amount {
                ink_env::debug_println!("Insufficient balance");
                return false
            }

            // update balances
            self.balances.insert(from, &(from_balance - amount));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + amount));

            // update allowances
            let caller = self.env().caller();
            let caller_allowance = self.allowance(from, caller);
            self.allowances.insert((from, caller), &(caller_allowance - amount));

            // emit Approval event
            self.env().emit_event(Approval {
                owner: Some(from),
                spender: Some(caller),
                amount: amount,
            });

            // emit Transfer event
            Self::env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                amount: amount,
            });

            true
        }

    

//// misc  //////////////////////////////////////////////////////////////////////
        
        /// function to provide rewards pool address to ilockrewards contract
        #[ink(message)]
        pub fn rewards_pool(&self) -> AccountId {
            self.pools[7]
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

        pub const DECIMALS_TOTAL: u128 = 1_000_000_000_000_000_000;
        pub const SUPPLY: u128 = 1_000_000_000;

        /// test if the default constructor does its job
        #[ink::test]
        fn constructor_works() {

            let pool_accounts: [AccountId; 12] = [
                AccountId::from([0x11; 32]),
                AccountId::from([0x12; 32]),
                AccountId::from([0x13; 32]),
                AccountId::from([0x14; 32]),
                AccountId::from([0x15; 32]),
                AccountId::from([0x16; 32]),
                AccountId::from([0x17; 32]),
                AccountId::from([0x18; 32]),
                AccountId::from([0x19; 32]),
                AccountId::from([0x1a; 32]),
                AccountId::from([0x1b; 32]),
                AccountId::from([0x1c; 32]),
            ];

            let ILOCKtokenERC20 = ILOCKtoken::new_token(pool_accounts);

            // check events
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(2, emitted_events.len());
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(ILOCKtokenERC20.env().account_id()),
                SUPPLY * DECIMALS_TOTAL,
            );
            assert_approval_event(
                &emitted_events[1],
                Some(ILOCKtokenERC20.env().account_id()),
                Some(ILOCKtokenERC20.owner),
                SUPPLY * DECIMALS_TOTAL,
            );

            assert_eq!(ILOCKtokenERC20.owner, ILOCKtokenERC20.env().caller());
            assert_eq!(ILOCKtokenERC20.name, "Interlock Network");
            assert_eq!(ILOCKtokenERC20.symbol, "ILOCK");
            assert_eq!(ILOCKtokenERC20.decimals, 18);
            assert_eq!(ILOCKtokenERC20.decimalfactor, DECIMALS_TOTAL);
            assert_eq!(ILOCKtokenERC20.totalsupply, SUPPLY * DECIMALS_TOTAL);
            assert_eq!(ILOCKtokenERC20.balance_of(ILOCKtokenERC20.env().account_id()), SUPPLY * DECIMALS_TOTAL);

            let test_pool6: PoolData = ILOCKtokenERC20.pooldata.get(pool_accounts[6]).unwrap();
            assert_eq!(test_pool6.name, "advisors");
            assert_eq!(test_pool6.tokens, 25_000_000 * DECIMALS_TOTAL);
            assert_eq!(test_pool6.vests, 24);
            assert_eq!(test_pool6.cliff, 1);

            assert_eq!(ILOCKtokenERC20.poolcount, 12);
            assert_eq!(ILOCKtokenERC20.monthspassed, 0);
            assert_eq!(ILOCKtokenERC20.onemonth, 2_592_000);
            assert_eq!(ILOCKtokenERC20.TGEtriggered, false);

        }

        /// test if name getter does its job
        #[ink::test]
        fn name_works() {
            let pool_accounts: [AccountId; 12] = [
                AccountId::from([0x11; 32]),
                AccountId::from([0x12; 32]),
                AccountId::from([0x13; 32]),
                AccountId::from([0x14; 32]),
                AccountId::from([0x15; 32]),
                AccountId::from([0x16; 32]),
                AccountId::from([0x17; 32]),
                AccountId::from([0x18; 32]),
                AccountId::from([0x19; 32]),
                AccountId::from([0x1a; 32]),
                AccountId::from([0x1b; 32]),
                AccountId::from([0x1c; 32]),
            ];
            let ILOCKtokenERC20 = ILOCKtoken::new_token(pool_accounts);
            assert_eq!(ILOCKtokenERC20.name(), "Interlock Network");
        }

        /// test if symbol getter does its job
        #[ink::test]
        fn symbol_works() {
            let pool_accounts: [AccountId; 12] = [
                AccountId::from([0x11; 32]),
                AccountId::from([0x12; 32]),
                AccountId::from([0x13; 32]),
                AccountId::from([0x14; 32]),
                AccountId::from([0x15; 32]),
                AccountId::from([0x16; 32]),
                AccountId::from([0x17; 32]),
                AccountId::from([0x18; 32]),
                AccountId::from([0x19; 32]),
                AccountId::from([0x1a; 32]),
                AccountId::from([0x1b; 32]),
                AccountId::from([0x1c; 32]),
            ];
            let ILOCKtokenERC20 = ILOCKtoken::new_token(pool_accounts);
            assert_eq!(ILOCKtokenERC20.symbol(), "ILOCK");
        }
        
        /// test if decimals getter does its job
        #[ink::test]
        fn decimals_works() {
            let pool_accounts: [AccountId; 12] = [
                AccountId::from([0x11; 32]),
                AccountId::from([0x12; 32]),
                AccountId::from([0x13; 32]),
                AccountId::from([0x14; 32]),
                AccountId::from([0x15; 32]),
                AccountId::from([0x16; 32]),
                AccountId::from([0x17; 32]),
                AccountId::from([0x18; 32]),
                AccountId::from([0x19; 32]),
                AccountId::from([0x1a; 32]),
                AccountId::from([0x1b; 32]),
                AccountId::from([0x1c; 32]),
            ];
            let ILOCKtokenERC20 = ILOCKtoken::new_token(pool_accounts);
            assert_eq!(ILOCKtokenERC20.decimals(), 18);
        }

        /// test if total supply getter does its job
        #[ink::test]
        fn totalsupply_works() {
            let pool_accounts: [AccountId; 12] = [
                AccountId::from([0x11; 32]),
                AccountId::from([0x12; 32]),
                AccountId::from([0x13; 32]),
                AccountId::from([0x14; 32]),
                AccountId::from([0x15; 32]),
                AccountId::from([0x16; 32]),
                AccountId::from([0x17; 32]),
                AccountId::from([0x18; 32]),
                AccountId::from([0x19; 32]),
                AccountId::from([0x1a; 32]),
                AccountId::from([0x1b; 32]),
                AccountId::from([0x1c; 32]),
            ];
            let ILOCKtokenERC20 = ILOCKtoken::new_token(pool_accounts);
            assert_eq!(ILOCKtokenERC20.total_supply(), SUPPLY * DECIMALS_TOTAL);
        }

        /// test if balance getter does its job
        #[ink::test]
        fn balance_of_works() {
            let pool_accounts: [AccountId; 12] = [
                AccountId::from([0x11; 32]),
                AccountId::from([0x12; 32]),
                AccountId::from([0x13; 32]),
                AccountId::from([0x14; 32]),
                AccountId::from([0x15; 32]),
                AccountId::from([0x16; 32]),
                AccountId::from([0x17; 32]),
                AccountId::from([0x18; 32]),
                AccountId::from([0x19; 32]),
                AccountId::from([0x1a; 32]),
                AccountId::from([0x1b; 32]),
                AccountId::from([0x1c; 32]),
            ];

            // construct contract and initialize accounts
            let ILOCKtokenERC20 = ILOCKtoken::new_token(pool_accounts);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice owns all the tokens on contract instantiation
            assert_eq!(ILOCKtokenERC20.balance_of(accounts.alice), SUPPLY * DECIMALS_TOTAL);

            // Bob does not own tokens
            assert_eq!(ILOCKtokenERC20.balance_of(accounts.bob), 0);
        }

        /// test if allowance getter does its job
        #[ink::test]
        fn allowance_works() {
            let pool_accounts: [AccountId; 12] = [
                AccountId::from([0x11; 32]),
                AccountId::from([0x12; 32]),
                AccountId::from([0x13; 32]),
                AccountId::from([0x14; 32]),
                AccountId::from([0x15; 32]),
                AccountId::from([0x16; 32]),
                AccountId::from([0x17; 32]),
                AccountId::from([0x18; 32]),
                AccountId::from([0x19; 32]),
                AccountId::from([0x1a; 32]),
                AccountId::from([0x1b; 32]),
                AccountId::from([0x1c; 32]),
            ];

            // construct contract and initialize accounts
            let mut ILOCKtokenERC20 = ILOCKtoken::new_token(pool_accounts);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice has not yet approved Bob
            assert_eq!(ILOCKtokenERC20.allowance(accounts.alice, accounts.bob), 0);

            // Alice approves Bob for tokens
            assert_eq!(ILOCKtokenERC20.approve(accounts.bob, 10), true);

            // Bob's new allowance reflects this approval
            assert_eq!(ILOCKtokenERC20.allowance(accounts.alice, accounts.bob), 10);
        }

        /// test if the transfer doer does its job
        #[ink::test]
        fn transfer_works() {
            let pool_accounts: [AccountId; 12] = [
                AccountId::from([0x11; 32]),
                AccountId::from([0x12; 32]),
                AccountId::from([0x13; 32]),
                AccountId::from([0x14; 32]),
                AccountId::from([0x15; 32]),
                AccountId::from([0x16; 32]),
                AccountId::from([0x17; 32]),
                AccountId::from([0x18; 32]),
                AccountId::from([0x19; 32]),
                AccountId::from([0x1a; 32]),
                AccountId::from([0x1b; 32]),
                AccountId::from([0x1c; 32]),
            ];

            // construct contract and initialize accounts
            let mut ILOCKtokenERC20 = ILOCKtoken::new_token(pool_accounts);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice transfers tokens to Bob
            assert_eq!(ILOCKtokenERC20.transfer(accounts.bob, 10), true);

            // Alice balance reflects transfer
            assert_eq!(ILOCKtokenERC20.balance_of(accounts.alice), SUPPLY * DECIMALS_TOTAL - 10);

            // Bob balance reflects transfer
            assert_eq!(ILOCKtokenERC20.balance_of(accounts.bob), 10);

            // Alice attempts transfer too large
            assert_eq!(ILOCKtokenERC20.transfer(accounts.bob, SUPPLY * DECIMALS_TOTAL), false);

            // check all events that happened during the previous calls
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);

            // check the transfer event relating to the actual trasfer
            assert_transfer_event(
                &emitted_events[2],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        /// test if the approve does does its job
        #[ink::test]
        fn approve_works() {
            let pool_accounts: [AccountId; 12] = [
                AccountId::from([0x11; 32]),
                AccountId::from([0x12; 32]),
                AccountId::from([0x13; 32]),
                AccountId::from([0x14; 32]),
                AccountId::from([0x15; 32]),
                AccountId::from([0x16; 32]),
                AccountId::from([0x17; 32]),
                AccountId::from([0x18; 32]),
                AccountId::from([0x19; 32]),
                AccountId::from([0x1a; 32]),
                AccountId::from([0x1b; 32]),
                AccountId::from([0x1c; 32]),
            ];

            // construct contract and initialize accounts
            let mut ILOCKtokenERC20 = ILOCKtoken::new_token(pool_accounts);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice approves bob to spend tokens
            assert_eq!(ILOCKtokenERC20.approve(accounts.bob, 10), true);

            // Bob is approved to spend tokens owned by Alice
            assert_eq!(ILOCKtokenERC20.allowance(accounts.alice, accounts.bob), 10);

            // check all events that happened during previous calls
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);

            // check the approval event relating to the actual approval
            assert_approval_event(
                &emitted_events[2],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        /// test if the transfer-from doer does its job
        #[ink::test]
        fn transfer_from_works() {
            let pool_accounts: [AccountId; 12] = [
                AccountId::from([0x11; 32]),
                AccountId::from([0x12; 32]),
                AccountId::from([0x13; 32]),
                AccountId::from([0x14; 32]),
                AccountId::from([0x15; 32]),
                AccountId::from([0x16; 32]),
                AccountId::from([0x17; 32]),
                AccountId::from([0x18; 32]),
                AccountId::from([0x19; 32]),
                AccountId::from([0x1a; 32]),
                AccountId::from([0x1b; 32]),
                AccountId::from([0x1c; 32]),
            ];

            // construct contract and initialize accounts
            let mut ILOCKtokenERC20 = ILOCKtoken::new_token(pool_accounts);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice approves Bob for token transfers on her behalf
            assert_eq!(ILOCKtokenERC20.approve(accounts.bob, 10), true);

            // set the contract as callee and Bob as caller
            let contract = ink_env::account_id::<ink_env::DefaultEnvironment>();
            ink_env::test::set_callee::<ink_env::DefaultEnvironment>(contract);
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.bob);

            // Bob transfers tokens from Alice to Eve
            assert_eq!(ILOCKtokenERC20.transfer_from(accounts.alice, accounts.eve, 10), true);

            // Eve received the tokens
            assert_eq!(ILOCKtokenERC20.balance_of(accounts.eve), 10);

            // Bob attempts a transferfrom too large
            assert_eq!(ILOCKtokenERC20.transfer_from(accounts.alice, accounts.eve, SUPPLY * DECIMALS_TOTAL), false);

            // check all events that happened during the previous callsd
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 5);

            // check that Transfer event was emitted        
            assert_transfer_event(
                &emitted_events[4],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x05; 32])),
                10,
            );
        }

        /// test if check_time function does what it's supposed to
        #[ink::test]
        fn distribute_pools_works() {
            let pool_accounts: [AccountId; 12] = [
                AccountId::from([0x11; 32]),
                AccountId::from([0x12; 32]),
                AccountId::from([0x13; 32]),
                AccountId::from([0x14; 32]),
                AccountId::from([0x15; 32]),
                AccountId::from([0x16; 32]),
                AccountId::from([0x17; 32]),
                AccountId::from([0x18; 32]),
                AccountId::from([0x19; 32]),
                AccountId::from([0x1a; 32]),
                AccountId::from([0x1b; 32]),
                AccountId::from([0x1c; 32]),
            ];

            // construct contract and initialize accounts
            let mut ILOCKtokenERC20 = ILOCKtoken::new_token(pool_accounts);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            ILOCKtokenERC20.distribute_pools();

            for pool in 0..12 {

                let this_pool: PoolData = ILOCKtokenERC20.pooldata.get(pool_accounts[pool]).unwrap();

                assert_eq!(ILOCKtokenERC20
                    .balance_of(
                        pool_accounts[pool]),
                        this_pool.tokens,
                );
                assert_eq!(ILOCKtokenERC20
                    .allowance(
                        pool_accounts[pool], accounts.alice),
                        this_pool.tokens,
                );
            }
        }

        /// test if wallet registration function works as intended 
        #[ink::test]
        fn register_member_works() {
            let pool_accounts: [AccountId; 12] = [
                AccountId::from([0x11; 32]),
                AccountId::from([0x12; 32]),
                AccountId::from([0x13; 32]),
                AccountId::from([0x14; 32]),
                AccountId::from([0x15; 32]),
                AccountId::from([0x16; 32]),
                AccountId::from([0x17; 32]),
                AccountId::from([0x18; 32]),
                AccountId::from([0x19; 32]),
                AccountId::from([0x1a; 32]),
                AccountId::from([0x1b; 32]),
                AccountId::from([0x1c; 32]),
            ];

            // construct contract and initialize accounts
            let mut ILOCKtokenERC20 = ILOCKtoken::new_token(pool_accounts);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // bob's member data
            let owes: u128 = 14_000;
            let share: u128 = 1_000_000;
            let pool: u128 = 3;

            // call registration function
            ILOCKtokenERC20.register_member(accounts.bob, owes, share, pool);

            // verify registration stuck
            let this_member = ILOCKtokenERC20.memberdata.get(accounts.bob).unwrap();
            assert_eq!(this_member.owes, owes);
            assert_eq!(this_member.paid, 0);
            assert_eq!(this_member.share, share);
            assert_eq!(this_member.pool, pool);
            assert_eq!(this_member.payouts, 0);

        }
        
        /// test if claim_tokens function does what it's supposed to
        #[ink::test]
        fn claim_tokens_works() {
            let pool_accounts: [AccountId; 12] = [
                AccountId::from([0x11; 32]),
                AccountId::from([0x12; 32]),
                AccountId::from([0x13; 32]),
                AccountId::from([0x14; 32]),
                AccountId::from([0x15; 32]),
                AccountId::from([0x16; 32]),
                AccountId::from([0x17; 32]),
                AccountId::from([0x18; 32]),
                AccountId::from([0x19; 32]),
                AccountId::from([0x1a; 32]),
                AccountId::from([0x1b; 32]),
                AccountId::from([0x1c; 32]),
            ];

            // construct contract and initialize accounts
            let mut ILOCKtokenERC20 = ILOCKtoken::new_token(pool_accounts);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();


            // bob's member data
            let owes: u128 = 0;
            let share: u128 = 1_000_000;
            let pool: u128 = 8;

            // call registration function for bob
            ILOCKtokenERC20.register_member(accounts.bob, owes, share, pool);


            ILOCKtokenERC20.distribute_pools();

            let this_pool: PoolData = ILOCKtokenERC20.pooldata.get(pool_accounts[pool as usize]).unwrap();
            let this_member: MemberData = ILOCKtokenERC20.memberdata.get(accounts.bob).unwrap();

            for monthspassed in 0..86 {

                ILOCKtokenERC20.monthspassed = monthspassed;
                ILOCKtokenERC20.claim_tokens(accounts.bob);


                if monthspassed < this_pool.cliff {

                    ink_env::debug_println!("{:?}", this_pool.cliff);
                    ink_env::debug_println!("1.{:?}", ILOCKtokenERC20.balance_of(accounts.bob));
                    assert_eq!(ILOCKtokenERC20
                        .balance_of(
                            accounts.bob),
                            0,
                    );
                } else if monthspassed >= this_pool.cliff + this_pool.vests - 1 {
                    ink_env::debug_println!("2.{:?}", ILOCKtokenERC20.balance_of(accounts.bob));
                    assert_eq!(ILOCKtokenERC20
                        .balance_of(
                            accounts.bob),
                            this_member.share,
                    );
                } else {

                    assert_eq!(ILOCKtokenERC20
                        .balance_of(
                            accounts.bob),
                            (this_member.share / this_pool.vests) * (monthspassed - this_pool.cliff + 1),
                    );
                    ink_env::debug_println!("3.{:?}", ILOCKtokenERC20.balance_of(accounts.bob));

                }
            }
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
