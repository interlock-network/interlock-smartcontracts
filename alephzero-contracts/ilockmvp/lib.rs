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

pub use self::ilockmvp::{
    ILOCKmvp,
    ILOCKmvpRef,
};

#[openbrush::contract]
pub mod ilockmvp {

    use ink_lang::{
        codegen::{EmitEvent, Env},
        reflect::ContractEventBase,
    };
    use ink_prelude::{
        vec::Vec,
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
        contracts::{
            psp22::{
                extensions::{metadata::*, burnable::*},
                Internal,
            },
            ownable::*},
        traits::Storage,
    };

////////////////////////////////////////////////////////////////////////////
//// constants /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

    /// . magic numbers
    pub const ID_LENGTH: usize = 32;                                // 32B account id
    pub const POOL_COUNT: usize = 12;                               // number of stakeholder pools
    pub const ONE_MONTH: Timestamp = 2_592_000_000;                 // milliseconds in 30 days

    /// . token data
    pub const TOKEN_CAP: u128 = 1_000_000_000;                      // 10^9
    pub const DECIMALS_POWER10: u128 = 1_000_000_000_000_000_000;   // 10^18
    pub const SUPPLY_CAP: u128 = TOKEN_CAP * DECIMALS_POWER10;      // 10^27
    pub const TOKEN_NAME: &str = "Interlock Network";
    pub const TOKEN_DECIMALS: u8 = 18;
    pub const TOKEN_SYMBOL: &str = "ILOCK";

    #[derive(Debug)]
    pub struct PoolData<'a> {
        name: &'a str,
        tokens: u128,
        vests: u8,
        cliffs: u8,
    }

    /// . pool data
    pub const POOLS: [PoolData; POOL_COUNT] = [
        PoolData { name: "early_backers+venture_capital", tokens: 20_000_000,  vests: 24, cliffs: 1, },
        PoolData { name: "presale_1",                     tokens: 48_622_222,  vests: 18, cliffs: 1, },
        PoolData { name: "presale_2",                     tokens: 66_666_667,  vests: 15, cliffs: 1, },
        PoolData { name: "presale_3",                     tokens: 40_000_000,  vests: 12, cliffs: 1, },
        PoolData { name: "team+founders",                 tokens: 200_000_000, vests: 36, cliffs: 6, },
        PoolData { name: "outlier_ventures",              tokens: 40_000_000,  vests: 24, cliffs: 1, },
        PoolData { name: "advisors",                      tokens: 25_000_000,  vests: 24, cliffs: 1, },
        PoolData { name: "rewards",                       tokens: 285_000_000, vests: 1,  cliffs: 0, },
        PoolData { name: "foundation",                    tokens: 172_711_111, vests: 84, cliffs: 1, },
        PoolData { name: "partners",                      tokens: 37_000_000,  vests: 1,  cliffs: 0, },
        PoolData { name: "whitelist",                     tokens: 15_000_000,  vests: 48, cliffs: 0, },
        PoolData { name: "public_sale",                   tokens: 50_000_000,  vests: 48, cliffs: 0, },
    ];

    pub const EARLY_BACKERS: u8     = 0;
    pub const PRESALE_1: u8         = 1;
    pub const PRESALE_2: u8         = 2;
    pub const PRESALE_3: u8         = 3;
    pub const TEAM_FOUNDERS: u8     = 4;
    pub const OUTLIER_VENTURES: u8  = 5;
    pub const ADVISORS: u8          = 6;
    pub const REWARDS: u8           = 7;
    pub const FOUNDATION: u8        = 8;
    pub const PARTNERS: u8          = 9;
    pub const WHITELIST: u8         = 10;
    pub const PUBLIC_SALE: u8       = 11;

////////////////////////////////////////////////////////////////////////////
//// structured data ///////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

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

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout, Default, )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct Port {
        application: Hash,
        tax: Balance,
        cap: Balance,
        locked: bool,
        paid: Balance,
        collected: Balance,
        owner: AccountId,
    }

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout, Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct Socket {
        operator: AccountId,
        portnumber: u16,
    }


    /// . ILOCKmvp struct contains overall storage data for contract
    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ILOCKmvp {

        // ABSOLUTELY DO NOT CHANGE THE ORDER OF THESE VARIABLES, OR TYPE!!!
        // . TO ADD NEW VARIABLE, IT MUST BE APPENDED TO END OF LIST

        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
		ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,

        stakeholderdata: Mapping<AccountId, StakeholderData>,
        rewardedinterlocker: Mapping<AccountId, Balance>,
        poolbalances: [Balance; POOL_COUNT],
        rewardedtotal: Balance,
        circulatingsupply: Balance,
        taxpool: Balance,
        monthspassed: u16,
        nextpayout: Timestamp,
        ports: Mapping<u16, Port>,
        sockets: Mapping<AccountId, Socket>,  // application address -> socket
                                              //                        socket == operatoraddress:port
    }

