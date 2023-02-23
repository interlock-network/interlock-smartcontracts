//
// INTERLOCK NETWORK MVP SMART CONTRACT
//  - PSP22 TOKEN
//  - REWARDS
//
// !!!!! INCOMPLETE AND UNAUDITED, WARNING !!!!!
//
// This is a standard ERC20-style token contract
// with provisions for enforcing a token distribution
// vesting schedule, and for rewarding interlockers for
// browsing the internet with the Interlock browser extension.
//
// Build with cargo-contract version 2.0.0
//
//      cargo install cargo-contract --force --version 2.0.0
//
// Build
//
//      cargo contract build
//

#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use self::ilockmvp::{
    ILOCKmvp,
    ILOCKmvpRef,
};

#[openbrush::contract]
pub mod ilockmvp {

    use ink::{
        codegen::{EmitEvent, Env},
        reflect::ContractEventBase,
    };
    use ink::prelude::{
        vec::Vec,
        format,
        string::{String, ToString},
    };
    use ink::storage::Mapping;
    use openbrush::{
        contracts::{
            psp22::{
                extensions::{metadata::*, burnable::*},
                Internal,
            },
            ownable::*,
        },
        traits::Storage,
    };

////////////////////////////////////////////////////////////////////////////
//// constants /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

    /// - magic numbers
    pub const ID_LENGTH: usize = 32;                                // 32B account id
    pub const POOL_COUNT: usize = 12;                               // number of stakeholder pools
    pub const ONE_MONTH: Timestamp = 2_592_000_000;                 // milliseconds in 30 days

    /// - token data
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

    /// - pool data
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

