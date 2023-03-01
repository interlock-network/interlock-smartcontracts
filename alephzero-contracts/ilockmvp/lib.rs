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
//  To build docs:
//
//      cargo +nightly doc --no-deps --document-private-items --open
//
// To reroute docs in Github
//
//      echo "<meta http-equiv=\"refresh\" content=\"0; url=build_wheel\">" >
//      target/doc/index.html;
//      cp -r target/doc ./docs

/*
#![doc(
    html_logo_url = "https://github.com/interlock-network/interlock-brand/blob/main/favicons/Interlock_Blue_BlackCircle128px.png",
    html_favicon_url = "https://github.com/interlock-network/interlock-brand/blob/main/favicons/Interlock_Blue_BlackCircle16px.png",
)]
*/

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
        proceeds: Balance,

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
    #[derive(scale::Encode, scale::Decode, Clone, Copy)]
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
        /// Returned if checked divide errors out
        DivError,
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
        
        ///
        /// - override default total_supply getter
        /// - total supply reflects token in circulation
        #[ink(message)]
        fn total_supply(&self) -> Balance {

            // revert, testing set code hash
            self.pool.circulating
        }

        ///
        /// - Override default transfer doer
        /// - Transfer from owner increases total circulating supply.
        /// - Transfer to owner decreases total circulating supply.
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
        pub fn remaining_time(
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
        pub fn withdraw_proceeds(
            &mut self,
            wallet: AccountId,
            amount: Balance
        ) -> PSP22Result<()> {

            // only withdraw what is available in pool
            if amount > self.pool.proceeds {
                return Err(OtherError::PaymentTooLarge.into());
            }

            let _ = self.transfer(wallet, amount, Default::default())?;
            
            // deduct withdraw amount
            match self.pool.proceeds.checked_sub(amount) {
                Some(difference) => self.pool.proceeds = difference,
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


            // emit Reward event
            self.env().emit_event(Reward {
                to: Some(wallet),
                amount: amount,
            });


            Ok(())
        }

        /// - display taxpool balance
        #[ink(message)]
        pub fn proceeds_available(
            &self,
        ) -> Balance {

            self.pool.proceeds
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
        #[openbrush::modifiers(only_owner)]
        pub fn TESTING_increment_month(
            &mut self,
        ) -> OtherResult<bool> {

            self.vest.monthspassed += 1;

            Ok(true)
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
                    self.app.ports.insert(socket.portnumber, &port);
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

                    let adjustedamount: Balance = self.tax_port_transfer(socket, port, amount)?;

                    // increment cost of uanft to operator's account
                    let mut operatorbalance: Balance = self.psp22.balance_of(socket.operator);
                    match operatorbalance.checked_add(adjustedamount) {
                        Some(sum) => operatorbalance = sum,
                        None => return Err(OtherError::Overflow),
                    };
                    self.psp22.balances.insert(&socket.operator, &operatorbalance);
                    
                    // emit Transfer event, uanft transfer
                    self.env().emit_event(Transfer {
                        from: Some(address),
                        to: Some(socket.operator),
                        amount: adjustedamount,
                    });

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

        /// - tax and reward transfer between socket calling address and socket operator
        pub fn tax_port_transfer(
            &mut self,
            socket: Socket,
            mut port: Port,
            amount: Balance,
        ) -> OtherResult<Balance> {

            // compute tax - tax number is in centipercent, 0.01% ==> 100% = 1 & 0.01% = 10_000
            //
            // a tax of 0.01% is amount/10_000
            let tax: Balance = match amount.checked_div(port.tax as Balance) {
                Some(quotient) => quotient,
                None => return Err(OtherError::DivError),
            };

            // update proceeds pool and total circulation
            match self.pool.proceeds.checked_add(tax) {
                Some(sum) => self.pool.proceeds = sum,
                None => return Err(OtherError::Overflow),
            };
            match port.collected.checked_add(tax) {
                Some(sum) => port.collected = sum,
                None => return Err(OtherError::Overflow),
            };
            match self.pool.circulating.checked_sub(tax) {
                Some(difference) => self.pool.circulating = difference,
                None => return Err(OtherError::Underflow),
            };

            // update port
            self.app.ports.insert(socket.portnumber, &port);
                    
            // emit Transfer event, operator to ILOCK proceeds pool
            self.env().emit_event(Transfer {
                from: Some(socket.operator),
                to: Some(self.ownable.owner),
                amount: tax,
            });

            // return adjusted amount
            Ok(amount - tax)
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
    

////////////////////////////////////////////////////////////////////////////
//// tests /////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

        /// - Test Events.
        #[ink(message)]
        pub fn test_events(
            &self,
            alice: AccountId,
            bob: AccountId,
        ) -> () {

            // emit Transfer event
            Self::env().emit_event(Transfer {
                from: Some(alice),
                to: Some(bob),
                amount: 1000,
            });
            // emit Approval event
            Self::env().emit_event(Approval {
                owner: Some(alice),
                spender: Some(bob),
                amount: 1000,
            });
            // emit Reward event
            Self::env().emit_event(Reward {
                to: Some(alice),
                amount: 1000,
            });
        }
    } // END OF ILOCKmvp IMPL BLOCK

//
// INCOMPLETE
//
// . To view debug prints and assertion failures run test via:
//
//      cargo +nightly test --features e2e-tests -- --show-output
//
// . To view debug for specific method run test via:
//
//      cargo nightly+ test <test_function_here> -- --nocapture
//
// . To run end-to-end tests, first make sure you have the substrate
//   dev node capabilities installed via:
//
//      cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git
//
//   Then run the node:
//
//      substrate-contracts-node
//


// TESTTODO
// in order of appearance
//
// [x] hunit_total_supply (tested in new_token
// [] he2e_transfer
// [] se2e_transfer
// [] he2e_transfer_from
// [] se2e_transfer_from
// [] he2e_burn
// [] se2e_burn
// [x] hunit_new_token (no sad, returns only Self)
// [] hunit_check_time
// [] sunit_check_time
// [] hunit_remaining_time
// [x] hunit_register_stakeholder
// [] sunit_register_stakeholder . ..... add sad case where share is greater than pool total?
// [] hunit_stakeholder_data
// [] he2e_distribute_tokens  <-- this is to check that the vesting schedule works...
// [] he2e_payout_tokens                 ...month passage is artificial here, without 
// [] se2e_payout_tokens                    advancing blocks.
// [x] hunit_pool_data
// [] hunit_pool_balances
// [] he2e_reward_interlocker
// [] se2e_reward_interlocker
// [] hunit_rewarded_interlocker_total
// [] hunit_rewarded_total
// [] he2e_withdraw_proceeds
// [] sunit_withdraw_proceeds
// [] hunit_proceeds_available
// [x] hunit_months_passed   <-- checked during new_token()
// [x] hunit_cap             <-- checked during new_token()
// [] hunit_update_contract
// [] sunit_update_contract
// [] hunit_create_port
//      [] hunit_port        <-- perform with create_port()
// [] ** he2e_create_socket     \
// [] ** se2e_create_socket     ]---- these must be performed from generic port
// [] ** he2e_call_socket       ]     or from the uanft contract's self minting message
// [] ** se2e_call_socket       /
// [] hunit_socket    
// [] hunit_tax_port_transfer
// [] sunit_tax_port_transfer
//


////////////////////////////////////////////////////////////////////////////
//// end to end ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
    
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {


        use super::*;
        use ink_e2e::{
            build_message,
            CallResult,
        };
        use openbrush::contracts::psp22::psp22_external::PSP22;
        use ink::primitives::{
            Clear,
            Hash,
        };
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        ///
        /// - Test if token distribution works as intended per vesting schedule.
        /// - Include register_stakeholder().
        /// - Include distribute_tokens().
        /// - Include check_time().
        #[ink_e2e::test]
        async fn vesting_schedule_works(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            // fire up contract
            let constructor = ILOCKmvpRef::new_token();
            let contract_acct_id = client
                .instantiate("ilockmvp", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // register generic stakeholder
            let stakeholder_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            let stakeholder_share = 1_000_000_000;

            // prepare messages
            let distribute_tokens_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.distribute_tokens(stakeholder_account.clone()));

            // iterate through one vesting schedule
            for month in 0..(POOLS[0].vests + POOLS[0].cliffs + 1) {

                let mut balance_of_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                    .call(|contract| contract.balance_of(stakeholder_account.clone()));
                let mut stakeholder_data_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                    .call(|contract| contract.stakeholder_data(stakeholder_account.clone()));
                let distribute_tokens_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                    .call(|contract| contract.distribute_tokens(stakeholder_account.clone()));
                let mut increment_month_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                    .call(|contract| contract.TESTING_increment_month());
                let mut months_passed_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                    .call(|contract| contract.months_passed());

                println!("{}", month);
/*
                let mut months_passed = client 
                    .call(&ink_e2e::alice(), months_passed_msg, 0, None)
                    .await
                    .return_value();
                assert_eq!(months_passed, month);
               
                match client
                    .call(&ink_e2e::alice(), distribute_tokens_msg, 0, None)
                    .await
                    .return_value() {
                        () => (),
                        OtherError::PayoutTooEarly => (),
                }; */
         /*       let mut stakeholder_data = client
                    .call(&ink_e2e::alice(), stakeholder_data_msg, 0, None)
                    .await;

                let mut increment_month_res = client 
                    .call(&ink_e2e::alice(), increment_month_msg, 0, None)
                    .await;

*/

            }
            
            Ok(())
        }

        ///
        /// - Test if customized transfer function works correctly.
        /// - When transfer from contract owner, circulating supply increases.
        /// - When transfer to contract owner, circulating supply decreases
        /// and rewards pool increases/
        #[ink_e2e::test]
        async fn transfer_works(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

            let constructor = ILOCKmvpRef::new_token();
            let contract_acct_id = client
                .instantiate("ilockmvp", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // alice is contract owner
            // transfers 1000 ILOCK from alice to bob and check for resulting Transfer event
            let alice_transfer_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.transfer(bob_account.clone(), 1000, Vec::new()));
            match client
                .call(&ink_e2e::alice(), alice_transfer_msg, 0, None)
                .await {
                Ok(result) => {
                    let mut transfer_present: bool = false;
                    for event in result.events.iter() {
                        let bytes_text: String = String::from_utf8_lossy(
                                                 event.expect("bad event").bytes()).to_string();
                        if bytes_text.contains("ILOCKmvp::Transfer") {
                            transfer_present = true;
                            break;
                        };
                    }
                    if !transfer_present {panic!("Transfer event not present")};
                },
                Err(error) => panic!("transfer calling error: {:?}", error),
            };
            
            // checks that bob has expected resulting balance
            let bob_balance_of_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.balance_of(bob_account.clone()));
            let bob_balance = client
                .call_dry_run(&ink_e2e::bob(), &bob_balance_of_msg, 0, None)
                .await
                .return_value();
            assert_eq!(0 + 1000, bob_balance);

            // checks that alice has expected resulting balance
            let alice_balance_of_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.balance_of(alice_account.clone()));
            let alice_balance = client
                .call_dry_run(&ink_e2e::alice(), &alice_balance_of_msg, 0, None)
                .await
                .return_value();
            assert_eq!(SUPPLY_CAP - 1000, alice_balance);

            // checks that circulating supply increased appropriately
            let total_supply_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.total_supply());
            let mut total_supply = client
                .call_dry_run(&ink_e2e::alice(), &total_supply_msg, 0, None)
                .await
                .return_value();
            assert_eq!(0 + 1000, total_supply);

            // transfers 500 ILOCK from bob to alice and check for resulting Transfer event
            let bob_transfer_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.transfer(alice_account.clone(), 500, Vec::new()));
            let _result = client
                .call(&ink_e2e::bob(), bob_transfer_msg, 0, None)
                .await;
               
            // checks that circulating supply decreased appropriately
            total_supply = client
                .call_dry_run(&ink_e2e::alice(), &total_supply_msg, 0, None)
                .await
                .return_value();
            assert_eq!(1000 - 500, total_supply);

            // check that rewards supply increased appropriately
            let rewards_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.pool_balance(REWARDS));
            let rewards_balance = client
                .call_dry_run(&ink_e2e::alice(), &rewards_balance_msg, 0, None)
                .await
                .return_value();
            assert_eq!(POOLS[REWARDS as usize].tokens * DECIMALS_POWER10 + 500, rewards_balance.1);

            Ok(())
        }

        /// - test if rewarding functionality works
        #[ink_e2e::test]
        async fn reward_interlocker_works(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

            let constructor = ILOCKmvpRef::new_token();
            let contract_acct_id = client
                .instantiate("ilockmvp", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // rewards 1000 ILOCK to bob
            let alice_test_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.test_transfer(bob_account.clone(), 1000));
            let _alice_reward = client
                .call(&ink_e2e::alice(), alice_test_msg, 0, None)
                .await;

            // Checks that bob has expected resulting balance
            let bob_balance_of_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.balance_of(bob_account.clone()));
            let bob_balance = client
                .call_dry_run(&ink_e2e::bob(), &bob_balance_of_msg, 0, None)
                .await
                .return_value();
            assert_eq!(1000, bob_balance);

            // Transfer event triggered during initial construction.
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();

            Ok(())
        }
    }

