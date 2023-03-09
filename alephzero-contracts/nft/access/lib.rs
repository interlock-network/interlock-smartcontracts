//!
//! INTERLOCK NETWORK - UNIVERSAL ACCESS NFT
//!
//! This is a PSP34 NFT in compatible with Art Zero marketplace and capable of managing user access
//! credentials on the blockchain using a strategy similar to two-factor-authentication (2FA).
//!
//! Build with cargo-contract version 2.0.0
//!
//!      cargo install cargo-contract --force --version 2.0.0
//!
//! Build
//!
//!      cargo +nightly contract build
//!
//!  To build docs:
//!
//!      cargo +nightly doc --no-deps --document-private-items --open
//!
//! To reroute docs in Github
//!
//!      echo "<meta http-equiv=\"refresh\" content=\"0; url=build_wheel\">" >
//!      target/doc/index.html;
//!      cp -r target/doc ./docs
//!

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

pub use self::psp34_nft::{Psp34Nft, Psp34NftRef};

#[openbrush::contract]
pub mod psp34_nft {

    // ink 4 imports
    use ink::{
        codegen::Env,
        storage::Mapping,
        prelude::{
            string::{String, ToString},
            vec::Vec,
            format,
        },
    };

    // openbrush 3 imports
    use openbrush::{
        traits::Storage,
        modifiers,
        contracts::{
            ownable::*,
            psp34::extensions::{enumerable::*, metadata::*},
            psp22::psp22_external::PSP22,
        },
    };

    // we use these to interface as uanft application
    // with the Interlock Network PSP22 contract
    use ilockmvp::{
        ilockmvp::OtherError,
        ILOCKmvpRef,
    };