    /// - This is upgradable storage for the token rewarding feature of this
    /// PSP22 contract.
    pub const REWARD_KEY: u32 = openbrush::storage_unique_key!(RewardData);
    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(REWARD_KEY)]
    pub struct RewardData {

        // ABSOLUTELY DO NOT CHANGE THE ORDER OF THESE VARIABLES
        // OR TYPES IF UPGRADING THIS CONTRACT!!!

        /// - How much ILOCK have we rewarded each Interlocker?
        interlocker: Mapping<AccountId, Balance>,

        /// - In total, how much ILOCK have we rewarded to Interlockers?
        total: Balance,

        /// - Expand storage related to the pool accounting functionality.
        pub _reserved: Option<()>,
    }

    /// - This is upgradable storage for the token pool management and accounting feature of this
    /// PSP22 contract.
    pub const POOL_KEY: u32 = openbrush::storage_unique_key!(TokenPools);
    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(POOL_KEY)]
    pub struct TokenPools {

        // ABSOLUTELY DO NOT CHANGE THE ORDER OF THESE VARIABLES
        // OR TYPES IF UPGRADING THIS CONTRACT!!!

        /// - What are the current balances of all the vesting pools?
        /// - This includes the rewards pool balance.
        balances: [Balance; POOL_COUNT],

        /// - How much ILOCK is circulating right now?
        /// - This includes token held by liquidity pools/exchanges.
        /// - This is the value of `total_supply()` getter.
        circulating: Balance,

        /// - How much do we have available in collected taxes/fees from port owners
        /// and application contract operators?
        tax: Balance,

        /// - Expand storage related to the pool accounting functionality.
        pub _reserved: Option<()>,
    }

    /// - This is upgradable storage for the application connection feature of this
    /// PSP22 contract (ie, the application/socket/port contract connectivity formalism).
    pub const VEST_KEY: u32 = openbrush::storage_unique_key!(VestData);
    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(VEST_KEY)]
    pub struct VestData {

        // ABSOLUTELY DO NOT CHANGE THE ORDER OF THESE VARIABLES
        // OR TYPES IF UPGRADING THIS CONTRACT!!!

        /// - Contains information about stakeholders and the vesting
        /// status.
        /// - See detailed struct below.
        ///
        /// stakeholder:         stakeholder account address -> info about stakeholder
        pub stakeholder: Mapping<AccountId, StakeholderData>,

        /// - Counter responsible for keeping track of how many months have passed
        /// along the vesting schedule.
        /// - Used in part to calculate and compare token amount paid out vs token amount owed.
        pub monthspassed: u16,

        /// - Stores the date timestamp one month ahead of the last increment of
        /// `monthspassed`
        pub nextpayout: Timestamp,

        /// - Expand storage related to the vesting functionality.
        pub _reserved: Option<()>,
    }
    /// - StakeholderData struct contains all pertinent information for each stakeholder
    /// (Besides balance and allowance mappings).
    /// - This is primarily for managing and implementing the vesting schedule.
    #[derive(scale::Encode, scale::Decode, Clone, Default)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
        PartialEq,
        Eq,
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout
        )
    )]
    pub struct StakeholderData {

        // ABSOLUTELY DO NOT CHANGE THE ORDER OF THESE VARIABLES
        // OR TYPES IF UPGRADING THIS CONTRACT!!!

        /// - How much so far has this stakeholder been paid in ILOCK?
        paid: Balance,

        /// - What is the overall ILOCK token share for this stakeholder?
        share: Balance,

        /// - Which vesting pool does this stakeholder belong to?
        /// - The pool determines the vesting schedule.
        pool: u8,
    }

    /// - This is upgradable storage for the application connection feature of this
    /// PSP22 contract (ie, the application/socket/port contract connectivity formalism).
    pub const APP_KEY: u32 = openbrush::storage_unique_key!(ApplicationData);
    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(APP_KEY)]
    pub struct AppData {

        // ABSOLUTELY DO NOT CHANGE THE ORDER OF THESE VARIABLES
        // OR TYPES IF UPGRADING THIS CONTRACT!!!

        /// - Contains information specifying a particular _type_ of connecting
        /// external application contract via the application/socket abstraction.
        /// - When an application contract creates a connecting socket with this token
        /// contract with a particular port, it adheres to the logic and protocol
        /// specified by the port type.
        /// - For example, PORT 0 in this contract only accepts connections from universal
        /// access NFT contract owned by Interlock, and for every socket call from a UANFT contract 
        /// application, tokens in the amount of the set NFT price are transferred from the calling minter
        /// to this ILOCK contract's owner account. On the application side, once the ILOCK
        /// tokens are successfully transferred via the port protocol, a UANFT is minted to
        /// the caller.
        /// - For example, PORT 1 in this contract is the same as PORT 0, but UANFT application
        /// contracts are owned by different operators, and on each socket call, the protocol
        /// includes an additional tax in ILOCK, which Interlock Network collects.
        /// - The mapping is from port number, to port details and specs.
        /// - Only this contract's owner has the authority to create or edit a port.
        /// - See detailed struct below.
        ///
        /// ports:         port number -> port(app contract hash, metadata, port owner)
        ///
        pub ports: Mapping<u16, Port>,

        /// - Contains information specifying a particular _instance_ of an application
        /// (as defined by port application hash) contract's connection to this PSP22
        /// contract.
        /// - Similar to the standard TCP/IP address:port format, the port specifies the
        /// protocol, and the address specifies the operator of that particular instance
        /// of the application contract connecting to this PSP22 contract.
        /// - In the example of PORT 1, the address of a socket connection is the address
        /// that receives the ILOCK token transfer, ultimately in exchange for the UANFT
        /// mint back on the application side.
        /// - The mapping is from application address, to socket operator address and port number.
        /// - One socket may serve multiple applications (ie, the same operator address:port
        /// number pair) which is a slight deviation from the socket formality in TCP/IP.
        /// - Any agent with a verified application contract may connect to this PSP22 contract
        /// without permission from this contract's owner.
        /// - See detailed struct below.
        ///
        /// sockets:         application contract address -> socket(app operator address : port)
        ///
        pub sockets: Mapping<AccountId, Socket>,

        /// - Expand storage related to the application/socket/port functionality.
        pub _reserved: Option<()>,
    }
    /// - Information pertaining to port definition in application/socket/port contract
    /// connectivity formalism.
    #[derive(scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
        PartialEq,
        Eq,
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout
        )
    )]
    pub struct Port {

        // ABSOLUTELY DO NOT CHANGE THE ORDER OF THESE VARIABLES
        // OR TYPES IF UPGRADING THIS CONTRACT!!!

        /// - What is the codehash of the application smart contract associated with
        /// this port?
        /// - This codehash is the application template that numerous individual application 
        /// contracts may be instantiated and connected to this PSP22 contract via socket
        /// without signed permission from this ILOCK contract's owner.
        /// - This codehash is essential to making sure that only safe and approved application
        /// contracts are able to connect to this token contract and manipulate its owneronly
        /// functionalities (as defined per respective port protocol).
        application: Hash,

        /// - How much does Interlock tax transaction taking place within a port protocol's
        /// socket call?
        tax: Balance,

        /// - For withdrawing rewards from ILOCK rewards pool, what is the max this particular
        /// port owner's application type can withdraw from rewards pool?
        cap: Balance,

        /// - If locked, only Interlock token contract owner can create a socket connection with
        /// this token contract using the appropriate application codehash.
        locked: bool,

        /// - How much ILOCK has this port been rewarded or issued throughout the course of
        /// its operation (in case where protocol rewards or issues ILOCK, that is)?
        paid: Balance,

        /// - How much has Interlock collected from this port in taxes or other collections?
        collected: Balance,

        /// - Who is the overall owner of this port?
        /// - Socket operators are not necessarily owners of the port.
        /// - For example, a restaurant franchise has one owner, whereas the franchise may have
        /// numberous restaurant locations, each with it's own operator, each operator/franchise
        /// pair forming a separate socket connection.
        owner: AccountId,
    }
    /// - Ink 4 has no AccountId Default impl thus struct Default cannot be derived
    /// due to `owner` field.
    /// - Default derivation is required by openbrush contract implementation of
    /// contract storage.
    impl Default for Port {
        fn default() -> Port {
            Port {
                application: Default::default(),
                tax: 0,
                cap: 0,
                locked: true,
                paid: 0,
                collected: 0,
                owner: AccountId::from([1_u8;32]),
            }
        }
    }
    /// - Information pertaining to socket definition in application/socket/port contract
    /// connectivity formalism.
    #[derive(scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
        PartialEq,
        Eq,
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout
        )
    )]
    pub struct Socket {

        // ABSOLUTELY DO NOT CHANGE THE ORDER OF THESE VARIABLES
        // OR TYPES IF UPGRADING THIS CONTRACT!!!

        /// - Who operates (owns usually) a specific instance of a connecting application
        /// contract?
        /// - Using the restaurant franchise metaphor again, the operator may have several
        /// different instances of the port's application contract.
        /// - Each instance of the application contract has its own address, but each restaurant
        /// has the same operator.
        /// - The socket (operator:franchise or operator:port#) is like the single business franchise
        /// agreement between the restaurant operator and the franchise owner.
        /// - There is only one agreement between the franchise and the restaurant operator,
        /// regardless of how many restaurants the operator has.
        operator: AccountId,

        /// - What port is this operator connected to?
        /// - Using the restaurant franchise metaphor again, the port is like the franchise
        /// itself.
        /// - The port number is what identifies a particular franchise and its protocols,
        /// procedures, metadata, and ultimately business model and standards for any
        /// franchisees.
        portnumber: u16,
    }
    /// - Ink 4 has no AccountId Default impl thus struct Default cannot be derived
    /// due to `operator` field.
    impl Default for Socket {
        fn default() -> Socket {
            Socket {
                operator: AccountId::from([1_u8;32]),
                portnumber: 65535,
            }
        }
    }

    /// - ILOCKmvp struct contains overall storage data for contract
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct ILOCKmvp {

        // ABSOLUTELY DO NOT CHANGE THE ORDER OF THESE VARIABLES
        // OR TYPES IF UPGRADING THIS CONTRACT!!!

        /// - Openbrush PSP22.
        #[storage_field]
        psp22: psp22::Data,

        /// - Openbrush ownership extension.
        #[storage_field]
		ownable: ownable::Data,

        /// - Openbrush metadata extension.
        #[storage_field]
        metadata: metadata::Data,

        /// - ILOCK Rewards info.
        #[storage_field]
        reward: RewardData,

        /// - ILOCK token pool info.
        #[storage_field]
        pool: TokenPools,

        /// - ILOCK vesting info.
        #[storage_field]
        vest: VestData,

        /// - ILOCK connecting application contract info
        #[storage_field]
        app: AppData,
    }