////////////////////////////////////////////////////////////////////////////
//// events and errors /////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

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
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo)
    )]
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
        PaymentTooLarge,
        /// Returned if socket does not exist
        NoSocket,
        /// Returned if port does not exist
        NoPort,
        /// Returned if not contract
        NotContract,
        /// Returned if only owner can add socket
        PortLocked,
        /// Returned if port cap is surpassed
        PortCapSurpassed,
        /// Returned if reward recipient is a contract
        CannotRewardContract,
        /// Returned if socket contract does not match registered hash
        UnsafeContract,
        /// Returned if application contract caller is not its operator
        CallerNotOperator,
        /// custome contract error
        Custom(String),
    }

    impl Into<PSP22Error> for OtherError {
        fn into(self) -> PSP22Error {
            PSP22Error::Custom(format!("{:?}", self))
        }
    }

    impl Into<OtherError> for PSP22Error {
        fn into(self) -> OtherError {
            OtherError::Custom(format!("{:?}", self))
        }
    }

    pub type PSP22Result<T> = core::result::Result<T, PSP22Error>;

    /// . OtherError result type.
    pub type OtherResult<T> = core::result::Result<T, OtherError>;

    pub type Event = <ILOCKmvp as ContractEventBase>::Type;

////////////////////////////////////////////////////////////////////////////
/////// reimplement some functions /////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

    impl PSP22 for ILOCKmvp {
        
        /// . override default total_supply getter
        /// . total supply reflects token in circulation
        #[ink(message)]
        fn total_supply(&self) -> Balance {

            // revert, testing set code hash
            self.circulatingsupply + 999
        }

        /// . override default transfer doer
        /// . transfer from owner increases total supply
        #[ink(message)]
        fn transfer(
            &mut self,
            to: AccountId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP22Error> {

            let from = self.env().caller();

            let _ = self._transfer_from_to(from, to, value, data)?;

            // if sender is owner, then tokens are entering circulation
            if from == self.ownable.owner {
                self.circulatingsupply += value;
            }

            // if recipient is owner, then tokens are being returned or added to rewards pool
            if to == self.ownable.owner {
                self.poolbalances[REWARDS as usize] += value;
                self.circulatingsupply -= value;
            }

            Ok(())
        }

        /// . override default transfer_from_to doer
        /// . transfer from owner increases total supply
        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
            data: Vec<u8>,
        ) -> Result<(), PSP22Error> {

            let _ = self._transfer_from_to(from, to, value, data)?;

            // if sender is owner, then tokens are entering circulation
            if from == self.ownable.owner {
                self.circulatingsupply += value;
            }

            // if recipient is owner, then tokens are being returned or added to rewards pool
            if to == self.ownable.owner {
                self.poolbalances[REWARDS as usize] += value;
                self.circulatingsupply -= value;
            }

            Ok(())
        }

    }

    impl PSP22Metadata for ILOCKmvp {}

    impl Ownable for ILOCKmvp {
        
        // PRIOR TO OWNER TRANSFER,
        // REMAINING OWNER NONCIRCULATING
        // BALANCE MUST BE TRANSFERRED TO NEW OWNER.
    }

    impl PSP22Burnable for ILOCKmvp {

        /// . override default burn doer
        /// . burn function to permanently remove tokens from circulation / supply
        #[ink(message)]
		#[openbrush::modifiers(only_owner)]
        fn burn(
            &mut self,
            donor: AccountId,
            amount: Balance,
        ) -> PSP22Result<()> {

            // burn the tokens
            let _ = self._burn_from(donor, amount)?;
            self.circulatingsupply -= amount;

            Ok(())
        }
	}

    // these implementations are because open brush does not implement
    impl Internal for ILOCKmvp {

        fn _emit_transfer_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _amount: Balance,
        ) {
            ILOCKmvp::emit_event(
                self.env(),
                Event::Transfer(Transfer {
                    from: _from,
                    to: _to,
                    amount: _amount,
                }),
            );
        }

        fn _emit_approval_event(
            &self,
            _owner: AccountId,
            _spender: AccountId,
            _amount: Balance
        ) {
            ILOCKmvp::emit_event(
                self.env(),
                Event::Approval(Approval {
                    owner: Some(_owner),
                    spender: Some(_spender),
                    amount: _amount,
                }),
            );
        }
    }

