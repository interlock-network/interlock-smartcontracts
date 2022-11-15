//
// INTERLOCK NETWORK - 
// PSP34 ACCESS CONTRACT - BOUNCER LICENSE
//
// !!!!! INCOMPLETE AND UNAUDITED, WARNING !!!!!
//
// This is a standard ERC721-style token contract
// with provisions for enforcing proof of Bouncer License ownership,

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod bouncerlicense {

    use ink_storage::{
        traits::SpreadAllocate,
        Mapping,
    };
    use ink_prelude::{
        string::String,
        vec::Vec,
        vec,
        format,
    };
    use openbrush::{
        contracts::{
            psp34::extensions::{
                metadata::*,
                mintable::*,
            },
            ownable::*,
        },
        traits::Storage,
    };

    pub const CAP: u16 = 1000;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct BouncerLicense {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        next_bouncerlicense_id: u16,
        nfts_held: Mapping<AccountId, Vec<u16>>,
    }


    impl PSP34 for BouncerLicense {

        /// . override transfer function to reset each NFT to 'not authenticated' on transfer
        #[ink(message)]
        fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {

            let _ = self._transfer_token(to, id.clone(), data)?;
            self._set_attribute(
                id,
                String::from("AUTHENTICATED").into_bytes(),
                vec![0],
            );

            Ok(())
        }

        // no transfer_from function for PSP34
    }

    impl PSP34Metadata for BouncerLicense {}
    impl Ownable for BouncerLicense {}
    impl PSP34Mintable for BouncerLicense {
        
        /// . overrides extention mint() to disable
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        fn mint(&mut self, _recipient: AccountId, _id: Id) -> Result<(), PSP34Error> {

            return Err(PSP34Error::Custom(format!("The default mint function is disabled.")))

        }
    }

    impl BouncerLicense {

        /// . initialize contract
        #[ink(constructor)]
        pub fn new(
        ) -> Self {

            ink_lang::codegen::initialize_contract(|contract: &mut Self| {
                
                contract._init_with_owner(contract.env().caller());
                contract.next_bouncerlicense_id = 0;

				let collection_id = contract.collection_id();
				contract._set_attribute(
                    collection_id.clone(),
                    String::from("name").into_bytes(),
                    String::from("Interlock Access NFT").into_bytes(),
                );
				contract._set_attribute(
                    collection_id.clone(),
                    String::from("symbol").into_bytes(),
                    String::from("ILOCKACCESS").into_bytes(),
                );
				contract._set_attribute(
                    collection_id,
                    String::from("ACCESS_CLASS").into_bytes(),
                    String::from("BOUNCER_LICENSE").into_bytes(),
                );
            })
        }

        /// . mint an NFT bouncer license certificate
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn mint_accessnft(&mut self, recipient: AccountId, jpeg_url: String) -> Result<(), PSP34Error> {

            // make sure cap is not surpassed
            if self.next_bouncerlicense_id >= CAP {
                return Err(PSP34Error::Custom(format!("The NFT cap of {:?} has been met. Cannot mint.", CAP)))
            }

            // mint next token id
            let _ = self._mint_to(recipient, psp34::Id::U16(self.next_bouncerlicense_id))?;

            // get nft collection of recipient if already holding
            let mut collection = match self.nfts_held.get(recipient) {
                Some(vec) => vec,
                None => Vec::new(),
            };

            // add id to recipient's nft collection
            collection.push(self.next_bouncerlicense_id);
            self.nfts_held.insert(recipient, &collection);

            // set metadata specific to token
            
            // where this jpeg lives
            self._set_attribute(
                psp34::Id::U16(self.next_bouncerlicense_id),
                String::from("JPEG").into_bytes(),
                jpeg_url.into_bytes(),
            );

            // initial authentication status is false
            self._set_attribute(
                psp34::Id::U16(self.next_bouncerlicense_id),
                String::from("AUTHENTICATED").into_bytes(),
                vec![0],
            );

            // setup for next mint
            self.next_bouncerlicense_id += 1;

            Ok(())
        }

        /// . get collection of nfts held by particular user
        #[ink(message)]
        pub fn user_collection(&self, user: AccountId) -> Result<Vec<u16>, PSP34Error> {

            // retrieve the collection
            match self.nfts_held.get(user) {
                Some(vec) => Ok(vec),
                None => Err(PSP34Error::Custom(format!("The user {:?} does not have a collection.", user))),
            }
        }

        /// . get NFT mint cap
        #[ink(message)]
        pub fn get_cap(&self) -> Result<u16, PSP34Error> {

            // retrieve and return the cap
            Ok(CAP)
        }

        /// . grant 'authenticated' status to interlocker
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn set_authenticated(&mut self, id: Id) -> Result<(), PSP34Error> {

            self._set_attribute(
                id,
                String::from("AUTHENTICATED").into_bytes(),
                vec![1],
            );

            Ok(())
        }

        /// . revoke 'authenticated' status from interlocker
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn set_not_authenticated(&mut self, id: Id) -> Result<(), PSP34Error> {

            self._set_attribute(
                id,
                String::from("AUTHENTICATED").into_bytes(),
                vec![0],
            );

            Ok(())
        }

        /// . modifies the code which is used to execute calls to this contract address
        /// . this upgrades the token contract logic while using old state
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn upgrade_contract(
            &mut self,
            code_hash: [u8; 32]
        ) -> Result<(), PSP34Error> {

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

        /// . test if the default constructor does its job
        #[ink::test]
        fn constructor_works() {

            let bouncerlicense = BouncerLicense::new();

            // check collection metadata -- unwrap() OK in this testing context
            assert_eq!(
                bouncerlicense.get_attribute(
                    bouncerlicense.collection_id(),
                    String::from("name").into_bytes()).unwrap(),
                String::from("Interlock Access NFT").into_bytes()
            );
            assert_eq!(
                bouncerlicense.get_attribute(
                    bouncerlicense.collection_id(),
                    String::from("symbol").into_bytes()).unwrap(),
                String::from("ILOCKACCESS").into_bytes()
            );
            assert_eq!(
                bouncerlicense.get_attribute(bouncerlicense.collection_id(), String::from("ACCESS_CLASS").into_bytes()).unwrap(),
                String::from("BOUNCER_LICENSE").into_bytes()
            );
            assert_eq!(bouncerlicense.next_bouncerlicense_id, 0);
        }

        /// . test if the vip mint function does its job
        #[ink::test]
        fn mint_bouncerlicense_works() {

            let mut bouncerlicense = BouncerLicense::new();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // mint vip nft -- unwrap() OK in this testing context
            bouncerlicense.mint_bouncerlicense(accounts.bob, "https://www.test.com".to_string()).unwrap();

            // check nft metadata -- unwrap() OK in this testing context
            assert_eq!(bouncerlicense.balance_of(accounts.bob), 1);
            assert_eq!(bouncerlicense.next_bouncerlicense_id, 1);
            assert_eq!(
                bouncerlicense.get_attribute(psp34::Id::U16(0), String::from("JPEG").into_bytes()).unwrap(),
                String::from("https://www.test.com").into_bytes()
            );
            assert_eq!(
                bouncerlicense.get_attribute(psp34::Id::U16(0), String::from("AUTHENTICATED").into_bytes()).unwrap(),
                [0]
            );
            assert_eq!(bouncerlicense.owner_of(psp34::Id::U16(0)).unwrap(), accounts.bob);
        }

        /// . test if the set authenticated funtion does its job
        #[ink::test]
        fn set_authenticated_works() {

            let mut bouncerlicense = BouncerLicense::new();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // mint vip nft -- unwrap() OK in this testing context
            bouncerlicense.mint_bouncerlicense(accounts.bob, "https://www.test.com".to_string()).unwrap();

            // set authenticated -- unwrap() OK in this testing context
            bouncerlicense.set_authenticated(psp34::Id::U16(0)).unwrap();

            // verify authentication
            assert_eq!(
                bouncerlicense.get_attribute(psp34::Id::U16(0), String::from("AUTHENTICATED").into_bytes()).unwrap(),
                [1]
            );
        }

        /// . test if the set not authenticated funtion does its job
        #[ink::test]
        fn set_not_authenticated_works() {

            let mut bouncerlicense = BouncerLicense::new();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // mint vip nft -- unwrap() OK in this testing context
            bouncerlicense.mint_bouncerlicense(accounts.bob, "https://www.test.com".to_string()).unwrap();

            // set authenticated -- unwrap() OK in this testing context
            bouncerlicense.set_authenticated(psp34::Id::U16(0)).unwrap();

            // set not authenticated -- unwrap() OK in this testing context
            bouncerlicense.set_not_authenticated(psp34::Id::U16(0)).unwrap();

            // verify authentication
            assert_eq!(
                bouncerlicense.get_attribute(psp34::Id::U16(0), String::from("AUTHENTICATED").into_bytes()).unwrap(),
                [0]
            );
        }

        // . test if the overridden transfer funtion does its job
        // .   . AUTHENTICATED flips to zero on transfer, verified on testnet
        //
        // . test if contract upgrade function does its job
        // .   . logic upgrades on contract set_code_hash, verified on testnet

    }
}

