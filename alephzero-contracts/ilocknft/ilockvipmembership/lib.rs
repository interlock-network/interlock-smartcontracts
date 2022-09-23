//
// INTERLOCK NETWORK - PSP34 VIP MEMBERSHIP CONTRACT
//
// !!!!! INCOMPLETE AND UNAUDITED, WARNING !!!!!
//
// This is a standard ERC721-style token contract
// with provisions for enforcing proof of interlocker
// VIP membership ownership.

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod ilockvipmembership {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::{
            psp34::extensions::{
                metadata::*,
                mintable::*,
            },
            ownable::*,
            pausable::*,
            access_control::*,
        },
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct ILOCKvipMembership {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        pause: pausable::Data,
        next_id: u8,
    }

    impl PSP34          for ILOCKvipMembership {}
    impl PSP34Mintable  for ILOCKvipMembership {}
    impl PSP34Metadata  for ILOCKvipMembership {}
    impl AccessControl  for ILOCKvipMembership {}
    impl Ownable        for ILOCKvipMembership {}
    impl Pausable       for ILOCKvipMembership {}


    impl ILOCKvipMembership {

        #[ink(constructor)]
        pub fn new(
        ) -> Self {

            ink_lang::codegen::initialize_contract(|_contract: &mut Self| {})
        }
    }
}