////////////////////////////////////////////////////////////////////////////
//// unit tests ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

    #[cfg(test)]
    mod tests {

        use ink::primitives::{
            Clear,
            Hash,
        };

        use super::*;

        /// - test if the default constructor does its job
        /// - and check months_passed()
        /// - and check cap()
        #[ink::test]
        fn new_token_works() {

            let ILOCKmvpPSP22 = ILOCKmvp::new_token();

            assert_eq!(ILOCKmvpPSP22.vest.monthspassed, ILOCKmvpPSP22.months_passed());
            assert_eq!(ILOCKmvpPSP22.vest.nextpayout, ILOCKmvpPSP22.env().block_timestamp() + ONE_MONTH);
            assert_eq!(ILOCKmvpPSP22.total_supply(), 0);
            assert_eq!(ILOCKmvpPSP22.metadata.name, Some("Interlock Network".as_bytes().to_vec()));
            assert_eq!(ILOCKmvpPSP22.metadata.symbol, Some("ILOCK".as_bytes().to_vec()));
            assert_eq!(ILOCKmvpPSP22.metadata.decimals, 18);

            // this checks that token numbers have been entered accurately into POOLS PoolData
            let mut total_tokens: u128 = 0;
            for pool in 0..POOL_COUNT {

                total_tokens += POOLS[pool].tokens * DECIMALS_POWER10;
            }
            assert_eq!(total_tokens, ILOCKmvpPSP22.cap());
            assert_eq!(ILOCKmvpPSP22.ownable.owner, ILOCKmvpPSP22.env().caller());
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

////////////////////////////////////////////////////////////////////////////
//// test events emit properly in general //////////////////////////////////
////////////////////////////////////////////////////////////////////////////

        ///
        /// - Test events emit in general.
        /// - Due to fact that openbrush invokes 'cross contract calls',
        /// it is not possible to run standard unit tests for any functions
        /// that include PSP22 standard messages, so we need to perform e2e tests in general.
        /// - For now, the most we can do is verify within e2e tests that a
        /// particular event occured.
        /// - There is no clear way to catch events in their entirety within an
        /// e2e test (ie, the topics and values).
        /// - To make up for this, in this non-e2e test we verify that *when* one of the
        /// three events are emitted, their topics are indeed accurate in general.
        /// - Capturing complete events within the e2e tests is an active issue, #225
        #[ink::test]
        fn test_events_work() {

            let mut ILOCKmvpPSP22 = ILOCKmvp::new_token();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ILOCKmvpPSP22.test_events(accounts.alice.clone(), accounts.bob.clone());

            // Transfer event triggered during initial construction.
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();

            assert_event(
                "Transfer",
                &emitted_events[1],
                Some(accounts.alice.clone()),
                Some(accounts.bob.clone()),
                1000,
            );
            assert_event(
                "Approval",
                &emitted_events[2],
                Some(accounts.alice.clone()),
                Some(accounts.bob.clone()),
                1000,
            );
            assert_event(
                "Reward",
                &emitted_events[3],
                Some(accounts.alice.clone()),
                Some(accounts.alice.clone()), // <- not used
                1000,
            );
        }


        ///
        /// Serves in test for the three emitted events.
        /// Taken from Ink! examples repo.
        ///
        /// For calculating the event topic hash.
        struct PrefixedValue<'a, 'b, T> {
            pub prefix: &'a [u8],
            pub value: &'b T,
        }

        ///
        /// Serves in test for the three emitted events.
        /// Taken from Ink! examples repo.
        ///
        /// Use this implementation to encode and decode events.
        impl<X> scale::Encode for PrefixedValue<'_, '_, X>
        where
            X: scale::Encode,
        {
            #[inline]
            fn size_hint(&self) -> usize {
                self.prefix.size_hint() + self.value.size_hint()
            }

            #[inline]
            fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
                self.prefix.encode_to(dest);
                self.value.encode_to(dest);
            }
        }
        type Event = <ILOCKmvp as ::ink::reflect::ContractEventBase>::Type;

        ///
        /// Serves in test for the three emitted events.
        /// Taken from Ink! examples repo, modified to check three Event types.
        ///
        /// This function compares emitted events against expectations.
        fn assert_event(
            kind: &str,
            event: &ink::env::test::EmittedEvent,
            expected_A: Option<AccountId>,
            expected_B: Option<AccountId>,
            expected_C: Balance,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");

            let mut expected_topics = Vec::new();
            match kind {

                "Transfer" => {
                    if let Event::Transfer(Transfer { from, to, amount }) = decoded_event {
                        assert_eq!(from, expected_A, "encountered invalid Transfer.from");
                        assert_eq!(to, expected_B, "encountered invalid Transfer.to");
                        assert_eq!(amount, expected_C, "encountered invalid Transfer.amount");

                        let expected_topics = vec![
                            encoded_into_hash(&PrefixedValue {
                                value: b"ILOCKmvp::Transfer",
                                prefix: b"",
                            }),
                            encoded_into_hash(&PrefixedValue {
                                prefix: b"ILOCKmvp::Transfer::from",
                                value: &expected_A,
                            }),
                            encoded_into_hash(&PrefixedValue {
                                prefix: b"ILOCKmvp::Transfer::to",
                                value: &expected_B,
                            }),
                            encoded_into_hash(&PrefixedValue {
                                prefix: b"ILOCKmvp::Transfer::amount",
                                value: &expected_C,
                            }),
                        ];
                    } else {
                        panic!("expected valid Transfer event");
                    }
                },

                "Approval" => {
                    if let Event::Approval(Approval { owner, spender, amount }) = decoded_event {
                        assert_eq!(owner, expected_A, "encountered invalid Approval.owner");
                        assert_eq!(spender, expected_B, "encountered invalid Approval.spender");
                        assert_eq!(amount, expected_C, "encountered invalid Approval.amount");

                        let expected_topics = vec![
                            encoded_into_hash(&PrefixedValue {
                                value: b"ILOCKmvp::Approval",
                                prefix: b"",
                            }),
                            encoded_into_hash(&PrefixedValue {
                                prefix: b"ILOCKmvp::Approval::owner",
                                value: &expected_A,
                            }),
                            encoded_into_hash(&PrefixedValue {
                                prefix: b"ILOCKmvp::Approval::spender",
                                value: &expected_B,
                            }),
                            encoded_into_hash(&PrefixedValue {
                                prefix: b"ILOCKmvp::Approval::amount",
                                value: &expected_C,
                            }),
                        ];
                    } else {
                        panic!("expected valid Approval event");
                    }
                },

                "Reward" => {
                    if let Event::Reward(Reward { to, amount }) = decoded_event {
                        assert_eq!(to, expected_A, "encountered invalid Reward.to");
                        assert_eq!(amount, expected_C, "encountered invalid Reward.amount");

                        let expected_topics = vec![
                            encoded_into_hash(&PrefixedValue {
                                value: b"ILOCKmvp::Approval",
                                prefix: b"",
                            }),
                            encoded_into_hash(&PrefixedValue {
                                prefix: b"ILOCKmvp::Approval::to",
                                value: &expected_A,
                            }),
                            encoded_into_hash(&PrefixedValue {
                                prefix: b"ILOCKmvp::Approval::amount",
                                value: &expected_C,
                            }),
                        ];
                    } else {
                        panic!("expected valid Reward event");
                    }
                },
                &_ => (),
            };

            let topics = event.topics.clone();
            for (n, (actual_topic, expected_topic)) in
                topics.iter().zip(expected_topics).enumerate()
            {
                let mut topic_hash = Hash::CLEAR_HASH;
                let len = actual_topic.len();
                topic_hash.as_mut()[0..len].copy_from_slice(&actual_topic[0..len]);

                assert_eq!(
                    topic_hash, expected_topic,
                    "encountered invalid topic at {n}"
                );
            }
        }

        ///
        /// Serves in test for the three emitted events.
        /// Taken from Ink! examples repo.
        ///
        /// This function takes hash of encoded topic data
        fn encoded_into_hash<T>(entity: &T) -> Hash
        where
            T: scale::Encode,
        {
            use ink::{
                env::hash::{
                    Blake2x256,
                    CryptoHash,
                    HashOutput,
                },
                primitives::Clear,
            };

            let mut result = Hash::CLEAR_HASH;
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