////////////////////////////////////////////////////////////////////////////
//// events and errors /////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

    /// - specify transfer event
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        amount: Balance,
    }

    /// - specify approve event
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        spender: Option<AccountId>,
        amount: Balance,
    }

    /// - specify reward event
    #[ink(event)]
    pub struct Reward {
        #[ink(topic)]
        to: Option<AccountId>,
        amount: Balance,
    }

    /// - Other contract error types
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
        /// Returned if checked add overflows
        Overflow,
        /// Returned if checked sub underflows
        Underflow,
        /// custome contract error
        Custom(String),
    }

    impl Into<PSP22Error> for OtherError {
        fn into(self) -> PSP22Error {
            PSP22Error::Custom(format!("{:?}", self).into_bytes())
        }
    }

    impl Into<OtherError> for PSP22Error {
        fn into(self) -> OtherError {
            OtherError::Custom(format!("{:?}", self))
        }
    }

    // for ILOCKmvpRef used in PSP34 contract
    impl From<OwnableError> for OtherError {
        fn from(error: OwnableError) -> Self {
            OtherError::Custom(format!("{:?}", error))
        }
    }

    pub type PSP22Result<T> = core::result::Result<T, PSP22Error>;

    pub type OtherResult<T> = core::result::Result<T, OtherError>;

    pub type Event = <ILOCKmvp as ContractEventBase>::Type;

