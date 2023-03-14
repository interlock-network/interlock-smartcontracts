//!
//! # INTERLOCK NETWORK - UNIVERSAL ACCESS NFT
//!
//! This is a PSP34 NFT in compatible with Art Zero marketplace and capable of managing user access
//! credentials on the blockchain using a strategy similar to two-factor-authentication (2FA).
//!
//! Build needs cargo-contract version 2.0.0:
//!
//! -     cargo install cargo-contract --force --version 2.0.0
//!
//! To build:
//!
//! -     cargo +nightly contract build
//!
//!  To build docs:
//!
//! -     cargo +nightly doc --no-deps --document-private-items --open
//!
//! To reroute docs in Github:
//!
//! -     echo "<meta http-equiv=\"refresh\" content=\"0; url=build_wheel\">" >
//! -     target/doc/index.html;
//! -     cp -r target/doc ./docs
//!

#![doc(
    html_logo_url = "https://uploads-ssl.webflow.com/6293b370c2da3eda80121e92/6293d7cffa42ae33001294d1_interlock-visual-hero.png",
    html_favicon_url = "https://uploads-ssl.webflow.com/6293b370c2da3eda80121e92/6293d7cffa42ae33001294d1_interlock-visual-hero.png",
)]

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use self::psp34_nft::{Psp34Nft, Psp34NftRef};

#[openbrush::contract]
pub mod psp34_nft {

    // ink 4 imports
    use ink::{
        codegen::{Env, EmitEvent},
        storage::Mapping,
        prelude::{
            string::{String, ToString},
            vec::Vec,
            format,
        },
        reflect::ContractEventBase,
    };

    // openbrush 3 imports
    use openbrush::{
        traits::Storage,
        modifiers,
        contracts::{
            ownable::*,
            psp34::{
                extensions::{
                    enumerable::*,
                    metadata::*,
                    burnable::*,
                },
                Internal,
                PSP34Error,
            },
            psp22::psp22_external::PSP22,
        },
    };

    // we use these to interface as uanft application
    // with the Interlock Network PSP22 contract
    use ilockmvp::{
        ilockmvp::OtherError,
        ILOCKmvpRef,
    };

