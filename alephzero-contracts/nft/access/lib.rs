//
// INTERLOCK NETWORK - GENERAL ACCESS NFT
//

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
pub use self::psp34_nft::{Psp34Nft, Psp34NftRef};

#[openbrush::contract]
pub mod psp34_nft {

    use ink::codegen::Env;
    use ink::prelude::{
        string::{String, ToString},
        vec::Vec,
        format,
    };
    use ink::storage::Mapping;
    use openbrush::{
        contracts::{
            ownable::*,
            psp34::extensions::{enumerable::*, metadata::*},
        },
        traits::Storage,
        modifiers,
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Psp34Nft {

        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,

        last_token_id: u64,
        attribute_count: u32,
        attribute_names: Mapping<u32, Vec<u8>>,
        locked_tokens: Mapping<Id, u8>,
        locked_token_count: u64,
        collection: Mapping<AccountId, Vec<Id>>,
        cap: u64,
        credentials: Mapping<Hash, (Hash, Id)>,
        // username hash -> (password hash, nft ID)
    }

    #[openbrush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Metadata;

    impl PSP34 for Psp34Nft {

        /// . override transfer function to reset each NFT to 'not authenticated' on transfer
        /// . also updates 'collection' status (collection)
        #[ink(message)]
        fn transfer(
            &mut self,
            to: AccountId,
            id: Id,
            data: Vec<u8>
        ) -> Result<(), PSP34Error> {

            // revoke authenticated status
            let from = self.env().caller();
            let _ = self._transfer_token(to, id.clone(), data)?;
            self._set_attribute(
                id.clone(),
                String::from("isauthenticated").into_bytes(),
                String::from("false").into_bytes(),
            );

            // update sender's collection
            let mut from_collection = match self.collection.get(from) {
                Some(collection) => collection,
                None => return Err(PSP34Error::Custom(
                        format!("No collection, fatal error").into_bytes())),
            };
            let index = match from_collection.iter().position(|element| *element == id) {
                Some(index) => index,
                None => return Err(PSP34Error::Custom(
                        format!("No NFT in collection, fatal error").into_bytes())),
            };
            from_collection.remove(index);
            self.collection.insert(from, &from_collection);

            // update recipient's collection
            let mut to_collection = match self.collection.get(to) {
                Some(collection) => collection,
                None => Vec::new(),
            };
            to_collection.push(id);
            self.collection.insert(to, &to_collection);

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

        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            class: String,
            cap: u64,
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

            // set cap
            contract.cap = cap;

            contract
        }

        /// . mint an access NFT
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(
            &mut self,
            recipient: AccountId,
        ) -> Result<(), PSP34Error> {

            // next token id
            self.last_token_id += 1;

            // make sure cap is not surpassed
            if self.last_token_id >= self.cap {
                return Err(PSP34Error::Custom(
                        format!("The NFT cap of {:?} has been met. Cannot mint.", self.cap).into_bytes()))
            }

            // if cap not surpassed, mint next id
            let _ = self._mint_to(recipient, psp34::Id::U64(self.last_token_id))?;

            // get nft collection of recipient if already holding
            let mut collection = match self.collection.get(recipient) {
                Some(collection) => collection,
                None => Vec::new(),
            };

            // add id to recipient's nft collection
            collection.push(psp34::Id::U64(self.last_token_id));
            self.collection.insert(recipient, &collection);

            // set metadata specific to token
            
            // initial authentication status is false
            self._set_attribute(
                psp34::Id::U64(self.last_token_id),
                String::from("isauthenticated").into_bytes(),
                String::from("false").into_bytes(),
            );

            // identifying info if relevant
            self._set_attribute(
                psp34::Id::U64(self.last_token_id),
                String::from("identification").into_bytes(),
                String::from("").into_bytes(),
            );

            Ok(())
        }

        /// . only contract owner can mint new token and add attributes for it
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
            if self.last_token_id >= self.cap {
                return Err(PSP34Error::Custom(
                        format!("The NFT cap of {:?} has been met. Cannot mint.", self.cap).into_bytes()))
            }

            // mint and set
            let _ = self._mint_to(recipient, Id::U64(self.last_token_id))?;
            let _ = self.set_multiple_attributes(Id::U64(self.last_token_id), attributes, values)?;

            // update recipient's collection
            let mut collection = match self.collection.get(recipient) {
                Some(collection) => collection,
                None => Vec::new(),
            };
            collection.push(Id::U64(self.last_token_id));
            self.collection.insert(recipient, &collection);

            Ok(())
        }

        /// . grant 'authenticated' status to interlocker
        /// . indicate no longer waiting for authentication transfer
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn set_authenticated(
            &mut self,
            id: Id,
        ) -> Result<(), PSP34Error> {

            // << insert custom logic here >>

            self._set_attribute(
                id.clone(),
                String::from("isauthenticated").into_bytes(),
                String::from("true").into_bytes(),
            );
            self._set_attribute(
                id,
                String::from("iswaiting").into_bytes(),
                String::from("false").into_bytes(),
            );

            Ok(())
        }

        /// . indicate that NFT is waiting for authentication transfer
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn set_waiting(
            &mut self,
            id: Id,
        ) -> Result<(), PSP34Error> {

            // << insert custom logic here >>

            self._set_attribute(
                id,
                String::from("iswaiting").into_bytes(),
                String::from("true").into_bytes(),
            );

            Ok(())
        }

        /// . store hashed username password pair
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn set_credential(
            &mut self,
            id: Id,
            username: Hash,
            password: Hash,
        ) -> Result<(), PSP34Error> {

            // << insert custom logic here >>

            self.credentials.insert(username, &(password, id));

            Ok(())
        }

        /// . revoke 'authenticated' status from interlocker
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn set_not_authenticated(
            &mut self,
            id: Id,
        ) -> Result<(), PSP34Error> {

            // << insert custom logic here >>

            self._set_attribute(
                id,
                String::from("isauthenticated").into_bytes(),
                String::from("false").into_bytes(),
            );

            Ok(())
        }

        /// . get collection of nfts held by particular wallet
        #[ink(message)]
        pub fn get_collection(
            &self,
            wallet: AccountId,
        ) -> Result<Vec<Id>, PSP34Error> {

            // retrieve the collection
            match self.collection.get(wallet) {
                Some(vec) => Ok(vec),
                None => Err(PSP34Error::Custom(
                        format!("The wallet {:?} does not have a collection.", wallet).into_bytes())),
            }
        }

        /// . get hashed username password pair
        #[ink(message)]
        pub fn check_credential(
            &mut self,
            username: Hash,
        ) -> Result<(Hash, Id), PSP34Error> {

            // << insert custom logic here >>

            // retrieve the collection
            match self.credentials.get(username) {
                Some((password, id)) => Ok((password, id)),
                None => Err(PSP34Error::Custom(
                        format!("Credentials nonexistent.").into_bytes())),
            }
        }

        /// . get total token count
        #[ink(message)]
        pub fn get_last_token_id(
            &self
        ) -> u64 {

            return self.last_token_id;
        }

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

        /// . lock nft - only token owner can call
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

        /// . check if token is locked or not
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

        /// . get locked token count
        #[ink(message)]
        pub fn get_locked_token_count(
            &self
        ) -> u64 {
            return self.locked_token_count;
        }

        /// . remove token from circulation
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

        /// . modifies the code which is used to execute calls to this contract address
        /// . this upgrades the token contract logic while using old state
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

        /// . change base URI
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

        /// . only contract owner can set multiple attributes to a token
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

        /// . get multiple attributes
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

        /// . get attribute count
        #[ink(message)]
        fn get_attribute_count(
            &self
        ) -> u32 {
            self.attribute_count
        }

        /// . get attribute name
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

        /// . get URI from token ID
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
}