////////////////////////////////////////////////////////////////////////////
/////// implement token contract ///////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

    impl ILOCKmvp {

        // Pete said this was probably necessary
        /// . function for internal _emit_event implementations
        pub fn emit_event<EE: EmitEvent<Self>>(emitter: EE, event: Event) {
            emitter.emit_event(event);
        }

        /// . constructor to initialize contract
        /// . note: pool contracts must be created prior to construction (for args)
        #[ink(constructor)]
        pub fn new_token(
        ) -> Self {

            // create contract
            ink_lang::codegen::initialize_contract(|contract: &mut Self| {

                // define owner as caller
                let caller = Self::env().caller();

                // set initial data
                contract.monthspassed = 0;
                contract.nextpayout = Self::env().block_timestamp() + ONE_MONTH;
                contract.rewardedtotal = 0;
                contract.circulatingsupply = 0;

                contract.metadata.name = Some(TOKEN_NAME.to_string());
                contract.metadata.symbol = Some(TOKEN_SYMBOL.to_string());
                contract.metadata.decimals = TOKEN_DECIMALS;

                // mint with openbrush:
                contract._mint(caller, SUPPLY_CAP)
                        .expect("Failed to mint the initial supply");
                contract._init_with_owner(caller);

                // create initial pool balances
                for pool in 0..POOL_COUNT {

                    contract.poolbalances[pool] =
                                    POOLS[pool].tokens * DECIMALS_POWER10;
                }
            })
        }

////////////////////////////////////////////////////////////////////////////
/////// timing /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

        /// . function to check if enough time has passed to collect next payout
        /// . this function ensures Interlock cannot rush the vesting schedule
        /// . this function must be called before the next round of token distributions
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn check_time(
            &mut self,
        ) -> PSP22Result<()> {

            // test to see if current time falls beyond time for next payout
            if self.env().block_timestamp() > self.nextpayout {

                // update time variables
                self.nextpayout += ONE_MONTH;
                self.monthspassed += 1;

                return Ok(());
            }

            // too early, do nothing
            return Err(OtherError::PayoutTooEarly.into())
        }
        
        /// . time in seconds until next payout in minutes
        #[ink(message)]
        pub fn remaining_time_until_next_payout(
            &self
        ) -> Timestamp {

            // calculate remaining time
            let timeleft: Timestamp = match self.nextpayout.checked_sub(self.env().block_timestamp()) {
                Some(difference) => difference,
                None => return 0,
            };

            timeleft
        }

////////////////////////////////////////////////////////////////////////////
/////// stakeholders  //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

        /// . function that registers a stakeholder's wallet and vesting info
        /// . used to calculate monthly payouts and track net paid
        /// . stakeholder data also used for stakeholder to verify their place in vesting schedule
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn register_stakeholder(
            &mut self,
            stakeholder: AccountId,
            share: Balance,
            pool: u8,
        ) -> PSP22Result<()> {

            // make sure share is > 0
            if share == 0 {
                return Err(PSP22Error::Custom("Share must be greater than zero.".to_string()));
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

        /// . function that returns a stakeholder's payout and other data
        /// . this will allow stakeholders to verify their stake from explorer if so motivated
        /// . returns tuple (paidout, payremaining, payamount, poolnumber)
        #[ink(message)]
        pub fn stakeholder_data(
            &self,
            stakeholder: AccountId,
        ) -> (String, String, String, String) {

            // get pool and stakeholder data structs first
            let this_stakeholder = self.stakeholderdata.get(stakeholder).unwrap();
            let pool = &POOLS[this_stakeholder.pool as usize];

            // how much has stakeholder already claimed?
            let paidout: Balance = this_stakeholder.paid;

            // how much does stakeholder have yet to collect?
            let payremaining: Balance = this_stakeholder.share - paidout;

            // how much does stakeholder get each month?
            let payamount: Balance = this_stakeholder.share / pool.vests as Balance;

            return (
                format!("paidout: {:?} ", paidout),
                format!("payremaining: {:?} ", payremaining),
                format!("payamount: {:?} ", payamount),
                format!("pool: {:?}", POOLS[this_stakeholder.pool as usize].name),
            )
        }

////////////////////////////////////////////////////////////////////////////
/////// token distribution /////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

        /// . general function to transfer the token share a stakeholder is currently entitled to
        /// . this is called once per stakeholder by Interlock, Interlock paying fees
        /// . pools are guaranteed to have enough tokens for all stakeholders
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn distribute_tokens(
            &mut self,
            stakeholder: AccountId,
        ) -> PSP22Result<()> {

            // get data structs
            let mut this_stakeholder = match self.stakeholderdata.get(stakeholder) {
                Some(s) => s,
                None => { return Err(OtherError::StakeholderNotFound.into()) },
            };
            let pool = &POOLS[this_stakeholder.pool as usize];

            // require cliff to have been surpassed
            if self.monthspassed < pool.cliffs as u16 {
                return Err(OtherError::CliffNotPassed.into())
            }

            // require share has not been completely paid out
            if this_stakeholder.paid == this_stakeholder.share {
                return Err(OtherError::StakeholderSharePaid.into())
            }

            // calculate the payout owed
            // ! no checked_div needed; pool.vests guaranteed to be nonzero
            let mut payout: Balance = this_stakeholder.share / pool.vests as Balance;

            // require that payout isn't repeatable for this month
            // ! no checked_div needed; this_stakeholder.share guaranteed to be nonzero
            let payments = this_stakeholder.paid / payout;
            if payments >= self.monthspassed as u128 {
                return Err(OtherError::PayoutTooEarly.into())
            }

            // calculate the new total paid to stakeholder
            let newpaidtotal: Balance = match this_stakeholder.paid.checked_add(payout) {
                Some(sum) => sum,
                None => return Err(PSP22Error::Custom("Overflow error.".to_string())),
            };

            // calculate remaining share
            let remainingshare: Balance = match this_stakeholder.paid.checked_sub(newpaidtotal) {
                Some(difference) => difference,
                None => return Err(PSP22Error::Custom("Underflow error.".to_string())),
            };

            // if this is final payment, add token remainder to payout
            // (this is to compensate for floor division that calculates payamount)
            // ! no checked_div needed; pool.vests guaranteed to be nonzero
            if remainingshare < this_stakeholder.share / pool.vests as Balance {

                // add remainder
                match payout.checked_add(this_stakeholder.share % pool.vests as Balance) {
                    Some(sum) => payout = sum,
                    None => return Err(PSP22Error::Custom("Overflow error.".to_string())),
                };
            }

            // now transfer tokens
            let _ = self.transfer(stakeholder, payout, Default::default())?;

            // update pool balance
            match self.poolbalances[this_stakeholder.pool as usize].checked_sub(payout) {
                Some(difference) => self.poolbalances[this_stakeholder.pool as usize] = difference,
                None => return Err(PSP22Error::Custom("Underflow error.".to_string())),
            };

            // finally update stakeholder data struct state
            this_stakeholder.paid = newpaidtotal;
            self.stakeholderdata.insert(stakeholder, &this_stakeholder);

            Ok(())
        }

        /// . function used to payout tokens to pools with no vesting schedule
        /// POOL ARGUMENTS:
        ///     PARTNERS
        ///     WHITELIST
        ///     PUBLIC_SALE
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn payout_tokens(
            &mut self,
            stakeholder: AccountId,
            amount: Balance,
            pool: String,
        ) -> PSP22Result<()> {

            let poolnumber: u8 = match pool.as_str() {
                "PARTNERS"      => 9,
                "WHITELIST"     => 10,
                "PUBLIC_SALE"   => 11,
                _ => return Err(OtherError::Custom(format!("Invalid pool.")).into()),
            };
        
            // make sure reward not too large
            if self.poolbalances[poolnumber as usize] < amount {
                return Err(OtherError::PaymentTooLarge.into())
            }

            // deduct payout amount
            match self.poolbalances[poolnumber as usize].checked_sub(amount) {
                Some(difference) => self.poolbalances[poolnumber as usize] = difference,
                None => return Err(PSP22Error::Custom("Underflow error.".to_string())),
            };

            // now transfer tokens
            let _ = self.transfer(stakeholder, amount, Default::default())?;

            Ok(())
        }

////////////////////////////////////////////////////////////////////////////
/////// pool data //////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

        /// . function that returns pool data
        /// . this will allow observers to verify vesting parameters for each pool (esp. theirs)
        /// . observers may verify pool data from explorer if so motivated
        /// . pool numbers range from 0-11
        /// . returns (name, tokens, vests, cliff)
        #[ink(message)]
        pub fn pool_data(
            &self,
            pool: u8,
        ) -> (String, String, String, String) {
        
            let pool = &POOLS[pool as usize];
            // just grab up and send it out
            return (
                format!("pool: {:?} ", pool.name.to_string()),
                format!("tokens alotted: {:?} ", pool.tokens),
                format!("number of vests: {:?} ", pool.vests),
                format!("vesting cliff: {:?} ", pool.cliffs),
            )
        }
        
        /// . get current balance of whitelist pool
        #[ink(message)]
        pub fn pool_balance(
            &self,
            pool: u8,
        ) -> (String, Balance) {

            (format!("pool: {:?} balance: {:?}", 
                    POOLS[pool as usize].name.to_string(),
                    self.poolbalances[pool as usize]),
             self.poolbalances[pool as usize])
        }

////////////////////////////////////////////////////////////////////////////
//// rewarding  ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

        /// . reward the interlocker for browsing
        /// . this is a manual rewarding function, to override the socket formalism
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn reward_interlocker(
            &mut self,
            reward: Balance,
            interlocker: AccountId
        ) -> PSP22Result<Balance> {

            // make sure reward not too large
            if self.poolbalances[REWARDS as usize] < reward {
                return Err(OtherError::PaymentTooLarge.into())
            }

            // update total amount rewarded to interlocker
            match self.rewardedtotal.checked_add(reward) {
                Some(sum) => self.rewardedtotal = sum,
                None => return Err(PSP22Error::Custom("Overflow error.".to_string())),
            };

            // update rewards pool balance
            match self.poolbalances[REWARDS as usize].checked_sub(reward) {
                Some(difference) => self.poolbalances[REWARDS as usize] = difference,
                None => return Err(PSP22Error::Custom("Underflow error.".to_string())),
            };

            // transfer reward tokens from rewards pool to interlocker
            let _ = self.transfer(interlocker, reward, Default::default())?;

            // get previous total rewarded to interlocker
            let rewardedinterlockertotal: Balance = match self.rewardedinterlocker.get(interlocker) {
                Some(total) => total,
                None => 0,
            };

            // compute and update new total awarded to interlocker
            let newrewardedtotal: Balance = match rewardedinterlockertotal.checked_add(reward) {
                Some(sum) => sum,
                None => return Err(PSP22Error::Custom("Overflow error.".to_string())),
            };
            self.rewardedinterlocker.insert(interlocker, &newrewardedtotal);

            // emit Reward event
            self.env().emit_event(Reward {
                to: Some(interlocker),
                amount: reward,
            });

            // this returns interlocker total reward amount for extension display purposes
            Ok(newrewardedtotal)
        }

        /// . get amount rewarded to interlocker to date
        #[ink(message)]
        pub fn rewarded_interlocker_total(
            &self,
            interlocker: AccountId
        ) -> Balance {

            match self.rewardedinterlocker.get(interlocker) {
                Some(total) => total,
                None => 0,
            }
        }

        /// . get total amount rewarded to date
        #[ink(message)]
        pub fn rewarded_total(
            &self
        ) -> Balance {

            self.rewardedtotal
        }

////////////////////////////////////////////////////////////////////////////
//// misc  /////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
        
        /// . get current balance of whitelist pool
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn withdraw_tax(
            &mut self,
            wallet: AccountId,
            amount: Balance
        ) -> PSP22Result<()> {

            // only withdraw what is available in pool
            if amount > self.taxpool {

                return Err(OtherError::PaymentTooLarge.into());
            }

            let _ = self.transfer(wallet, amount, Default::default())?;
            
            // deduct withdraw amount
            match self.taxpool.checked_sub(amount) {
                Some(difference) => self.taxpool = difference,
                None => return Err(PSP22Error::Custom("Underflow error.".to_string())),
            };

            Ok(())
        }

        /// . display taxpool balance
        #[ink(message)]
        pub fn tax_available(
            &self,
        ) -> Balance {

            self.taxpool
        }

        /// . function to get the number of months passed for contract
        #[ink(message)]
        pub fn months_passed(
            &self,
        ) -> u16 {

            self.monthspassed
        }

        /// . function to get the supply cap minted on TGE
        #[ink(message)]
        pub fn cap(
            &self,
        ) -> u128 {

            SUPPLY_CAP
        }

        /// . function to increment monthspassed for testing
        /// 
        ///
        ///     MUST BE DELETED PRIOR TO AUDIT
        ///
        ///
        #[ink(message)]
        pub fn TESTING_increment_month(
            &mut self,
        ) -> bool {

            self.monthspassed += 4;

            true
        }

////////////////////////////////////////////////////////////////////////////
//// portability and extensibility  ////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

        /// . modifies the code which is used to execute calls to this contract address
        /// . this upgrades the token contract logic while using old state
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn update_contract(
            &mut self,
            code_hash: [u8; 32]
        ) -> PSP22Result<()> {

            // takes code hash of updates contract and modifies preexisting logic to match
            ink_env::set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!(
                    "Failed to `set_code_hash` to {:?} due to {:?}",
                    code_hash, err
                )
            });

            Ok(())
        }

        /// . create a new port that rewards contract can register with
        /// . eaech port tracks amount rewarded, tax collected, and if it is locked or not
        /// . a locked port may only be registered by the interlock network foundation
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn create_port(
            &mut self,
            codehash: Hash,
            tax: Balance,
            cap: Balance,
            locked: bool,
            number: u16,
            owner: AccountId,
        ) -> PSP22Result<()> {

            let port = Port {
                application: codehash,     // <--! a port defines an external staking/reward contract plus any
                tax: tax,                  //      custom logic preceding the tax_and_reward() function
                cap: cap,
                locked: locked,
                paid: 0,
                collected: 0,
                owner: owner,
            };

            self.ports.insert(number, &port);

            Ok(())
        }

        /// . rewards/staking contracts register with token contract here
        /// . contract must first register with token contract to allow reward transfers
        #[ink(message)]
        pub fn create_socket(
            &mut self,
            operator: AccountId,
            portnumber: u16,
        ) -> OtherResult<()> {

            // get application address
            let application: AccountId = self.env().caller();

            // make sure caller is a contact, return if not
            if !self.env().is_contract(&application) {

                return Err(OtherError::NotContract);
            };

            // get hash of calling contract
            let callinghash: Hash = match self.env().code_hash(&application) {
                Ok(hash) => hash,
                Err(_) => return Err(OtherError::NotContract),
            };

            // get port specified by calling contract
            let port: Port = match self.ports.get(portnumber) {
                Some(port) => port,
                None => return Err(OtherError::NoPort),
            };

            // make sure port is unlocked, or caller is token contract owner (interlock)
            //   . this makes it so that people can't build their own client application
            //     to 'hijack' an approved and registered rewards contract.
            //   . if port is locked then only interlock can create new socket with port
            //   . socket creation is only called by an external contract that the port represents
            if port.locked && (self.ownable.owner != operator) {

                return Err(OtherError::PortLocked);
            }
            
            // compare calling contract hash to registered port hash
            // to make sure it is safe (ie, approved and audited by interlock)
            if callinghash == port.application {
                
                // if the same, contract is allowed to create socket (socket == operatoraddress:portnumber)
                let socket = Socket { operator: operator, portnumber: portnumber };

                // socket is registered with token contract thus the calling
                // contract that created the socket may start calling socket to receive rewards
                self.sockets.insert(application, &socket);
            
                // give socket allowance up to port cap
                //   . connecting contracts will not be able to reward
                //     more than cap specified by interlock (this may be a stipend, for example)
                //   . rewards fail to transfer if the amount paid plus the reward exceeds cap
                self.psp22.allowances.insert(
                    &(&self.ownable.owner, &application),
                    &port.cap
                );

                self._emit_approval_event(self.ownable.owner, application, port.cap);

                return Ok(()); 
            }

            // returns error if calling staking application contract is not a known
            // safe contract registered by interlock as a 'port' 
            Err(OtherError::UnsafeContract)
        }

        /// . check for socket and charge owner per port spec
        #[ink(message)]
        pub fn call_socket(
            &mut self,
            address: AccountId,
            amount: Balance,
            _data: Vec<u8>,
        ) -> OtherResult<()> {

            // make sure address is not contract; we do not want to reward contracts
            if self.env().is_contract(&address) {

                return Err(OtherError::CannotRewardContract);
            }

            // get socket, to get port assiciated with socket
            let socket: Socket = match self.sockets.get(self.env().caller()) {
                Some(socket) => socket,
                None => return Err(OtherError::NoSocket),
            };


            // get port info
            let port: Port = match self.ports.get(socket.portnumber) {
                Some(port) => port,
                None => return Err(OtherError::NoPort),
            };

            // tax socket owner, inject port logic, transfer reward
            match socket.portnumber {

                // NOTE: injecting custom logic into port requires Interlock Token
                //       contract codehash update after internal port contract audit
                
                // reserved Interlock ports
                0 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                1 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                2 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                3 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                4 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                5 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                6 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                7 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                8 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                9 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },

                // reserved community node ports
                10 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                11 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                12 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                13 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                14 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                15 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                16 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                17 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                18 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },
                19 => { self.tax_and_reward(address, amount, port, socket.portnumber)? },

                //
                // .
                // .
                // .
                //

                // ... custom logic example:
                65535 => {

                    // < inject custom logic here BEFORE tax_and_reward >
                    // <    (ie, do stuff with port and socket data)    >

                    // then reward and tax
                    self.tax_and_reward(address, amount, port, socket.portnumber)?
                },

                _ => return Err(OtherError::Custom(format!("Socket registered with invalid port."))),
            };

            Ok(())
        }

        /// . tax and reward socket
        pub fn tax_and_reward(
            &mut self,
            address: AccountId,
            amount: Balance,
            mut port: Port,
            portnumber: u16
        ) -> OtherResult<()> {

            // make sure this will not exceed port cap
            if port.cap < (port.paid + amount) {

                return Err(OtherError::PortCapSurpassed.into());
            }

            // TODO/QUESTION:
            // tax should probably be a fraction of reward,
            // instead of a flat rate per reward
            //   . this would change the logic a little bit
            // ?

            // transfer transaction tax from socket owner to token contract owner
            let _ = match self.transfer_from(port.owner, self.ownable.owner, port.tax, Default::default()) {
                Err(error) => return Err(error.into()),
                Ok(()) => (),  
            };

            // update pools
            self.taxpool += port.tax;
            port.collected += port.tax;

            // transfer reward to reward recipient
            let _ = match self.transfer_from(self.ownable.owner, address, amount, Default::default()) {
                Err(error) => return Err(error.into()),
                Ok(()) => (),
            };

            // update balance pool and totals
            // (the port.tax subtraction is to offset rewardpool increase on transfer from token owner)
            self.poolbalances[REWARDS as usize] -= amount + port.tax; // << extra port.tax term to
            self.rewardedtotal += amount;                             // offset transfer function

            // update port
            port.paid += amount;
            self.ports.insert(portnumber, &port);

            // emit Reward event
            self.env().emit_event(Reward {
                to: Some(address),
                amount: amount,
            });

            Ok(())
        }

        /// . get socket info
        #[ink(message)]
        pub fn socket(
            &self,
            application: AccountId,
        ) -> Socket {
            
            match self.sockets.get(application) {
                Some(socket) => socket,
                None => Default::default(),
            }
        }

        /// . get port info
        #[ink(message)]
        pub fn port(
            &self,
            portnumber: u16,
        ) -> Port {
            
            match self.ports.get(portnumber) {
                Some(port) => port,
                None => Default::default(),
            }
        }        
    }