    #[openbrush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Metadata;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InvalidInput,
        Custom(String),
        OwnableError(OwnableError),
        PSP34Error(PSP34Error),
    }
    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            Error::OwnableError(ownable)
        }
    }
    impl From<PSP34Error> for Error {
        fn from(error: PSP34Error) -> Self {
            Error::PSP34Error(error)
        }
    }


    /// This is a type wrapper to implement Default method
    /// on AccountId type. Ink 4 stable eliminated AccountId Default
    /// (which was zero address, that has known private key)
    /// ...we only really need this because Openbrush contract
    ///    relies on deriving Default for contract storage, and
    ///    our AccesData struct contains AccountId.
    #[derive(scale::Encode, scale::Decode, Clone, Debug)]
    #[cfg_attr(
        feature = "std",
        derive(
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout,
        )
    )]
    pub struct AccountID {
        address: AccountId,
    }
    impl Default for AccountID {
        fn default() -> AccountID {
            AccountID {
                address: AccountId::from([1_u8;32]),
            }
        }
    }

    /// - This is upgradable storage for the access features for this
    /// universal access nft contract.
    pub const ACCESS_KEY: u32 = openbrush::storage_unique_key!(AccessData);
    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(ACCESS_KEY)]
    pub struct AccessData {

        /// uanft token cap
        pub cap: u64,

        /// nft sale price in ILOCK (or other) PSP22 token
        pub nft_psp22price: Balance,

        /// - Collections contains information about which uanft IDs a particular
        /// address holds.
        /// - This in part is important because it provides information
        /// about how many different access credential instances a particular wallet
        /// owner has for a given access usecase.
        ///
        /// collections:         user accress -> vector of uanft IDs in collection
        pub collections: Mapping<AccountId, Vec<Id>>,

        /// - Credentials contains a SHA256 (or other) hashed secret and uanft ID for said
        /// secret, one pair per user identifing (eg username) SHA256 hash.
        /// - This is important because it provides a means of verifying possession
        /// of secret, and for which uanft this owner has access to for those
        /// given credentials.
        ///
        /// credentials:         username hash -> (password hash, uanft ID)
        pub credentials: Mapping<Hash, (Hash, Id)>,

        /// - Userhashes contains information about which identifying credential hash
        /// a given uanft commands. this is important because on transfer event
        /// we need to revoke access to particular user identifying hash, but transfer
        /// events by PSP34 standard do not include the identifying information needed 
        /// revoke access for a particulare credential pair.
        ///
        /// userhashes:         uanft ID - > username hash
        pub userhashes: Mapping<Id, Hash>,

        /// - This is to expand storage related to this uanft's access functionality.
        pub _reserved: Option<()>
    }

    /// - This is upgradable storage for the features that allow this universal
    /// access nft contract to connect as an application to the ILOCK (or other)
    /// PSP22 contract the application socket abstraction.
    pub const APP_KEY: u32 = openbrush::storage_unique_key!(AppData);
    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(APP_KEY)]
    pub struct AppData {

        /// - This is PSP22 token contract that this uanft application connects to via socket.
        /// - This is used for self-minting purposes, which means non-owner can
        /// mint in excange for PSP22 token (ILOCK in this case) without needing
        /// to rely on a transaction relay server off-chain.
        pub token_instance: ILOCKmvpRef,

        /// - This is address that manages this uanft contract and receives ILOCK
        /// (or other) PSP22 token for self-mint transactions.
        pub operator: AccountID,

        /// - This is to expand storage related to this uanft application functionality.
        pub _reserved: Option<()>
    }
    /// - This is the port number for this type of uanft application socket connections to ILOCK (or other)
    /// PSP22 token contract.
    /// - PORT 0 designates uanft contracts owned by Interlock Network.
    /// - This port is locked by default (only Interlock Network may connect this uanft
    /// contract via socket to ILOCK PSP22 contrac.
    pub const PORT: u16 = 0;

    /// - Main contract storage.
    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Psp34Nft {

        /// - Openbrush PSP34 storage fields.
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,

        /// - Openbrush metadata extension storage fields.
        #[storage_field]
        metadata: metadata::Data,

        /// - Openbrush ownable extension storage fields.
        #[storage_field]
        ownable: ownable::Data,

        /// - Universal access NFT storage fields.
        #[storage_field]
        access: AccessData,

        /// - Storage fields related to the UANFT as an application for the ILOCK PSP22 contract.
        #[storage_field]
        app: AppData,

        /// - Art zero storage fields.
        last_token_id: u64,
        attribute_count: u32,
        attribute_names: Mapping<u32, Vec<u8>>,
        locked_tokens: Mapping<Id, bool>,
        locked_token_count: u64,
        is_attribute: Mapping<String, bool>,

    }

    /// - Specify transfer event.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        id: Id,
    }

    /// - Specify approval event.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        id: Id,
        approved: bool,
    }

    /// - Needed for Openbrush internal event emission implementations.
    pub type Event = <Psp34Nft as ContractEventBase>::Type;

    impl Internal for Psp34Nft {

        /// - Impliment Transfer emit event because Openbrush doesn't.
        fn _emit_transfer_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _id: Id,
        ) {
            Psp34Nft::emit_event(
                self.env(),
                Event::Transfer(Transfer {
                    from: _from,
                    to: _to,
                    id: _id,
                }),
            );
        }

        /// - Impliment Approval emit event because Openbrush doesn't.
        fn _emit_approval_event(
            &self,
            _from: AccountId,
            _to: AccountId,
            _id: Option<Id>,
            _approved: bool,
        ) {
            Psp34Nft::emit_event(
                self.env(),
                Event::Approval(Approval {
                    from: Some(_from),
                    to: Some(_to),
                    id: _id.unwrap(),
                    approved: _approved,
                }),
            );
        }
    }

    impl PSP34 for Psp34Nft {

        /// - Override transfer function to revoke access credentials if existent.
        /// - This also updates collection.
        #[ink(message)]
        fn transfer(
            &mut self,
            to: AccountId,
            id: Id,
            data: Vec<u8>
        ) -> Result<(), PSP34Error> {

            // transfer
            let from = self.env().caller();
            let _ = self._transfer_token(to, id.clone(), data)?;

            // revoke access if uanft registered to prior owner
            match self.access.userhashes.get(id.clone()) {
                
                // aunft registered by prior owner
                Some(hash) => {
                    self.access.credentials.remove(hash);
                    self.access.userhashes.remove(id.clone());
                },

                // aunft never registered by prior owner
                None => (),
            };

            // update sender's collection
            let mut from_collection = match self.access.collections.get(from) {
                Some(collection) => collection,
                None => return Err(PSP34Error::Custom(
                        format!("No collection, fatal error").into_bytes())),
            };
            let index = match from_collection.iter().position(|element| *element == id) {
                Some(index) => index,
                None => return Err(PSP34Error::Custom(
                        format!("token not in collection").into_bytes())),
            };
            from_collection.remove(index);
            self.access.collections.insert(from, &from_collection);

            // update recipient's collection
            let mut to_collection = match self.access.collections.get(to) {
                Some(collection) => collection,
                None => Vec::new(),
            };
            to_collection.push(id);
            self.access.collections.insert(to, &to_collection);

            Ok(())
        }
    }

    impl Ownable for Psp34Nft {}
    impl PSP34Metadata for Psp34Nft {}
    impl PSP34Enumerable for Psp34Nft {}

    impl PSP34Burnable for Psp34Nft {

        /// - Art Zero message.
        ///
        #[ink(message)]
        fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            let caller = self.env().caller();
            let token_owner = self.owner_of(id.clone()).unwrap();
            if token_owner != account {
                return Err(PSP34Error::Custom(String::from("not token owner").into_bytes()))
            }

            let allowance = self.allowance(account,caller,Some(id.clone()));

            if caller == account || allowance {
                self.locked_tokens.remove(&id);
                self.locked_token_count = self.locked_token_count.checked_sub(1).unwrap();
                self._burn_from(account, id)
            } else{
                Err(PSP34Error::Custom(String::from("caller is not token owner or approved").into_bytes()))
            }
        }
    }

    /// - Art Zero trait definitions.
    ///
    #[openbrush::trait_definition]
    pub trait Psp34Traits {

        #[ink(message)]
        fn get_last_token_id(&self) -> u64;

        #[ink(message)]
        fn lock(&mut self, token_id: Id) -> Result<(), Error>;

        #[ink(message)]
        fn is_locked_nft(&self, token_id: Id) -> bool;

        #[ink(message)]
        fn get_locked_token_count(&self) -> u64;

        #[ink(message)]
        fn set_base_uri(&mut self, uri: String) -> Result<(), Error>;

        #[ink(message)]
        fn set_multiple_attributes(&mut self, token_id: Id, metadata: Vec<(String, String)>) -> Result<(), Error>;

        #[ink(message)]
        fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String>;
        
        #[ink(message)]
        fn get_attribute_count(&self) -> u32;
        
        #[ink(message)]
        fn get_attribute_name(&self, index: u32) -> String;
        
        #[ink(message)]
        fn token_uri(&self, token_id: u64) -> String;
        
        #[ink(message)]
        fn get_owner(&self) -> AccountId;
    }

    impl Psp34Nft {

        /// - Function for internal _emit_event implementations.
        pub fn emit_event<EE: EmitEvent<Self>>(emitter: EE, event: Event) {
            emitter.emit_event(event);
        }

        /// - UANFT contract constructor.
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            class: String,
            cap: u64,
            price: Balance,
            token_address: AccountId,
        ) -> Self {
            
            // create the contract
            let mut contract = Self::default();
                
            // set attributes
            contract._set_attribute(
                Id::U8(0),
                String::from("name").into_bytes(),
                name.into_bytes(),
            );
            contract._set_attribute(
                Id::U8(0),
                String::from("symbol").into_bytes(),
                symbol.into_bytes(),
            );
            contract._set_attribute(
                Id::U8(0),
                String::from("class").into_bytes(),
                class.into_bytes(),
            );

            // assign caller as owner
            contract._init_with_owner(contract.env().caller());

            // create a reference to the deployed PSP22 ILOCK token contract
            contract.app.token_instance = ink::env::call::FromAccountId::from_account_id(token_address);
            contract.app.operator.address = Self::env().caller();

            // set cap
            contract.access.cap = cap;

            // set nft price in PSP22 token
            contract.access.nft_psp22price = price;

            contract
        }

        /// - This generic mint function is for Art Zero interface.
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(
            &mut self,
        ) -> Result<(), Error> {

            let caller = self.env().caller();

            // next token id
            // (overflow impossible due to cap check)
            self.last_token_id += 1;

            // make sure cap is not surpassed
            if self.last_token_id >= self.access.cap {
                return Err(Error::Custom(
                       format!("The NFT cap of {:?} has been met. Cannot mint.", self.access.cap)))
            }

            // if cap not surpassed, mint next id
            let _ = self._mint_to(caller, Id::U64(self.last_token_id))?;

            // get nft collection of recipient if already holding
            let mut collection = match self.access.collections.get(caller) {
                Some(collection) => collection,
                None => Vec::new(),
            };

            // add id to recipient's nft collection
            collection.push(psp34::Id::U64(self.last_token_id));
            self.access.collections.insert(caller, &collection);

            Ok(())
        }

        /// - This mints a universal access nft by Interlock Network to specific recipient.
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint_to(
            &mut self,
            recipient: AccountId,
        ) -> Result<(), Error> {

            // next token id
            self.last_token_id += 1;

            // make sure cap is not surpassed
            if self.last_token_id >= self.access.cap {
                return Err(Error::Custom(
                       format!("The NFT cap of {:?} has been met. Cannot mint.", self.access.cap)))
            }

            // if cap not surpassed, mint next id
            let _ = self._mint_to(recipient, psp34::Id::U64(self.last_token_id))?;

            // get nft collection of recipient if already holding
            let mut collection = match self.access.collections.get(recipient) {
                Some(collection) => collection,
                None => Vec::new(),
            };

            // add id to recipient's nft collection
            collection.push(psp34::Id::U64(self.last_token_id));
            self.access.collections.insert(recipient, &collection);

            Ok(())
        }

        /// - This mints a universal access nft to caller's self at token_price in terms of PSP22 token.
        #[ink(message)]
        pub fn self_mint(
            &mut self,
            price: Balance,
        ) -> Result<(), Error> {

            // next token id
            self.last_token_id += 1;

            // mint recipient
            let minter: AccountId = self.env().caller();

            // make sure cap is not surpassed
            if self.last_token_id >= self.access.cap {
                return Err(Error::Custom(
                       format!("The NFT cap of {:?} has been met. Cannot mint.", self.access.cap)))
            }

            // make sure asking price matches nft_psp22price
            // ...this is to ensure that contract owner doesn't hike up token price between the
            //    time somebody checks the price, and the time that somebody submits tx to
            //    self-mint for that given price
            if self.access.nft_psp22price > price {
                return Err(Error::Custom(
                       format!("Current NFT price greater than agreed sale price of {:?}.", price)))
            }
            
            // make sure mint recipient can afford the PSP22 token price
            let recipient_balance: Balance = self.app.token_instance.balance_of(minter);
            if recipient_balance < price {
                return Err(Error::Custom(
                       format!("Minter cannot affort NFT at current price of {:?}.", price)))
            }

            // if can afford, initiate PSP22 token transfer to contract operator now
            let _ = self.call_socket(minter, price, Vec::new());

            // mint next id
            let _ = self._mint_to(minter, psp34::Id::U64(self.last_token_id))?;

            // get nft collection of recipient if already holding
            let mut collection = match self.access.collections.get(minter) {
                Some(collection) => collection,
                None => Vec::new(),
            };

            // add id to recipient's nft collection
            collection.push(psp34::Id::U64(self.last_token_id));
            self.access.collections.insert(minter, &collection);

            Ok(())
        }

        /// - This is a mint function for Art Zero interface.
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint_with_attributes(
            &mut self,
            metadata: Vec<(String, String)>,
        ) -> Result<(), Error> {

            let caller = self.env().caller();

            // next token id
            self.last_token_id += 1;

            // make sure cap is not surpassed
            if self.last_token_id >= self.access.cap {
                return Err(Error::Custom(
                       format!("The NFT cap of {:?} has been met. Cannot mint.", self.access.cap)))
            }

            // mint and set
            let _ = self._mint_to(caller, Id::U64(self.last_token_id))?;
            let _ = self.set_multiple_attributes(Id::U64(self.last_token_id), metadata)?;

            // update recipient's collection
            let mut collection = match self.access.collections.get(caller) {
                Some(collection) => collection,
                None => Vec::new(),
            };
            collection.push(Id::U64(self.last_token_id));
            self.access.collections.insert(caller, &collection);

            Ok(())
        }

        /// - This registers this universal access nft contract with
        /// ILOCK PSP22 token contract to allow self-minting.
        /// - Only contract owner may create a socket between this contract and the ILOCK PSP22 token.
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn create_socket(
            &mut self
        ) -> Result<(), OtherError> {

            // make sure caller is operator
            if self.env().caller() != self.app.operator.address {

                return Err(OtherError::CallerNotOperator);
            }

            self.app.token_instance.create_socket(self.env().caller(), PORT)
        }

        /// - This makes call through universal access nft socket to ILOCK PSP22 token contract on
        /// port 0 or port 1, depending on this contract's configuration and affiliation with
        /// Interlock Network.
        /// - (Ie, transfer token from recipient to contract owner within PSP22 contract.)
        /// - Only operator may call.
        #[ink(message)]
        pub fn call_socket(
            &mut self,
            address: AccountId,
            amount: Balance,
            data: Vec<u8>,                  // <--! data vector to pass custom information to token
            ) -> Result<(), OtherError> {   //      contract logic

            self.app.token_instance.call_socket(address, amount, data)
        }

        /// - Store hashed username password pair to register credentials of UANFT owner.
        /// - Anybody may call, but only UANFT owner of Id may successfully register.
        #[ink(message)]
        pub fn register(
            &mut self,
            id: Id,
            userhash: Hash,
            passhash: Hash,
        ) -> Result<(), PSP34Error> {
    
            // get nft owner
            let owner: AccountId = match self.owner_of(id.clone()) {
                Some(owner) => owner,
                None => return Err(PSP34Error::Custom(
                               format!("NFT id {:?} does not exist.", id).into_bytes())),
            };

            // make sure signing caller owns UANFT
            if self.env().caller() != owner {

                return Err(PSP34Error::Custom(
                       format!("Caller does not own UANFT id {:?}.", id).into_bytes()))
            }

            // make sure username is not already taken
            match self.access.credentials.get(userhash) {

                // if id matches id in credential mapping, then duplicate username belongs to
                // caller, and caller is effectively resetting password
                Some(credential) => {

                    // no id match thus username registered with different uanft
                    if credential.1 != id {
                        return Err(PSP34Error::Custom(
                               format!("Username already taken by UANFT ID {:?}.",
                                       credential.1).into_bytes()))
                    }
                },

                // None means username is not registered thus is available
                None => (),
            };

            // make sure uanft owner hasn't already registered different credentials
            match self.access.userhashes.get(id.clone()) {

                // if entry exists, owner has already registered under different userhash
                Some(userhash) => {

                    // eliminate entry to deduplicate credentials
                    self.access.credentials.remove(userhash);
                },

                // None means this is first time registering credentials with uanft
                None => (),
            };

            // password and uanft id info affiliated with username
            self.access.credentials.insert(userhash, &(passhash, id.clone()));

            // username affiliated with uanft id
            // ...this is necessary to revoke access upon uanft transfer
            self.access.userhashes.insert(id, &userhash);

            Ok(())
        }

        /// - Store hashed username password pair.
        /// - Also associate uanft id with username.
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn set_credential(
            &mut self,
            id: Id,
            userhash: Hash,
            passhash: Hash,
        ) -> Result<(), PSP34Error> {

            // password and uanft id info affiliated with username
            self.access.credentials.insert(userhash, &(passhash, id.clone()));

            // username affiliated with uanft id
            // ...this is necessary to revoke access upon uanft transfer
            self.access.userhashes.insert(id, &userhash);

            Ok(())
        }

        /// - Revoke access for particular user.
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn revoke_access(
            &mut self,
            userhash: Hash,
        ) -> Result<(), PSP34Error> {

            // get the uanft id associated with username
            let uanft: Id = match self.access.credentials.get(userhash) {
                Some(credential) => credential.1,
                None => return Err(PSP34Error::Custom(
                               format!("Username not registered.").into_bytes())),
            };

            // remove hash pair
            self.access.credentials.remove(userhash);

            // remove uanft id mapping to userhash
            self.access.userhashes.remove(uanft);

            Ok(())
        }

        /// - Retrieve the current price of universal access nft self-minting.
        #[ink(message)]
        pub fn get_token_price(
            &self,
        ) -> Balance {

            self.access.nft_psp22price
        }

        /// - Owner may change the price that self-minter must pay for universal access nft.
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn set_token_price(
            &mut self,
            price: Balance,
        ) -> Result<(), PSP34Error> {

            self.access.nft_psp22price = price;

            Ok(())
        }

        /// - Get collection of nfts held by particular address.
        #[ink(message)]
        pub fn get_collection(
            &self,
            address: AccountId,
        ) -> Result<Vec<Id>, PSP34Error> {

            // retrieve the collection
            match self.access.collections.get(address) {
                Some(vec) => Ok(vec),
                None => Err(PSP34Error::Custom(
                        format!("The address {:?} does not have a collection.", address).into_bytes())),
            }
        }

        /// - Get hashed username password pair (plus UANFT Id).
        #[ink(message)]
        pub fn get_credential(
            &mut self,
            username: Hash,
        ) -> Result<(Hash, Id), PSP34Error> {

            // retrieve the collection
            match self.access.credentials.get(username) {
                Some((password, id)) => Ok((password, id)),
                None => Err(PSP34Error::Custom(
                        format!("Credentials nonexistent.").into_bytes())),
            }
        }

        /// - Check to see if UANFT is authenticated (has credentials registered).
        #[ink(message)]
        pub fn is_authenticated(
            &mut self,
            id: Id,
        ) -> Result<bool, PSP34Error> {

            // if userhash exists, then uanft is authenticated
            match self.access.userhashes.get(id) {
                Some(_hash) => Ok(true),
                None => Ok(false),
            }
        }

        /// - Modifies the code which is used to execute calls to this contract address.
        /// - This upgrades the token contract logic while using old state.
        #[ink(message)]
        #[openbrush::modifiers(only_owner)]
        pub fn update_contract(
            &mut self,
            code_hash: [u8; 32]
        ) -> Result<(), PSP34Error> {

            // takes code hash of updates contract and modifies preexisting logic to match
            ink::env::set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!(
                    "Failed to `set_code_hash` to {:?} due to {:?}",
                    code_hash, err
                )
            });

            Ok(())
        }

        /// - This is a testing helper for port applications.
        /// - Returns code hash for contract running e2e test (for port application verification).
        /// - Otherwise, each attempt to hardcode hash in application contract
        /// changes the hash for that run.
        #[ink(message)]
        pub fn contract_hash(
            &self,
            application: AccountId,
        ) -> Hash {

            self.env().code_hash(&application).unwrap()
        }

        /// - Art Zero helper function.
        pub fn add_attribute_name(
            &mut self,
            attribute_input: &Vec<u8>
        ) {
            let attr_input: String = String::from_utf8((*attribute_input).clone()).unwrap();
            let exist: bool = self.is_attribute.get(&attr_input).is_some();

            if !exist {
                self.attribute_count = self.attribute_count.checked_add(1).unwrap();
                self.attribute_names.insert(self.attribute_count, attribute_input);
                self.is_attribute.insert(&attr_input, &true);
            }
        }
    }

    impl Psp34Traits for Psp34Nft {

        /// - Art Zero message.
        ///
        /// - Get total token count.
        #[ink(message)]
        fn get_last_token_id(
            &self
        ) -> u64 {

            return self.last_token_id;
        }

        /// - Art Zero message.
        ///
        /// - Lock UANFT, only token owner can call.
        #[ink(message)]
        fn lock(
            &mut self,
            token_id: Id
        ) -> Result<(), Error> {
            
            let caller = self.env().caller();

            let token_owner = match self.owner_of(token_id.clone()) {
                Some(owner) => owner,
                None => return Err(Error::Custom(
                        format!("Token does not exist."))),
            };

            if caller != token_owner {
                return Err(Error::Custom(
                        format!("Caller not token owner.")));
            }

            match self.locked_token_count.checked_add(1) {
                Some(sum) => self.locked_token_count = sum,
                None => return Err(Error::Custom(
                        format!("Overflow"))),
            };

            self.locked_tokens.insert(&token_id, &true);

            Ok(())
        }

        /// - Art Zero message.
        ///
        /// - Check if token is locked or not.
        #[ink(message)]
        fn is_locked_nft(
            &self,
            token_id: Id
        ) -> bool {

            match self.locked_tokens.get(&token_id) {
                Some(_) => return true,
                None => return false,
            }
        }

        /// - Art Zero message.
        ///
        /// - Get locked token count.
        #[ink(message)]
        fn get_locked_token_count(
            &self
        ) -> u64 {
            return self.locked_token_count;
        }

        /// - Art Zero message.
        ///
        /// - Change UANFT base URI.
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_base_uri(
            &mut self,
            uri: String
        ) -> Result<(), Error> {

            self._set_attribute(
                Id::U8(0),
                String::from("baseURI").into_bytes(),
                uri.into_bytes(),
            );
            Ok(())
        }

        /// - Art Zero message.
        ///
        /// - Only contract owner can set multiple attributes to a UANFT.
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_multiple_attributes(
            &mut self,
            token_id: Id,
            metadata: Vec<(String, String)>,
        ) -> Result<(), Error> {

            if token_id == Id::U64(0){
                return Err(Error::InvalidInput)
            }            
            if self.is_locked_nft(token_id.clone()) {
                return Err(Error::Custom(
                        String::from("Token is locked")));
            }
            for (attribute, value) in &metadata {
                self.add_attribute_name(&attribute.clone().into_bytes());
                self._set_attribute(token_id.clone(), attribute.clone().into_bytes(), value.clone().into_bytes());
            }

            Ok(())
        }

        /// - Art Zero message.
        ///
        /// - Get multiple attributes.
        #[ink(message)]
        fn get_attributes(
            &self,
            token_id: Id,
            attributes: Vec<String>
        ) -> Vec<String> {

            let length = attributes.len();
            let mut ret = Vec::<String>::new();
            for i in 0..length {
                let attribute = attributes[i].clone();
                let value = self.get_attribute(token_id.clone(), attribute.into_bytes());
                if value.is_some() {
                    ret.push(String::from_utf8(value.unwrap()).unwrap());
                } else {
                    ret.push(String::from(""));
                }
            }
            ret
        }

        /// - Art Zero message.
        ///
        /// - Get attribute count.
        #[ink(message)]
        fn get_attribute_count(
            &self
        ) -> u32 {
            self.attribute_count
        }

        /// - Art Zero message.
        ///
        /// - Get attribute name.
        #[ink(message)]
        fn get_attribute_name(
            &self,
            index: u32
        ) -> String {
            
            match self.attribute_names.get(&index) {
                Some(attribute) => String::from_utf8(attribute).unwrap(),
                None => String::from(""),
            }
        }

        /// - Art Zero message.
        ///
        /// - Get URI from UANFT Id.
        #[ink(message)]
        fn token_uri(
            &self,
            token_id: u64
        ) -> String {
            let value = self.get_attribute(Id::U8(0), String::from("baseURI").into_bytes());
            let mut token_uri = String::from_utf8(value.unwrap()).unwrap();
            token_uri = token_uri + &token_id.to_string() + &String::from(".json");
            token_uri
        }

        /// - Art Zero message.
        ///
        /// - Get contract owner.
        #[ink(message)]
        fn get_owner(
            &self,
        ) -> AccountId {
            
            self.owner()
        }
    }