////////////////////////////////////////////////////////////////////////////
/////// reimplement some functions /////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

    impl PSP22 for ILOCKmvp {
        
        /// - override default total_supply getter
        /// - total supply reflects token in circulation
        #[ink(message)]
        fn total_supply(&self) -> Balance {

            // revert, testing set code hash
            self.pool.circulating
        }

        /// - override default transfer doer
        /// - transfer from owner increases total supply
        #[ink(message)]
        fn transfer(
            &mut self,
            to: AccountId,
            value: Balance,
            data: Vec<u8>,
        ) -> PSP22Result<()> {

            let from = self.env().caller();

            let _ = self._transfer_from_to(from, to, value, data)?;

            // if sender is owner, then tokens are entering circulation
            if from == self.ownable.owner {

                match self.pool.circulating.checked_add(value) {
                    Some(sum) => self.pool.circulating = sum,
                    None => return Err(OtherError::Overflow.into()),
                };
            }

            // if recipient is owner, then tokens are being returned or added to rewards pool
            if to == self.ownable.owner {

                match self.pool.balances[REWARDS as usize].checked_add(value) {
                    Some(sum) => self.pool.balances[REWARDS as usize] = sum,
                    None => return Err(OtherError::Overflow.into()),
                };
                match self.pool.circulating.checked_sub(value) {
                    Some(difference) => self.pool.circulating = difference,
                    None => return Err(OtherError::Underflow.into()),
                };
            }

            Ok(())
        }

        /// - override default transfer_from_to doer
        /// - transfer from owner increases total supply
        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
            data: Vec<u8>,
        ) -> PSP22Result<()> {

            let _ = self._transfer_from_to(from, to, value, data)?;

            // if sender is owner, then tokens are entering circulation
            if from == self.ownable.owner {

                match self.pool.circulating.checked_add(value) {
                    Some(sum) => self.pool.circulating = sum,
                    None => return Err(OtherError::Overflow.into()),
                };
            }

            // if recipient is owner, then tokens are being returned or added to rewards pool
            if to == self.ownable.owner {

                match self.pool.balances[REWARDS as usize].checked_add(value) {
                    Some(sum) => self.pool.balances[REWARDS as usize] = sum,
                    None => return Err(OtherError::Overflow.into()),
                };
                match self.pool.circulating.checked_sub(value) {
                    Some(difference) => self.pool.circulating = difference,
                    None => return Err(OtherError::Underflow.into()),
                };
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

        /// - override default burn doer
        /// - burn function to permanently remove tokens from circulation / supply
        #[ink(message)]
		#[openbrush::modifiers(only_owner)]
        fn burn(
            &mut self,
            donor: AccountId,
            amount: Balance,
        ) -> PSP22Result<()> {

            // burn the tokens
            let _ = self._burn_from(donor, amount)?;
            self.pool.circulating -= amount;

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

    // this is for linking openbrush PSP34 contract
    impl Default for ILOCKmvpRef {
        fn default() -> ILOCKmvpRef {
            ink::env::call::FromAccountId::from_account_id(AccountId::from([1_u8; 32]))
        }
    }

////////////////////////////////////////////////////////////////////////////
/////// implement token contract ///////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

    impl ILOCKmvp {

        /// - function for internal _emit_event implementations
        pub fn emit_event<EE: EmitEvent<Self>>(emitter: EE, event: Event) {
            emitter.emit_event(event);
        }

        /// - constructor to initialize contract
        /// - note: pool contracts must be created prior to construction (for args)
        #[ink(constructor)]
        pub fn new_token(
        ) -> Self {

            // create contract
            let mut contract = Self::default();
                
            // define owner as caller
            let caller = contract.env().caller();

            // set initial data
            contract.vest.monthspassed = 0;
            contract.vest.nextpayout = Self::env().block_timestamp() + ONE_MONTH;
            contract.reward.total = 0;
            contract.pool.circulating = 0;

            contract.metadata.name = Some(TOKEN_NAME.to_string().into_bytes());
            contract.metadata.symbol = Some(TOKEN_SYMBOL.to_string().into_bytes());
            contract.metadata.decimals = TOKEN_DECIMALS;

            // mint with openbrush:
            contract._mint_to(caller, SUPPLY_CAP)
                    .expect("Failed to mint the initial supply");
            contract._init_with_owner(caller);

            // create initial pool balances
            for pool in 0..POOL_COUNT {

                contract.pool.balances[pool] =
                                POOLS[pool].tokens * DECIMALS_POWER10;
            }
            
            contract
        }

////////////////////////////////////////////////////////////////////////////
/////// timing /////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

        /// - function to check if enough time has passed to collect next payout
        /// - this function ensures Interlock cannot rush the vesting schedule
        /// - this function must be called before the next round of token distributions
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn check_time(
            &mut self,
        ) -> PSP22Result<()> {

            // test to see if current time falls beyond time for next payout
            if self.env().block_timestamp() > self.vest.nextpayout {

                // update time variables
                self.vest.nextpayout += ONE_MONTH;
                self.vest.monthspassed += 1;

                return Ok(());
            }

            // too early, do nothing
            return Err(OtherError::PayoutTooEarly.into())
        }
        
        /// - time in seconds until next payout in minutes
        #[ink(message)]
        pub fn remaining_time_until_next_payout(
            &self
        ) -> Timestamp {

            // calculate remaining time
            let timeleft: Timestamp = match self.vest.nextpayout.checked_sub(self.env().block_timestamp()) {
                Some(difference) => difference,
                None => return 0,
            };

            timeleft
        }

////////////////////////////////////////////////////////////////////////////
/////// stakeholders  //////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

        /// - function that registers a stakeholder's wallet and vesting info
        /// - used to calculate monthly payouts and track net paid
        /// - stakeholder data also used for stakeholder to verify their place in vesting schedule
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
                return Err(PSP22Error::Custom("Share must be greater than zero.".as_bytes().to_vec()));
            }

            // create stakeholder struct
            let this_stakeholder = StakeholderData {
                paid: 0,
                share: share,
                pool: pool,
            };

            // insert stakeholder struct into mapping
            self.vest.stakeholder.insert(stakeholder, &this_stakeholder);

            Ok(())
        }

        /// - function that returns a stakeholder's payout and other data
        /// - this will allow stakeholders to verify their stake from explorer if so motivated
        /// - returns tuple (paidout, payremaining, payamount, poolnumber)
        #[ink(message)]
        pub fn stakeholder_data(
            &self,
            stakeholder: AccountId,
        ) -> (String, String, String, String) {

            // get pool and stakeholder data structs first
            let this_stakeholder = self.vest.stakeholder.get(stakeholder).unwrap();
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

        /// - general function to transfer the token share a stakeholder is currently entitled to
        /// - this is called once per stakeholder by Interlock, Interlock paying fees
        /// - pools are guaranteed to have enough tokens for all stakeholders
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn distribute_tokens(
            &mut self,
            stakeholder: AccountId,
        ) -> PSP22Result<()> {

            // get data structs
            let mut this_stakeholder = match self.vest.stakeholder.get(stakeholder) {
                Some(s) => s,
                None => { return Err(OtherError::StakeholderNotFound.into()) },
            };
            let pool = &POOLS[this_stakeholder.pool as usize];

            // require cliff to have been surpassed
            if self.vest.monthspassed < pool.cliffs as u16 {
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
            if payments >= self.vest.monthspassed as u128 {
                return Err(OtherError::PayoutTooEarly.into())
            }

            // calculate the new total paid to stakeholder
            let newpaidtotal: Balance = match this_stakeholder.paid.checked_add(payout) {
                Some(sum) => sum,
                None => return Err(OtherError::Overflow.into()),
            };

            // calculate remaining share
            let remainingshare: Balance = match this_stakeholder.share.checked_sub(newpaidtotal) {
                Some(difference) => difference,
                None => return Err(OtherError::Underflow.into()),
            };

            // if this is final payment, add token remainder to payout
            // (this is to compensate for floor division that calculates payamount)
            // ! no checked_div needed; pool.vests guaranteed to be nonzero
            if remainingshare < this_stakeholder.share / pool.vests as Balance {

                // add remainder
                match payout.checked_add(this_stakeholder.share % pool.vests as Balance) {
                    Some(sum) => payout = sum,
                    None => return Err(OtherError::Overflow.into()),
                };
            }

            // now transfer tokens
            let _ = self.transfer(stakeholder, payout, Default::default())?;

            // update pool balance
            match self.pool.balances[this_stakeholder.pool as usize].checked_sub(payout) {
                Some(difference) => self.pool.balances[this_stakeholder.pool as usize] = difference,
                None => return Err(OtherError::Underflow.into()),
            };

            // finally update stakeholder data struct state
            this_stakeholder.paid = newpaidtotal;
            self.vest.stakeholder.insert(stakeholder, &this_stakeholder);

            Ok(())
        }

        /// - function used to payout tokens to pools with no vesting schedule
        /// POOL ARGUMENTS:
        ///      PARTNERS
        ///      WHITELIST
        ///      PUBLIC_SALE
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
            if self.pool.balances[poolnumber as usize] < amount {
                return Err(OtherError::PaymentTooLarge.into())
            }

            // deduct payout amount
            match self.pool.balances[poolnumber as usize].checked_sub(amount) {
                Some(difference) => self.pool.balances[poolnumber as usize] = difference,
                None => return Err(OtherError::Underflow.into()),
            };

            // now transfer tokens
            let _ = self.transfer(stakeholder, amount, Default::default())?;

            Ok(())
        }

////////////////////////////////////////////////////////////////////////////
/////// pool data //////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

        /// - function that returns pool data
        /// - this will allow observers to verify vesting parameters for each pool (esp. theirs)
        /// - observers may verify pool data from explorer if so motivated
        /// - pool numbers range from 0-11
        /// - returns (name, tokens, vests, cliff)
        #[ink(message)]
        pub fn pool_data(
            &self,
            poolnumber: u8,
        ) -> (String, String, String, String) {
        
            let pool = &POOLS[poolnumber as usize];

            return (
                format!("pool: {:?} ", pool.name.to_string()),
                format!("tokens alotted: {:?} ", pool.tokens),
                format!("number of vests: {:?} ", pool.vests),
                format!("vesting cliff: {:?} ", pool.cliffs),
            )
        }
        
        /// - get current balance of whitelist pool
        #[ink(message)]
        pub fn pool_balance(
            &self,
            pool: u8,
        ) -> (String, Balance) {

            (format!("pool: {:?}, balance: {:?}", 
                    POOLS[pool as usize].name.to_string(),
                    self.pool.balances[pool as usize]),
             self.pool.balances[pool as usize])
        }

////////////////////////////////////////////////////////////////////////////
//// rewarding  ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

        /// - reward the interlocker for browsing
        /// - this is a manual rewarding function, to override the socket formalism
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn reward_interlocker(
            &mut self,
            reward: Balance,
            interlocker: AccountId
        ) -> PSP22Result<Balance> {

            // make sure reward not too large
            if self.pool.balances[REWARDS as usize] < reward {
                return Err(OtherError::PaymentTooLarge.into())
            }

            // update total amount rewarded to interlocker
            match self.reward.total.checked_add(reward) {
                Some(sum) => self.reward.total = sum,
                None => return Err(OtherError::Overflow.into()),
            };

            // update rewards pool balance
            match self.pool.balances[REWARDS as usize].checked_sub(reward) {
                Some(difference) => self.pool.balances[REWARDS as usize] = difference,
                None => return Err(OtherError::Underflow.into()),
            };

            // transfer reward tokens from rewards pool to interlocker
            let _ = self.transfer(interlocker, reward, Default::default())?;

            // get previous total rewarded to interlocker
            let rewardedinterlockertotal: Balance = match self.reward.interlocker.get(interlocker) {
                Some(total) => total,
                None => 0,
            };

            // compute and update new total awarded to interlocker
            let newrewardedtotal: Balance = match rewardedinterlockertotal.checked_add(reward) {
                Some(sum) => sum,
                None => return Err(OtherError::Overflow.into()),
            };
            self.reward.interlocker.insert(interlocker, &newrewardedtotal);

            // emit Reward event
            self.env().emit_event(Reward {
                to: Some(interlocker),
                amount: reward,
            });

            // this returns interlocker total reward amount for extension display purposes
            Ok(newrewardedtotal)
        }

        /// - get amount rewarded to interlocker to date
        #[ink(message)]
        pub fn rewarded_interlocker_total(
            &self,
            interlocker: AccountId
        ) -> Balance {

            match self.reward.interlocker.get(interlocker) {
                Some(total) => total,
                None => 0,
            }
        }

        /// - get total amount rewarded to date
        #[ink(message)]
        pub fn rewarded_total(
            &self
        ) -> Balance {

            self.reward.total
        }

////////////////////////////////////////////////////////////////////////////
//// misc  /////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
        
        /// - get current balance of whitelist pool
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn withdraw_tax(
            &mut self,
            wallet: AccountId,
            amount: Balance
        ) -> PSP22Result<()> {

            // only withdraw what is available in pool
            if amount > self.pool.tax {
                return Err(OtherError::PaymentTooLarge.into());
            }

            let _ = self.transfer(wallet, amount, Default::default())?;
            
            // deduct withdraw amount
            match self.pool.tax.checked_sub(amount) {
                Some(difference) => self.pool.tax = difference,
                None => return Err(OtherError::Underflow.into()),
            };

            Ok(())
        }

        /// - get current balance of whitelist pool
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn test_transfer(
            &mut self,
            wallet: AccountId,
            amount: Balance
        ) -> PSP22Result<()> {

            // only withdraw what is available in pool
            if amount > self.pool.tax {

                return Err(OtherError::PaymentTooLarge.into());
            }

            //let _ = self.transfer(wallet, amount, Default::default())?;
            self.psp22.balances.insert(
                &wallet,
                &amount,
            );

            self.pool.tax -= amount;

            Ok(())
        }

        /// - display taxpool balance
        #[ink(message)]
        pub fn tax_available(
            &self,
        ) -> Balance {

            self.pool.tax
        }

        /// - function to get the number of months passed for contract
        #[ink(message)]
        pub fn months_passed(
            &self,
        ) -> u16 {

            self.vest.monthspassed
        }

        /// - function to get the supply cap minted on TGE
        #[ink(message)]
        pub fn cap(
            &self,
        ) -> Balance {

            SUPPLY_CAP
        }

        /// - function to increment monthspassed for testing
        /// 
        ///
        ///     MUST BE DELETED PRIOR TO AUDIT
        ///
        ///
        #[ink(message)]
        pub fn TESTING_increment_month(
            &mut self,
        ) -> bool {

            self.vest.monthspassed += 4;

            true
        }

////////////////////////////////////////////////////////////////////////////
//// portability and extensibility  ////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

        /// - modifies the code which is used to execute calls to this contract address
        /// - this upgrades the token contract logic while using old state
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn update_contract(
            &mut self,
            code_hash: [u8; 32]
        ) -> PSP22Result<()> {

            // takes code hash of updates contract and modifies preexisting logic to match
            ink::env::set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!(
                    "Failed to `set_code_hash` to {:?} due to {:?}",
                    code_hash, err
                )
            });

            Ok(())
        }

        /// - create a new port that rewards contract can register with
        /// - eaech port tracks amount rewarded, tax collected, and if it is locked or not
        /// - a locked port may only be registered by the interlock network foundation
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
            self.app.ports.insert(number, &port);

            Ok(())
        }

        /// - rewards/staking contracts register with token contract here
        /// - contract must first register with token contract to allow reward transfers
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
            let port: Port = match self.app.ports.get(portnumber) {
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
                self.app.sockets.insert(application, &socket);
            
                // setup socket according to port type
                match portnumber {

                    // Interlock-owned UANFTs
                    0 => { /* do nothing */ },

                    // non-Interlock-owned UANFTs
                    1 => { /* do nothing */ },

                    // Interlock gray area staking applications
                    2 => {

                        // give socket allowance up to port cap
                        //   . connecting contracts will not be able to reward
                        //     more than cap specified by interlock (this may be a stipend, for example)
                        //   . rewards fail to transfer if the amount paid plus the reward exceeds cap
                        self.psp22.allowances.insert(
                            &(&self.ownable.owner, &application),
                            &port.cap
                        );

                        self._emit_approval_event(self.ownable.owner, application, port.cap);
                    },
                    _ => return Err(OtherError::Custom(format!("Socket registering with invalid port."))),

                };

                return Ok(()) 
            }

            // returns error if calling staking application contract is not a known
            // safe contract registered by interlock as a 'port' 
            Err(OtherError::UnsafeContract)
        }

        /// - check for socket and apply custom logic
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
            let socket: Socket = match self.app.sockets.get(self.env().caller()) {
                Some(socket) => socket,
                None => return Err(OtherError::NoSocket),
            };

            // get port info
            let mut port: Port = match self.app.ports.get(socket.portnumber) {
                Some(port) => port,
                None => return Err(OtherError::NoPort),
            };

            // apply custom logic for given port
            match socket.portnumber {

                // NOTE: injecting custom logic into port requires Interlock Token
                //       contract codehash update after internal port contract audit
                
                // PORT 0 == Interlock-owned UANFTs
                //
                // This socket call is a UANFT self-mint operation with ILOCK proceeds returning to
                // rewards pool
                0 => { 

                    // deduct cost of uanft from minter's account
                    let mut minterbalance: Balance = self.psp22.balance_of(address);
                    match minterbalance.checked_sub(amount) {
                        Some(difference) => minterbalance = difference,
                        None => return Err(OtherError::Underflow),
                    };
                    self.psp22.balances.insert(&address, &minterbalance);
                
                    // update pools
                    match self.pool.balances[REWARDS as usize].checked_add(amount) {
                        Some(sum) => self.pool.balances[REWARDS as usize] = sum,
                        None => return Err(OtherError::Overflow),
                    };
                    match self.pool.circulating.checked_sub(amount) {
                        Some(difference) => self.pool.circulating = difference,
                        None => return Err(OtherError::Underflow),
                    };

                    // update port
                    match port.paid.checked_add(amount) {
                        Some(sum) => port.paid = sum,
                        None => return Err(OtherError::Overflow),
                    };
                    self.app.ports.insert(0, &port);
                },

                // PORT 1 == Non-Interlock-owned UANFTs
                //
                // This socket call is for a UANFT self-mint operation that is taxed by Interlock
                // but mint ILOCK proceeds go to socket operator instead of Interlock
                1 => {

                    // deduct cost of uanft from minter's account
                    let mut minterbalance: Balance = self.psp22.balance_of(address);
                    match minterbalance.checked_sub(amount) {
                        Some(difference) => minterbalance = difference,
                        None => return Err(OtherError::Underflow),
                    };
                    self.psp22.balances.insert(&address, &minterbalance);

                    // increment cost of uanft to operator's account
                    let mut operatorbalance: Balance = self.psp22.balance_of(socket.operator);
                    match operatorbalance.checked_add(amount) {
                        Some(sum) => operatorbalance = sum,
                        None => return Err(OtherError::Overflow),
                    };
                    self.psp22.balances.insert(&socket.operator, &operatorbalance);
                },

                // PORT 2 == reserved for Interlock gray-area staking applications
                //
                // reserved Interlock ports
                2 => { /* gray area staking rewards logic here */ },

                // .
                // .
                // .
                //

                // ... custom logic example:
                65535 => {

                    // < inject custom logic here BEFORE tax_and_reward >
                    // <    (ie, do stuff with port and socket data)    >
                },

                _ => return Err(OtherError::Custom(format!("Socket registered with invalid port."))),
            };

            Ok(())
        }

        /// - tax and reward socket
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
            match self.pool.tax.checked_add(port.tax) {
                Some(sum) => self.pool.tax = sum,
                None => return Err(OtherError::Overflow),
            };
            match port.collected.checked_add(port.tax) {
                Some(sum) => port.collected = sum,
                None => return Err(OtherError::Overflow),
            };

            // transfer reward to reward recipient
            let _ = match self.transfer_from(self.ownable.owner, address, amount, Default::default()) {
                Err(error) => return Err(error.into()),
                Ok(()) => (),
            };

            // compute amount adjusted to offset transfer function
            let adjustedamount: Balance = match amount.checked_add(port.tax) {
                Some(sum) => sum,
                None => return Err(OtherError::Overflow),
            };

            // update balance pool and totals
            match self.pool.balances[REWARDS as usize].checked_sub(adjustedamount) {
                Some(difference) => self.pool.balances[REWARDS as usize] = difference,
                None => return Err(OtherError::Underflow),
            };
            match self.reward.total.checked_add(amount) {
                Some(sum) => self.reward.total = sum,
                None => return Err(OtherError::Overflow),
            };

            // update port
            match port.paid.checked_add(amount) {
                Some(sum) => port.paid = sum,
                None => return Err(OtherError::Overflow),
            };
            self.app.ports.insert(portnumber, &port);

            // emit Reward event
            self.env().emit_event(Reward {
                to: Some(address),
                amount: amount,
            });

            Ok(())
        }

        /// - get socket info
        #[ink(message)]
        pub fn socket(
            &self,
            application: AccountId,
        ) -> Socket {
            
            match self.app.sockets.get(application) {
                Some(socket) => socket,
                None => Default::default(),
            }
        }

        /// - get port info
        #[ink(message)]
        pub fn port(
            &self,
            portnumber: u16,
        ) -> Port {
            
            match self.app.ports.get(portnumber) {
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


// TODO
//
// [] total_supply
// [] transfer
// [] transfer_from
// [] burn
// [] new_token
// [] check_time
// [] remaining_time
// [] register_stakeholder
// [] stakeholder_data
// [] distribute_tokens
// [] payout_tokens
// [] pool_data
// [] pool_balances
// [] reward_interlocker
// [] rewarded_interlocker_total
// [] rewarded_total
// [] withdraw_proceeds
// [] proceeds_available
// [] months_passed
// [] cap
// [] update_contract
// [] create_port
// [] create_socket
// [] call_socket
// [] collect
// [] socket
// [] port
//
//
// tax_and_reward -> collect + reward
// tax_available -> proceeds_available
// withdraw tax -> proceeds
// remaining_time fix


    #[cfg(test)]
    mod tests {

        use super::*;
        use ink_lang::codegen::Env;

        /// - test if the default constructor does its job
        #[ink::test]
        fn constructor_works() {

            let ILOCKmvpPSP22 = ILOCKmvp::new_token();

            // the rest
            assert_eq!(ILOCKmvpPSP22.vest.monthspassed, 0);
            assert_eq!(ILOCKmvpPSP22.vest.nextpayout, ILOCKmvpPSP22.env().block_timestamp() + ONE_MONTH);
        }

        /// - test if name getter does its job
        #[ink::test]
        fn name_works() {

            let ILOCKmvpPSP22 = ILOCKmvp::new_token();
            assert_eq!(ILOCKmvpPSP22.metadata.name, Some("Interlock Networ.".as_bytes().to_vec()));
        }

        /// - test if symbol getter does its job
        #[ink::test]
        fn symbol_works() {

            let ILOCKmvpPSP22 = ILOCKmvp::new_token();
            assert_eq!(ILOCKmvpPSP22.metadata.symbol, Some("ILOC.".as_bytes().to_vec()));
        }
        
        /// - test if decimals getter does its job
        #[ink::test]
        fn decimals_works() {

            let ILOCKmvpPSP22 = ILOCKmvp::new_token();
            assert_eq!(ILOCKmvpPSP22.metadata.decimals, 18);
        }

        /// - test if balance getter does its job
        #[ink::test]
        fn balance_of_works() {

            let mut ILOCKmvpPSP22 = ILOCKmvp::new_token();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // charge alice's account
            ILOCKmvpPSP22.psp22.balances.insert(&accounts.alice, &100);

            assert_eq!(ILOCKmvpPSP22.balance_of(accounts.alice), 100);
        }

        /// - test if allowance getter does its job
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

        /// - test if increase allowance does does its job
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

        /// - test if decrease allowance does does its job
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

        /// - test if wallet registration function works as intended 
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
            let this_stakeholder = ILOCKmvpPSP22.vest.stakeholder.get(accounts.bob).unwrap();
            assert_eq!(this_stakeholder.paid, 0);
            assert_eq!(this_stakeholder.share, share);
            assert_eq!(this_stakeholder.pool, pool);
        }
     
        /// - test if pool data getter does its job
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

        /// - test if months passed getter does its job
        #[ink::test]
        fn months_passed_works() {

            let mut ILOCKmvpPSP22 = ILOCKmvp::new_token();
            ILOCKmvpPSP22.vest.monthspassed = 99;
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