////////////////////////////////////////////////////////////////////////////
//// tests /////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
//
// INCOMPLETE
//
// . To view debug prints and assertion failures run test via:
//   cargo nightly+ test -- --nocapture
// . To view debug for specific method run test via:
//   cargo nightly+ test <test_function_here> -- --nocapture

    #[cfg(test)]
    mod tests {

        use super::*;
        use ink_lang as ink;
        use ink_lang::codegen::Env;

        /// . test if the default constructor does its job
        #[ink::test]
        fn constructor_works() {

            let ILOCKmvpPSP22 = ILOCKmvp::new_token();

            // the rest
            assert_eq!(ILOCKmvpPSP22.monthspassed, 0);
            assert_eq!(ILOCKmvpPSP22.nextpayout, ILOCKmvpPSP22.env().block_timestamp() + ONE_MONTH);
        }

        /// . test if name getter does its job
        #[ink::test]
        fn name_works() {

            let ILOCKmvpPSP22 = ILOCKmvp::new_token();
            assert_eq!(ILOCKmvpPSP22.metadata.name, Some("Interlock Network".to_string()));
        }

        /// . test if symbol getter does its job
        #[ink::test]
        fn symbol_works() {

            let ILOCKmvpPSP22 = ILOCKmvp::new_token();
            assert_eq!(ILOCKmvpPSP22.metadata.symbol, Some("ILOCK".to_string()));
        }
        
        /// . test if decimals getter does its job
        #[ink::test]
        fn decimals_works() {

            let ILOCKmvpPSP22 = ILOCKmvp::new_token();
            assert_eq!(ILOCKmvpPSP22.metadata.decimals, 18);
        }

        /// . test if balance getter does its job
        #[ink::test]
        fn balance_of_works() {

            let mut ILOCKmvpPSP22 = ILOCKmvp::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // charge alice's account
            ILOCKmvpPSP22.psp22.balances.insert(&accounts.alice, &100);

            assert_eq!(ILOCKmvpPSP22.balance_of(accounts.alice), 100);
        }

        /// . test if allowance getter does its job
        #[ink::test]
        fn allowance_works() {

            let mut ILOCKmvpPSP22 = ILOCKmvp::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice has not yet approved Bob
            assert_eq!(ILOCKmvpPSP22.allowance(accounts.alice, accounts.bob), 0);

            // Alice approves Bob for tokens
            assert_eq!(ILOCKmvpPSP22.approve(accounts.bob, 10), Ok(()));

            // Bob's new allowance reflects this approval
            assert_eq!(ILOCKmvpPSP22.allowance(accounts.alice, accounts.bob), 10);
        }

        /// . test if increase allowance does does its job
        #[ink::test]
        fn increase_allowance_works() {

            let mut ILOCKmvpPSP22 = ILOCKmvp::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice approves bob to spend tokens
            assert_eq!(ILOCKmvpPSP22.approve(accounts.bob, 10), Ok(()));

            // Bob is approved to spend tokens owned by Alice
            assert_eq!(ILOCKmvpPSP22.allowance(accounts.alice, accounts.bob), 10);

            // Alice increases Bobs allowance
            assert_eq!(ILOCKmvpPSP22.increase_allowance(accounts.bob, 10), Ok(()));

            // Bob is approved to spend extra tokens owned by Alice
            assert_eq!(ILOCKmvpPSP22.allowance(accounts.alice, accounts.bob), 20);
        }

        /// . test if decrease allowance does does its job
        #[ink::test]
        fn decrease_allowance_works() {

            let mut ILOCKmvpPSP22 = ILOCKmvp::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice approves bob to spend tokens
            assert_eq!(ILOCKmvpPSP22.approve(accounts.bob, 10), Ok(()));

            // Bob is approved to spend tokens owned by Alice
            assert_eq!(ILOCKmvpPSP22.allowance(accounts.alice, accounts.bob), 10);

            // Alice increases Bobs allowance
            assert_eq!(ILOCKmvpPSP22.decrease_allowance(accounts.bob, 5), Ok(()));

            // Bob is approved to spend extra tokens owned by Alice
            assert_eq!(ILOCKmvpPSP22.allowance(accounts.alice, accounts.bob), 5);
        }

        /// . test if wallet registration function works as intended 
        #[ink::test]
        fn register_stakeholder_works() {

            let mut ILOCKmvpPSP22 = ILOCKmvp::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // bob's stakeholder data
            let share: Balance = 1_000_000;
            let pool: u8 = 3;

            // call registration function
            ILOCKmvpPSP22.register_stakeholder(accounts.bob, share, pool).unwrap();

            // verify registration stuck
            let this_stakeholder = ILOCKmvpPSP22.stakeholderdata.get(accounts.bob).unwrap();
            assert_eq!(this_stakeholder.paid, 0);
            assert_eq!(this_stakeholder.share, share);
            assert_eq!(this_stakeholder.pool, pool);
        }
     
        /// . test if pool data getter does its job
        #[ink::test]
        fn pool_data_works() {

            let ILOCKmvpPSP22 = ILOCKmvp::new_token();
            let pool = &POOLS[1];
            assert_eq!(ILOCKmvpPSP22.pool_data(1), (
                format!("pool: {:?} ", pool.name.to_string()),
                format!("tokens alotted: {:?} ", pool.tokens),
                format!("number of vests: {:?} ", pool.vests),
                format!("vesting cliff: {:?} ", pool.cliffs),
            ));
        }

        /// . test if months passed getter does its job
        #[ink::test]
        fn months_passed_works() {

            let mut ILOCKmvpPSP22 = ILOCKmvp::new_token();
            ILOCKmvpPSP22.monthspassed = 99;
            assert_eq!(ILOCKmvpPSP22.months_passed(), 99);
        }

//
// Cannot perform following unit tests: off-chain environment does not support contract invocation.
//
// This is an unavoidable openbrush problem, for now.
//
//  fn transfer()
//  fn approve()
//  fn transfer_from()
//  fn distribute_tokens()
//  fn burn()
//

    }
}