//
// TESTING 
//
// . To view debug prints and assertion failures run test via:
//
//      cargo +nightly test --features e2e-tests -- --show-output
//
// . To view debug for specific method run test via:
//
//      cargo +nightly test <test_function_here> -- --nocapture
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
//
//
// TEST TODO
// in order of appearance
//
// [x] happye2e_transfer      
// [] sade2e_transfer       
// [x] happyunit_new (no sad, returns only Self)
// [x] happye2e_mint
//      [x] happye2e_get_collection
// [] sade2e_mint
// [x] happye2e_self_mint            <-- includes call_socket()
// [] sade2e_self_mint
// [x] ** happye2e_create_socket     \
// [] ** sade2e_create_socket       |----- these must be performed from generic port
// [x] ** happye2e_call_socket       |      or from the uanft contract's self minting message
// [] ** sade2e_call_socket         /
// [x] happyunit_register
//      [x] happyunit_set_credential
//      [x] happyunit_get_gredental
//      [x] happyunit_is_authenticated
//      [x] happyunit_revoke_access
// [x] happyunit_set_token_price
//      [x] happyunit_get_token_price
//

////////////////////////////////////////////////////////////////////////////
//// end to end ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {

        use super::*;
        use crate::psp34_nft::PSP34Error::Custom;
        use ink_e2e::{
            build_message,
        };
        use openbrush::contracts::psp34::psp34_external::PSP34;

        // byte array representing SHA256('test_username')
        const TEST_USERNAME_ARRAY: [u8; 32] = [ 204, 221, 179,  10, 141,  56,  15, 156,
                                                  2, 209, 187,  54, 104,  62,  98, 214,
                                                103, 214,  46,  36,  77,  66, 122, 252,
                                                 68,  10, 183, 131, 110, 216,  20, 240 ];

        // byte array representing SHA256('test_password')
        const TEST_PASSWORD_ARRAY: [u8; 32] = [  16, 166, 230, 204, 131,  17, 163, 226,
                                                188, 192, 155, 246, 193, 153, 173, 236,
                                                213, 221,  89,  64, 140,  52,  62, 146,
                                                107,  18, 156,  73,  20, 243, 203,   1 ];

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// HAPPY TRANSFER
        /// - Test if customized transfer function works correctly.
        /// - When transfer, credentials are revoked.
        /// - Test that register function works correctly.
        /// - Test that transfer events are properly emitted.
        /// - Test that get_credential() and get_collection() works..
        #[ink_e2e::test(additional_contracts = "../contract-ilockmvp/Cargo.toml")]
        async fn happye2e_mint_register_transfer(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            let test_username_hash: Hash = Hash::from(TEST_USERNAME_ARRAY);
            let test_password_hash: Hash = Hash::from(TEST_PASSWORD_ARRAY);

            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            let charlie_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie);

            let ilock_constructor = ilockmvp::ILOCKmvpRef::new_token();
            let ilock_contract_acct_id = client
                .instantiate("ilockmvp", &ink_e2e::alice(), ilock_constructor, 0, None)
                .await.expect("instantiate failed").account_id;

            let constructor = Psp34NftRef::new(
                "Interlock Network Universal Access NFT".to_string(),
                "ILOCK-UANFT".to_string(),
                "GENERAL-ACCESS".to_string(),
                10_000,
                100,
                ilock_contract_acct_id,
            );
            let uanft_contract_acct_id = client
                .instantiate("interlock_access_nft", &ink_e2e::alice(), constructor, 0, None)
                .await.expect("instantiate failed").account_id;
        
            let mint_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.mint_to(bob_account.clone()));
            let mint_response = client
                .call(&ink_e2e::alice(), mint_msg, 0, None).await.unwrap();
            
            // filter for transfer mint event
            let contract_emitted_transfer = mint_response
                .events
                .iter()
                .find(|event| {
                    event
                        .as_ref()
                        .expect("expected event")
                        .event_metadata()
                        .event()
                        == "ContractEmitted" &&
                        String::from_utf8_lossy(
                            event.as_ref().expect("bad event").bytes()).to_string()
                       .contains("Psp34Nft::Transfer")
                })
                .expect("Expect ContractEmitted event")
                .unwrap();

            // decode to the expected event type (skip field_context)
            let transfer_event = contract_emitted_transfer.field_bytes();
            let decoded_transfer =
                <Transfer as scale::Decode>::decode(&mut &transfer_event[34..]).expect("invalid data");

            // destructor decoded eapproval
            let Transfer { from, to, id } = decoded_transfer;

            // assert with the expected value
            assert_eq!(from, None, "encountered invalid Transfer.to");
            assert_eq!(to, Some(bob_account), "encountered invalid Transfer.from");
            assert_eq!(id, Id::U64(1), "encountered invalid Transfer.id");  
            
            let owner_of_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.owner_of(Id::U64(1)));
            let owner = client
                .call_dry_run(&ink_e2e::alice(), &owner_of_msg, 0, None).await.return_value().unwrap();
            assert_eq!(owner, bob_account.clone());

            let get_bob_collection_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.get_collection(bob_account.clone()));
            let bob_collection = client
                .call_dry_run(&ink_e2e::alice(), &get_bob_collection_msg, 0, None).await.return_value().unwrap();
            assert_eq!(bob_collection, [Id::U64(1)]);

            let mint_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.mint_to(bob_account.clone()));
            let _mint_result = client
                .call(&ink_e2e::alice(), mint_msg, 0, None).await;

            let bob_collection = client
                .call_dry_run(&ink_e2e::alice(), &get_bob_collection_msg, 0, None).await.return_value().unwrap();
            assert_eq!(bob_collection, [Id::U64(1), Id::U64(2)]);
        
            let register_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.register(Id::U64(2), test_username_hash, test_password_hash));
            let _register_result = client
                .call(&ink_e2e::bob(), register_msg, 0, None).await;

            let bob_get_credential_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.get_credential(test_username_hash));
            let bob_credential = client
                .call_dry_run(&ink_e2e::bob(), &bob_get_credential_msg, 0, None).await.return_value().unwrap();
            assert_eq!(bob_credential.0, test_password_hash);
            assert_eq!(bob_credential.1, Id::U64(2));
                    
            let transfer_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.transfer(
                    charlie_account.clone(), Id::U64(2), Default::default()));
            let transfer_result = client
                .call(&ink_e2e::bob(), transfer_msg, 0, None).await.unwrap();
            
            // filter for transfer event
            let contract_emitted_transfer = transfer_result
                .events
                .iter()
                .find(|event| {
                    event
                        .as_ref()
                        .expect("expected event")
                        .event_metadata()
                        .event()
                        == "ContractEmitted" &&
                        String::from_utf8_lossy(
                            event.as_ref().expect("bad event").bytes()).to_string()
                       .contains("Psp34Nft::Transfer")
                })
                .expect("Expect ContractEmitted event")
                .unwrap();

            // decode to the expected event type (skip field_context)
            let transfer_event = contract_emitted_transfer.field_bytes();
            let decoded_transfer =
                <Transfer as scale::Decode>::decode(&mut &transfer_event[35..]).expect("invalid data");

            // destructor decoded eapproval
            let Transfer { from, to, id } = decoded_transfer;

            // assert with the expected value
            assert_eq!(from, Some(bob_account), "encountered invalid Transfer.to");
            assert_eq!(to, Some(charlie_account), "encountered invalid Transfer.from");
            assert_eq!(id, Id::U64(2), "encountered invalid Transfer.id");  

            let bob_collection = client
                .call_dry_run(&ink_e2e::alice(), &get_bob_collection_msg, 0, None).await.return_value().unwrap();
            assert_eq!(bob_collection, [Id::U64(1)]);

            let get_charlie_collection_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.get_collection(charlie_account.clone()));
            let charlie_collection = client
                .call_dry_run(&ink_e2e::alice(), &get_charlie_collection_msg, 0, None)
                .await.return_value().unwrap();
            assert_eq!(charlie_collection, [Id::U64(2)]);

            let owner_of_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.owner_of(Id::U64(2)));
            let owner = client
                .call_dry_run(&ink_e2e::alice(), &owner_of_msg, 0, None).await.return_value().unwrap();
            assert_eq!(owner, charlie_account.clone());

            let bob_get_credential_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.get_credential(test_username_hash));
            let bob_credential = client
                .call_dry_run(&ink_e2e::bob(), &bob_get_credential_msg, 0, None).await.return_value();
            assert_eq!(bob_credential,
                // Error: collection does not exist
                Err(Custom([67, 114, 101, 100, 101, 110, 116,
                           105, 97, 108, 115, 32, 110, 111, 110,
                           101, 120, 105, 115, 116, 101, 110, 116, 46].to_vec())));

            let set_credential_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.set_credential(Id::U64(1), test_username_hash, test_password_hash));
            let _set_credential_result = client
                .call(&ink_e2e::alice(), set_credential_msg, 0, None).await;

            let is_authenticated_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.is_authenticated(Id::U64(1)));
            let status = client
                .call_dry_run(&ink_e2e::alice(), &is_authenticated_msg, 0, None).await.return_value().unwrap();
            assert_eq!(status, true);

            let revoke_access_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.revoke_access(test_username_hash));
            let _revoke_access_result = client
                .call(&ink_e2e::alice(), revoke_access_msg, 0, None).await;

            let is_authenticated_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.is_authenticated(Id::U64(1)));
            let status = client
                .call_dry_run(&ink_e2e::alice(), &is_authenticated_msg, 0, None).await.return_value().unwrap();
            assert_eq!(status, false);


            Ok(())
        }

        /// HAPPY SELF-MINT
        /// - Test that anybody can mint UANFT for themselves using ILOCK.
        #[ink_e2e::test(additional_contracts = "../contract-ilockmvp/Cargo.toml")]
        async fn happye2e_self_mint(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

            let ilock_constructor = ilockmvp::ILOCKmvpRef::new_token();
            let ilock_contract_acct_id = client
                .instantiate("ilockmvp", &ink_e2e::alice(), ilock_constructor, 0, None)
                .await.expect("instantiate failed").account_id;

            let uanft_constructor = Psp34NftRef::new(
                "Interlock Network Universal Access NFT".to_string(),
                "ILOCK-UANFT".to_string(),
                "GENERAL-ACCESS".to_string(),
                10_000,
                0,
                ilock_contract_acct_id,
            );
            let uanft_contract_acct_id = client
                .instantiate("interlock_access_nft", &ink_e2e::alice(), uanft_constructor, 0, None)
                .await.expect("instantiate failed").account_id;

            let set_price_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.set_token_price(100));
            let _create_port_result = client
                .call(&ink_e2e::alice(), set_price_msg, 0, None).await;

            // we are assuming this testing contract is safe
            let get_hash_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.contract_hash(uanft_contract_acct_id.clone()));
            let application_hash = client
                .call_dry_run(&ink_e2e::alice(), &get_hash_msg, 0, None).await.return_value();

            let create_port_msg = build_message::<ilockmvp::ILOCKmvpRef>(ilock_contract_acct_id.clone())
                .call(|contract| contract.create_port(application_hash, 0, 0, false, 0, alice_account.clone() ));
            let _create_port_result = client
                .call(&ink_e2e::alice(), create_port_msg, 0, None).await;

            let reward_bob_msg = build_message::<ilockmvp::ILOCKmvpRef>(ilock_contract_acct_id.clone())
                .call(|contract| contract.reward_interlocker(100_000, bob_account.clone()));
            let _reward_result = client
                .call(&ink_e2e::alice(), reward_bob_msg, 0, None).await;
          
            let create_socket_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.create_socket());
            let _create_socket_result = client
                .call(&ink_e2e::alice(), create_socket_msg, 0, None).await;
           
            let get_token_price_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.get_token_price());
            let token_price = client
                .call_dry_run(&ink_e2e::alice(), &get_token_price_msg, 0, None).await.return_value();
            assert_eq!(token_price, 100);

            let self_mint_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.self_mint(token_price));
            let _mint_result = client
                .call(&ink_e2e::bob(), self_mint_msg, 0, None).await;

            let get_bob_collection_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.get_collection(bob_account.clone()));
            let bob_collection = client
                .call_dry_run(&ink_e2e::alice(), &get_bob_collection_msg, 0, None).await.return_value().unwrap();
            assert_eq!(bob_collection, [Id::U64(1)]);

            let bob_balance_of_msg = build_message::<ilockmvp::ILOCKmvpRef>(ilock_contract_acct_id.clone())
                .call(|contract| contract.balance_of(bob_account.clone()));
            let bob_balance = client
                .call_dry_run(&ink_e2e::alice(), &bob_balance_of_msg, 0, None).await.return_value();
            assert_eq!(bob_balance, 100_000 - 100);

            let supply_msg = build_message::<ilockmvp::ILOCKmvpRef>(ilock_contract_acct_id.clone())
                .call(|contract| contract.total_supply());
            let supply = client
                .call_dry_run(&ink_e2e::alice(), &supply_msg, 0, None).await.return_value();
            assert_eq!(supply, 100_000 - 100);

            Ok(())
        }
    }
}



