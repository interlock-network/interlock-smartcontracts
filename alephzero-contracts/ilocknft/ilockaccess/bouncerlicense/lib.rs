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

    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::vec;
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
    }

    impl PSP34 for BouncerLicense {

        /// . override transfer function to reset each NFT to 'not authenticated' on transfer
        #[ink(message)]
        fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {

            self._transfer_token(to, id.clone(), data)?;
            self._set_attribute(
                id,
                String::from("AUTHENTICATED").into_bytes(),
                vec![0],
            );

            Ok(())
        }

        // no transfer from function for PSP34
    }

    impl PSP34Metadata for BouncerLicense {}
    impl Ownable for BouncerLicense {}
    impl PSP34Mintable for BouncerLicense {
        
        /// . mint general NFT
        /// . overrides extention mint() to enforce only_owner modifier
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        fn mint(&mut self, recipient: AccountId, id: Id) -> Result<(), PSP34Error> {

            self._mint_to(recipient, id)?;

            Ok(())
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

        /// . mint an NFT VIP membership certificate
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn mint_bouncerlicense(&mut self, recipient: AccountId, jpeg_url: String) -> Result<(), PSP34Error> {

            // mint next token id
            self._mint_to(recipient, psp34::Id::U16(self.next_bouncerlicense_id))?;

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
}

