//
// INTERLOCK NETWORK - 
// PSP34 ACCESS CONTRACT - VIP MEMBERSHIP
//
// !!!!! INCOMPLETE AND UNAUDITED, WARNING !!!!!
//
// This is a standard ERC721-style token contract
// with provisions for enforcing proof of VIP membership,

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod vipmembership {

    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::string::String;
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
    pub struct VIPmembership {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        next_vipmembership_id: u16,
    }

    impl PSP34          for VIPmembership {}
    impl PSP34Metadata  for VIPmembership {}
    impl Ownable        for VIPmembership {}
    impl PSP34Mintable  for VIPmembership {
        
        /// . mint general NFT
        /// . overrides extention mint() to enforce only_owner modifier
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        fn mint(&mut self, recipient: AccountId, id: Id) -> Result<(), PSP34Error> {

            self._mint_to(recipient, id)?;

            Ok(())
        }
    }

    impl VIPmembership {

        /// . initialize contract
        #[ink(constructor)]
        pub fn new(
        ) -> Self {

            ink_lang::codegen::initialize_contract(|contract: &mut Self| {
                
                contract._init_with_owner(contract.env().caller());
                contract.next_vipmembership_id = 0;

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
                    String::from("VIP_MEMBERSHIP").into_bytes(),
                );
            })
        }

        /// . mint an NFT VIP membership certificate
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn mint_vipmembership(&mut self, recipient: AccountId, jpeg_url: String) -> Result<(), PSP34Error> {

            // mint next token id
            self._mint_to(recipient, psp34::Id::U16(self.next_vipmembership_id))?;

            // set metadata specific to token
            
            // where this jpeg lives
            self._set_attribute(
                psp34::Id::U16(self.next_vipmembership_id),
                String::from("JPEG").into_bytes(),
                jpeg_url.into_bytes(),
            );

            // initial authentication status is false
            self._set_attribute(
                psp34::Id::U16(self.next_vipmembership_id),
                String::from("AUTHENTICATED").into_bytes(),
                [0_u8; 1].to_vec(),
            );

            // setup for next mint
            self.next_vipmembership_id += 1;

            Ok(())
        }

        /// . grant 'authenticated' status to interlocker
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn set_authenticated(&mut self, id: u16) -> Result<(), PSP34Error> {

            self._set_attribute(
                psp34::Id::U16(id),
                String::from("AUTHENTICATED").into_bytes(),
                [1_u8; 1].to_vec(),
            );

            Ok(())
        }

        /// . revoke 'authenticated' status from interlocker
        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn set_not_authenticated(&mut self, id: u16) -> Result<(), PSP34Error> {

            self._set_attribute(
                psp34::Id::U16(id),
                String::from("AUTHENTICATED").into_bytes(),
                [0_u8; 1].to_vec(),
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