    /// this is a type wrapper to implement Default method
    /// on AccountId type. Ink 4 stable eliminated AccountId Default
    /// (which was zero address, that has known private key)
    /// ...we only really need this because Openbrush contract
    ///    relies on deriving Default for contract storage, and
    ///    our AccesData struct contains AccountId
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
        locked_tokens: Mapping<Id, u8>,
        locked_token_count: u64,
    }

    #[openbrush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Metadata;

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

    #[openbrush::trait_definition]
    pub trait Psp34Traits {
        #[ink(message)]
        fn set_base_uri(
            &mut self,
            uri: String
        ) -> Result<(), PSP34Error>;
        #[ink(message)]
        fn set_multiple_attributes(
            &mut self,
            token_id: Id,
            attributes: Vec<String>,
            values: Vec<String>,
        ) -> Result<(), PSP34Error>;
        #[ink(message)]
        fn get_attributes(
            &self,
            token_id: Id,
            attributes: Vec<String>
        ) -> Vec<String>;
        #[ink(message)]
        fn get_attribute_count(
            &self
        ) -> u32;
        #[ink(message)]
        fn get_attribute_name(
            &self,
            index: u32
        ) -> String;
        #[ink(message)]
        fn token_uri(
            &self,
            token_id: u64
        ) -> String;
    }

    impl Psp34Nft {

        /// - UANFY contract constructor.
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

            // create a reference to the deployed token contract
            contract.app.token_instance = ink::env::call::FromAccountId::from_account_id(token_address);
            contract.app.operator.address = Self::env().caller();

            // set cap
            contract.access.cap = cap;

            // set nft price in PSP22 token
            contract.access.nft_psp22price = price;

            contract
        }

        /// - This mints a universal access nft.
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(
            &mut self,
            recipient: AccountId,
        ) -> Result<(), PSP34Error> {

            // next token id
            self.last_token_id += 1;

            // make sure cap is not surpassed
            if self.last_token_id >= self.access.cap {
                return Err(PSP34Error::Custom(
                       format!("The NFT cap of {:?} has been met. Cannot mint.", self.access.cap).into_bytes()))
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
        ) -> Result<(), PSP34Error> {

            // next token id
            self.last_token_id += 1;

            // mint recipient
            let minter: AccountId = self.env().caller();

            // make sure cap is not surpassed
            if self.last_token_id >= self.access.cap {
                return Err(PSP34Error::Custom(
                       format!("The NFT cap of {:?} has been met. Cannot mint.", self.access.cap).into_bytes()))
            }

            // make sure asking price matches nft_psp22price
            // ...this is to ensure that contract owner doesn't hike up token price between the
            //    time somebody checks the price, and the time that somebody submits tx to
            //    self-mint for that given price
            if self.access.nft_psp22price > price {
                return Err(PSP34Error::Custom(
                       format!("Current NFT price greater than agreed sale price of {:?}.", price).into_bytes()))
            }
            
            // make sure mint recipient can afford the PSP22 token price
            let recipient_balance: Balance = self.app.token_instance.balance_of(minter);
            if recipient_balance < price {
                return Err(PSP34Error::Custom(
                       format!("Minter cannot affort NFT at current price of {:?}.", price).into_bytes()))
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

        /// - Only contract owner can mint new token and add custom attributes for it.
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint_with_attributes(
            &mut self,
            recipient: AccountId,
            attributes: Vec<String>,
            values: Vec<String>,
        ) -> Result<(), PSP34Error> {

            // next token id
            self.last_token_id += 1;

            // make sure cap is not surpassed
            if self.last_token_id >= self.access.cap {
                return Err(PSP34Error::Custom(
                       format!("The NFT cap of {:?} has been met. Cannot mint.", self.access.cap).into_bytes()))
            }

            // mint and set
            let _ = self._mint_to(recipient, Id::U64(self.last_token_id))?;
            let _ = self.set_multiple_attributes(Id::U64(self.last_token_id), attributes, values)?;

            // update recipient's collection
            let mut collection = match self.access.collections.get(recipient) {
                Some(collection) => collection,
                None => Vec::new(),
            };
            collection.push(Id::U64(self.last_token_id));
            self.access.collections.insert(recipient, &collection);

            Ok(())
        }

        /// - This registers this universal access nft contract with ILOCK PSP22 token contract to allow self-minting.
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

        /// - Art Zero message.
        /// - Get total token count.
        #[ink(message)]
        pub fn get_last_token_id(
            &self
        ) -> u64 {

            return self.last_token_id;
        }

        /// - Art Zero function.
        fn add_attribute_name(
            &mut self,
            attribute_input: Vec<u8>
        ) {

            let mut exist: bool = false;
            for index in 0..self.attribute_count {
                let attribute_name = self.attribute_names.get(&(index + 1));
                if attribute_name.is_some() {
                    if attribute_name.unwrap() == attribute_input {
                        exist = true;
                        break;
                    }
                }
            }
            if !exist {
                self.attribute_count += 1;
                self.attribute_names
                    .insert(&self.attribute_count, &attribute_input);
            }
        }

        /// - Art Zero message.
        /// - Lock UANFT, only token owner can call.
        #[ink(message)]
        pub fn lock(
            &mut self,
            token_id: Id
        ) -> Result<(), PSP34Error> {
            
            let caller = self.env().caller();

            let token_owner = match self.owner_of(token_id.clone()) {
                Some(owner) => owner,
                None => return Err(PSP34Error::Custom(
                        format!("Token does not exist.").into_bytes())),
            };

            if caller != token_owner {
                return Err(PSP34Error::Custom(
                        format!("Caller not token owner.").into_bytes()));
            }

            self.locked_token_count += 1;
            self.locked_tokens.insert(&token_id, &1);

            Ok(())
        }

        /// - Art Zero message.
        /// - Check if token is locked or not.
        #[ink(message)]
        pub fn is_locked_nft(
            &self,
            token_id: Id
        ) -> bool {

            match self.locked_tokens.get(&token_id) {
                Some(_) => return true,
                None => return false,
            }
        }

        /// - Art Zero message.
        /// - Get locked token count.
        #[ink(message)]
        pub fn get_locked_token_count(
            &self
        ) -> u64 {
            return self.locked_token_count;
        }

        /// - Art Zero message.
        /// - Remove token from circulation.
        #[ink(message)]
        pub fn burn(
            &mut self,
            id: Id
        ) -> Result<(), PSP34Error> {
            
            let caller = self.env().caller();

            let token_owner = match self.owner_of(id.clone()) {
                Some(owner) => owner,
                None => return Err(PSP34Error::Custom(
                        format!("Token does not exist.").into_bytes())),
            };

            if caller != token_owner {
                return Err(PSP34Error::Custom(
                        format!("Caller not token owner.").into_bytes()));
            }

            self._burn_from(caller, id)
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
    }

    impl Psp34Traits for Psp34Nft {

        /// - Art Zero message.
        /// - Change UANFT base URI.
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_base_uri(
            &mut self,
            uri: String
        ) -> Result<(), PSP34Error> {

            self._set_attribute(
                Id::U8(0),
                String::from("baseURI").into_bytes(),
                uri.into_bytes(),
            );
            Ok(())
        }

        /// - Art Zero message.
        /// - Only contract owner can set multiple attributes to a UANFT.
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_multiple_attributes(
            &mut self,
            token_id: Id,
            attributes: Vec<String>,
            values: Vec<String>,
        ) -> Result<(), PSP34Error> {

            assert!(token_id != Id::U64(0));
            if self.is_locked_nft(token_id.clone()) {
                return Err(PSP34Error::Custom(
                        String::from("Token is locked").into_bytes()));
            }
            if attributes.len() != values.len() {
                return Err(PSP34Error::Custom(
                        String::from("Inputs not same length").into_bytes()));
            }
            //Check Duplication
            let mut sorted_attributes = attributes.clone();
            sorted_attributes.sort();
            let length = sorted_attributes.len();
            for i in 0..length {
                let attribute = sorted_attributes[i].clone();
                let byte_attribute = attribute.into_bytes();
                if i + 1 < length {
                    let next_attribute = sorted_attributes[i + 1].clone();
                    let byte_next_attribute = next_attribute.into_bytes();
                    if byte_attribute == byte_next_attribute {
                        return Err(PSP34Error::Custom(
                                String::from("Duplicated attributes").into_bytes()));
                    }
                }
                let unsorted_attribute = attributes[i].clone();
                let byte_unsorted_attribute = unsorted_attribute.into_bytes();
                let value = values[i].clone();
                self.add_attribute_name(byte_unsorted_attribute.clone());
                self._set_attribute(
                    token_id.clone(),
                    byte_unsorted_attribute.clone(),
                    value.into_bytes(),
                );
            }
            Ok(())
        }

        /// - Art Zero message.
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
        /// - Get attribute count.
        #[ink(message)]
        fn get_attribute_count(
            &self
        ) -> u32 {
            self.attribute_count
        }

        /// - Art Zero message.
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
        /// - Get URI from UANFT Id.
        #[ink(message)]
        fn token_uri(
            &self,
            token_id: u64
        ) -> String {
            let value = self.get_attribute(Id::U8(0), String::from("baseURI").into_bytes());
            let mut token_uri = String::from_utf8(value.unwrap()).unwrap();
            token_uri = token_uri + &token_id.to_string() + &String::from(".json");
            return token_uri;
        }

    }
//
// TESTING INCOMPLETE
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
// [] happye2e_transfer      
// [] sade2e_transfer       
// [] happyunit_new (no sad, returns only Self)
// [] happye2e_mint
//      [] happye2e_get_collection
// [] sade2e_mint
// [] happye2e_self_mint            <-- includes call_socket()
// [] sade2e_self_mint
// [] ** happye2e_create_socket     \
// [] ** sade2e_create_socket       |----- these must be performed from generic port
// [] ** happye2e_call_socket       |      or from the uanft contract's self minting message
// [] ** sade2e_call_socket         /
// [] happyunit_register
//      [] happyunit_set_credential
//      [] happyunit_get_gredental
//      [] happyunit_is_authenticated
//      [] happyunit_revoke_access
// [] happyunit_set_token_price
//      [] happyunit_get_token_price
//

////////////////////////////////////////////////////////////////////////////
//// end to end ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {

    }


////////////////////////////////////////////////////////////////////////////
//// unit tests ////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////

    #[cfg(test)]
    mod tests {

    }
}

