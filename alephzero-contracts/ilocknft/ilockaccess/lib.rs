//
// INTERLOCK NETWORK - PSP34 BOUNCER LICENSE CONTRACT
//
// !!!!! INCOMPLETE AND UNAUDITED, WARNING !!!!!
//
// This is a standard ERC721-style token contract
// with provisions for enforcing proof of Bouncer
// NFT license ownership.

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod ilockbouncerlicense {
    use ink_storage::{
        traits::SpreadAllocate,
        Mapping,
    };
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

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ILOCKbouncerLicense {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        pause: pausable::Data,
        next_license_id: u16,
        next_membership_id: u16,
        authenticated: Mapping<(AccountId, u16), bool>,
    }

    impl PSP34          for ILOCKbouncerLicense {}
    impl PSP34Mintable  for ILOCKbouncerLicense {}
    impl PSP34Metadata  for ILOCKbouncerLicense {}
    impl Ownable        for ILOCKbouncerLicense {}
    impl Pausable       for ILOCKbouncerLicense {}

    impl ILOCKbouncerLicense {

        #[ink(constructor)]
        pub fn new(
        ) -> Self {

            ink_lang::codegen::initialize_contract(|contract: &mut Self| {
                
                contract.next_license_id = 0;
                contract.next_membership_id = 5000;
            })
        }

        #[ink(message)]
        pub fn mint_license(&mut self, recipient: AccountId) -> Result<(), PSP34Error> {

            self._mint_to(recipient, psp34::Id::U16(self.next_license_id));
            self.next_license_id += 1;

            Ok(())
        }
    }
}
