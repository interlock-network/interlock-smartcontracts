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
    use ink_prelude::{
        string::String,
        vec::Vec,
        vec,
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

    impl PSP34 for VIPmembership {

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

        // no transfer_from function for PSP34
    }

    impl PSP34Metadata for VIPmembership {}
    impl Ownable for VIPmembership {}
    impl PSP34Mintable for VIPmembership {
        
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
        pub fn mint_accessnft(&mut self, recipient: AccountId, jpeg_url: String) -> Result<(), PSP34Error> {

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
                vec![0],
            );

            // setup for next mint
            self.next_vipmembership_id += 1;

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

            let vipmembership = VIPmembership::new();

            // check collection metadata -- unwrap() OK in this testing context
            assert_eq!(
                vipmembership.get_attribute(vipmembership.collection_id(), String::from("name").into_bytes()).unwrap(),
                String::from("Interlock Access NFT").into_bytes()
            );
            assert_eq!(
                vipmembership.get_attribute(vipmembership.collection_id(), String::from("symbol").into_bytes()).unwrap(),
                String::from("ILOCKACCESS").into_bytes()
            );
            assert_eq!(
                vipmembership.get_attribute(vipmembership.collection_id(), String::from("ACCESS_CLASS").into_bytes()).unwrap(),
                String::from("VIP_MEMBERSHIP").into_bytes()
            );
            assert_eq!(vipmembership.next_vipmembership_id, 0);
        }

        /// . test if the vip mint function does its job
        #[ink::test]
        fn mint_vipmembership_works() {

            let mut vipmembership = VIPmembership::new();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // mint vip nft -- unwrap() OK in this testing context
            vipmembership.mint_vipmembership(accounts.bob, "https://www.test.com".to_string()).unwrap();

            // check nft metadata -- unwrap() OK in this testing context
            assert_eq!(vipmembership.balance_of(accounts.bob), 1);
            assert_eq!(vipmembership.next_vipmembership_id, 1);
            assert_eq!(
                vipmembership.get_attribute(psp34::Id::U16(0), String::from("JPEG").into_bytes()).unwrap(),
                String::from("https://www.test.com").into_bytes()
            );
            assert_eq!(
                vipmembership.get_attribute(psp34::Id::U16(0), String::from("AUTHENTICATED").into_bytes()).unwrap(),
                [0]
            );
            assert_eq!(vipmembership.owner_of(psp34::Id::U16(0)).unwrap(), accounts.bob);
        }

        /// . test if the set authenticated funtion does its job
        #[ink::test]
        fn set_authenticated_works() {

            let mut vipmembership = VIPmembership::new();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // mint vip nft -- unwrap() OK in this testing context
            vipmembership.mint_vipmembership(accounts.bob, "https://www.test.com".to_string()).unwrap();

            // set authenticated -- unwrap() OK in this testing context
            vipmembership.set_authenticated(psp34::Id::U16(0)).unwrap();

            // verify authentication
            assert_eq!(
                vipmembership.get_attribute(psp34::Id::U16(0), String::from("AUTHENTICATED").into_bytes()).unwrap(),
                [1]
            );
        }

        /// . test if the set not authenticated funtion does its job
        #[ink::test]
        fn set_not_authenticated_works() {

            let mut vipmembership = VIPmembership::new();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // mint vip nft -- unwrap() OK in this testing context
            vipmembership.mint_vipmembership(accounts.bob, "https://www.test.com".to_string()).unwrap();

            // set authenticated -- unwrap() OK in this testing context
            vipmembership.set_authenticated(psp34::Id::U16(0)).unwrap();

            // set not authenticated -- unwrap() OK in this testing context
            vipmembership.set_not_authenticated(psp34::Id::U16(0)).unwrap();

            // verify authentication
            assert_eq!(
                vipmembership.get_attribute(psp34::Id::U16(0), String::from("AUTHENTICATED").into_bytes()).unwrap(),
                [0]
            );
        }


        // no transfer_from function for PSP34
    
        // . test if the overridden transfer funtion does its job
        // .    . AUTHENTICATED flips to zero on transfer, verified on testnet
        //
        // . test if contract upgrade function does its job
        // .    . logic upgrades on contract set_code_hash, verified on testnet

    }
}

