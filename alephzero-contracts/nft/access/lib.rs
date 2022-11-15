//
// INTERLOCK NETWORK - GENERAL ACCESS NFT
//

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
pub use self::psp34_nft::{Psp34Nft, Psp34NftRef};

#[openbrush::contract]
pub mod psp34_nft {
    use ink_prelude::string::String;
    use ink_prelude::string::ToString;
    use ink_prelude::vec::Vec;
    use ink_prelude::format;
    use ink_storage::traits::SpreadAllocate;
    use ink_storage::Mapping;
    use ink_lang::codegen::Env;
    use openbrush::contracts::ownable::*;
    use openbrush::contracts::psp34::extensions::enumerable::*;
    use openbrush::contracts::psp34::extensions::metadata::*;
    use openbrush::contracts::psp34::*;
    use openbrush::modifiers;

    #[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage, OwnableStorage)]
    #[ink(storage)]
    pub struct Psp34Nft {
        #[PSP34StorageField]
        psp34: PSP34Data<EnumerableBalances>,
        #[PSP34MetadataStorageField]
        metadata: PSP34MetadataData,
        #[OwnableStorageField]
        ownable: OwnableData,
        last_token_id: u64,
        attribute_count: u32,
        attribute_names: Mapping<u32, Vec<u8>>,
        locked_tokens: Mapping<Id, u8>,
        locked_token_count: u64,
        nfts_held: Mapping<AccountId, Vec<Id>>,
        cap: u64,

    }


    #[openbrush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Metadata;
    impl PSP34 for Psp34Nft {

        /// . override transfer function to reset each NFT to 'not authenticated' on transfer
        #[ink(message)]
        fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {

            let from = self.env().caller();
            let _ = self._transfer_token(to, id.clone(), data)?;
            self._set_attribute(
                id.clone(),
                String::from("isauthenticated").into_bytes(),
                String::from("false").into_bytes(),
            );

            // update sender's collection
            let mut from_collection = match self.nfts_held.get(from) {
                Some(collection) => collection,
                None => return Err(PSP34Error::Custom(format!("No collection, fatal error"))),
            };
            let index = match from_collection.iter().position(|element| *element == id) {
                Some(index) => index,
                None => return Err(PSP34Error::Custom(format!("No NFT in collection, fatal error"))),
            };
            from_collection.remove(index);
            self.nfts_held.insert(from, &from_collection);

            // update recipient's collection
            let mut to_collection = match self.nfts_held.get(to) {
                Some(collection) => collection,
                None => Vec::new(),
            };
            to_collection.push(id);
            self.nfts_held.insert(to, &to_collection);

            Ok(())
        }
    }

    impl Ownable for Psp34Nft {}
    impl PSP34Metadata for Psp34Nft {}
    impl PSP34Internal for Psp34Nft {}
    impl PSP34Enumerable for Psp34Nft {}

    #[openbrush::trait_definition]
    pub trait Psp34Traits {
        #[ink(message)]
        fn set_base_uri(&mut self, uri: String) -> Result<(), PSP34Error>;
        #[ink(message)]
        fn set_multiple_attributes(
            &mut self,
            token_id: Id,
            attributes: Vec<String>,
            values: Vec<String>,
        ) -> Result<(), PSP34Error>;
        #[ink(message)]
        fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String>;
        #[ink(message)]
        fn get_attribute_count(&self) -> u32;
        #[ink(message)]
        fn get_attribute_name(&self, index: u32) -> String;
        #[ink(message)]
        fn token_uri(&self, token_id: u64) -> String;
    }

    impl Psp34Nft {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, class: String, cap: u64) -> Self {
            
