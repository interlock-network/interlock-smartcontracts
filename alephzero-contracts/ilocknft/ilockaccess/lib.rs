//
// INTERLOCK NETWORK - PSP34 ACCESS CONTRACT
//
// INCLUDES:
// - BOUNCER LICENSE NFT CLASS
// - VIP MEMBERSHIP NFT CLASS
// - ...
//
// !!!!! INCOMPLETE AND UNAUDITED, WARNING !!!!!
//
// This is a standard ERC721-style token contract
// with provisions for enforcing proof of Bouncer
// NFT license ownership, proof of VIP membership,
// and other access features in future upgrades.

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod ilockaccess {
    use ink_storage::{
        traits::SpreadAllocate,
        Mapping,
    };
    use ink_prelude::string::String;
    use openbrush::{
        contracts::{
            psp34::extensions::{
                metadata::*,
                mintable::*,
            },
            ownable::*,
            pausable::*,
        },
        modifiers,
        traits::Storage,
    };

    pub const ACCESS_CLASS: &str = "ACCESS_CLASS";
    pub const BOUNCER_LICENSE: &str = "BOUNCER_LICENSE";
    pub const VIP_MEMBERSHIP: &str = "VIP_MEMBERSHIP";

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ILOCKaccess {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        pause: pausable::Data,
        next_bouncerlicense_id: u32,
        next_vipmembership_id: u32,
        authenticated: Mapping<(AccountId, u32), bool>,
    }

    impl PSP34          for ILOCKaccess {}
    impl PSP34Mintable  for ILOCKaccess {}
    impl PSP34Metadata  for ILOCKaccess {}
    impl Ownable        for ILOCKaccess {}
    impl Pausable       for ILOCKaccess {}

    impl ILOCKaccess {

        #[ink(constructor)]
        pub fn new(
        ) -> Self {

            ink_lang::codegen::initialize_contract(|contract: &mut Self| {
                
                contract._init_with_owner(contract.env().caller());
                contract.next_bouncerlicense_id = 0;
                contract.next_vipmembership_id = 10_000;

				let collection_id = contract.collection_id();
				contract._set_attribute(
                    collection_id.clone(),
                    String::from("name").into_bytes(),
                    String::from("Interlock Access NFTs").into_bytes(),
                );
				contract._set_attribute(
                    collection_id,
                    String::from("symbol").into_bytes(),
                    String::from("ILOCKACCESS").into_bytes(),
                );
            })
        }

        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn mint_bouncerlicense(&mut self, recipient: AccountId) -> Result<(), PSP34Error> {

            self._mint_to(recipient, psp34::Id::U32(self.next_bouncerlicense_id));
            self._set_attribute(
                psp34::Id::U32(self.next_bouncerlicense_id),
                ACCESS_CLASS.as_bytes().to_vec(),
                BOUNCER_LICENSE.as_bytes().to_vec(),
            );
            self.next_bouncerlicense_id += 1;

            Ok(())
        }

        #[openbrush::modifiers(only_owner)]
        #[ink(message)]
        pub fn mint_vipmembership(&mut self, recipient: AccountId) -> Result<(), PSP34Error> {

            self._mint_to(recipient, psp34::Id::U32(self.next_vipmembership_id));
            self._set_attribute(
                psp34::Id::U32(self.next_vipmembership_id),
                ACCESS_CLASS.as_bytes().to_vec(),
                VIP_MEMBERSHIP.as_bytes().to_vec(),
            );
            self.next_vipmembership_id += 1;


            Ok(())
        }
    }
}