            // create the contract
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {

                // set attributes
                instance._set_attribute(
                    Id::U8(0),
                    String::from("name").into_bytes(),
                    name.into_bytes(),
                );
                instance._set_attribute(
                    Id::U8(0),
                    String::from("symbol").into_bytes(),
                    symbol.into_bytes(),
                );
                instance._set_attribute(
                    Id::U8(0),
                    String::from("class").into_bytes(),
                    class.into_bytes(),
                );

                // assign caller as owner
                instance._init_with_owner(instance.env().caller());

                // set cap
                instance.cap = cap;
            })
        }

        /// . mint an NFT VIP membership certificate
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
                return Err(PSP34Error::Custom(format!("The NFT cap of {:?} has been met. Cannot mint.", self.cap)))
            }

            // if cap not surpassed, mint next id
            let _ = self._mint_to(recipient, psp34::Id::U64(self.last_token_id))?;

            // get nft collection of recipient if already holding
            let mut collection = match self.nfts_held.get(recipient) {
                Some(collection) => collection,
                None => Vec::new(),
            };

            // add id to recipient's nft collection
            collection.push(psp34::Id::U64(self.last_token_id));
            self.nfts_held.insert(recipient, &collection);

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

        ///Only Owner can mint new token and add attributes for it
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
                return Err(PSP34Error::Custom(format!("The NFT cap of {:?} has been met. Cannot mint.", self.cap)))
            }

            // mint and set
            let _ = self._mint_to(recipient, Id::U64(self.last_token_id))?;
            let _ = self.set_multiple_attributes(Id::U64(self.last_token_id), attributes, values)?;

            // update recipient's collection
            let mut collection = match self.nfts_held.get(recipient) {
                Some(collection) => collection,
                None => Vec::new(),
            };
            collection.push(Id::U64(self.last_token_id));
            self.nfts_held.insert(recipient, &collection);

            Ok(())
        }

        ///Get Token Count
        #[ink(message)]
        pub fn get_last_token_id(&self) -> u64 {
            return self.last_token_id;
        }

        fn add_attribute_name(&mut self, attribute_input: Vec<u8>) {
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

        /// . Lock nft - Only owner token
        #[ink(message)]
        pub fn lock(&mut self, token_id: Id) -> Result<(), PSP34Error> {
            
            let caller = self.env().caller();

            let token_owner = match self.owner_of(token_id.clone()) {
                Some(owner) => owner,
                None => return Err(PSP34Error::Custom(format!("Token does not exist."))),
            };

            if caller != token_owner {
                return Err(PSP34Error::Custom(format!("Caller not token owner.")));
            }

            self.locked_token_count += 1;
            self.locked_tokens.insert(&token_id, &1);

            Ok(())
        }

        /// . Check token is locked or not
        #[ink(message)]
        pub fn is_locked_nft(&self, token_id: Id) -> bool {

            match self.locked_tokens.get(&token_id) {
                Some(_) => return true,
                None => return false,
            }
        }

        /// . Get Locked Token Count
        #[ink(message)]
        pub fn get_locked_token_count(&self) -> u64 {
            return self.locked_token_count;
        }

        /// . remove token from circulation
        #[ink(message)]
        pub fn burn(&mut self, id: Id) -> Result<(), PSP34Error> {
            
            let caller = self.env().caller();

            let token_owner = match self.owner_of(id.clone()) {
                Some(owner) => owner,
                None => return Err(PSP34Error::Custom(format!("Token does not exist."))),
            };

            if caller != token_owner {
                return Err(PSP34Error::Custom(format!("Caller not token owner.")));
            }

            self._burn_from(caller, id)
        }
    }

    impl Psp34Traits for Psp34Nft {

        /// . Change baseURI
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_base_uri(&mut self, uri: String) -> Result<(), PSP34Error> {

            self._set_attribute(
                Id::U8(0),
                String::from("baseURI").into_bytes(),
                uri.into_bytes(),
            );
            Ok(())
        }

        ///Only Owner can set multiple attributes to a token
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
                return Err(PSP34Error::Custom(String::from("Token is locked")));
            }
            if attributes.len() != values.len() {
                return Err(PSP34Error::Custom(String::from("Inputs not same length")));
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
                        return Err(PSP34Error::Custom(String::from("Duplicated Attributes")));
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

        /// Get multiple  attributes
        #[ink(message)]
        fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String> {
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

        ///Get Attribute Count
        #[ink(message)]
        fn get_attribute_count(&self) -> u32 {
            self.attribute_count
        }

        ///Get Attribute Name
        #[ink(message)]
        fn get_attribute_name(&self, index: u32) -> String {
            
            match self.attribute_names.get(&index) {
                Some(attribute) => String::from_utf8(attribute).unwrap(),
                None => String::from(""),
            }
        }

        /// Get URI from token ID
        #[ink(message)]
        fn token_uri(&self, token_id: u64) -> String {
            let value = self.get_attribute(Id::U8(0), String::from("baseURI").into_bytes());
            let mut token_uri = String::from_utf8(value.unwrap()).unwrap();
            token_uri = token_uri + &token_id.to_string() + &String::from(".json");
            return token_uri;
        }
    }
}
