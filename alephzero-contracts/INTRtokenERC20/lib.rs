// INTERLOCK NETWORK
//
// blairmunroakusa@1019Fri.17Jun22.anch.AK:br
//
// THIS IS A TEST ERC20 CONTRACT TO DETERMINE HOW
// EASILY ONE CAN SPIN UP A TOKEN ON ALEPH ZERO
// USING INK! FRAMEWORK


#![allow(non_snake_case)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;


#[ink::contract]
mod INTRtokenERC20 {

    use ink_lang::utils::initialize_contract;
    use ink_prelude::string::String;
    use ink_prelude::string::ToString;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;

    /// defines contract storage
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct INTRTOKENERC20 {
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: u32,
        balances: Mapping<AccountId, u32>,
        allowances: Mapping<(AccountId, AccountId), u32>,
    }


    /// specify ERC-20 error type
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {

        /// return if the balance cannot fulfill a request
        InsufficientBalance,
    }

    /// specify the ERC-20 result type
    pub type Result<T> = core::result::Result<T, Error>;

    /// specify transfer event
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        amount: u32,
    }

    /// specify approve event
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        spender: Option<AccountId>,
        amount: u32,
    }


    impl INTRTOKENERC20 {

        /// Constructor that initializes the `bool` value to the given `init_value`
        #[ink(constructor)]
        pub fn new_token(supply: u32) -> Self {

            // create contract
            initialize_contract(|contract: &mut Self| {

                // define owner as caller
                let caller = Self::env().caller();

                // mint
                contract.balances.insert(&caller, &supply);
                contract.total_supply = supply;

                // emit Transfer event
                Self::env().emit_event(Transfer {
                    from: None,
                    to: Some(caller),
                    amount: supply,
                });

                // set optional metadata
                contract.name = "Interlock Network".to_string();
                contract.symbol = "INTR".to_string();
                contract.decimals = 18;
            })
        }

        /// token decimal count getter
        #[ink(message)]
        pub fn name(&self) -> String {

            self.name.clone()
        }

        /// token decimal count getter
        #[ink(message)]
        pub fn symbol(&self) -> String {

            self.symbol.clone()
        }

        /// token decimal count getter
        #[ink(message)]
        pub fn decimals(&self) -> u8 {

            self.decimals
        }

        /// total supply getter
        #[ink(message)]
        pub fn total_supply(&self) -> u32 {

            self.total_supply
        }

        /// account balance getter
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> u32 {

            match self.balances.get(&account) {
                Some(value) => value,
                None => 0,
            }
        }

        /// account allowance getter
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> u32 {

            match self.allowances.get((&owner, &spender)) {
                Some(value) => value,
                None => 0,
            }
        }
        
        /// transfer method
        #[ink(message)]
        pub fn transfer(&mut self, recipient: AccountId, amount: u32) -> bool {

            // get caller information
            let sender = self.env().caller();
            let sender_balance = self.balance_of(sender);

            // make sure balance is adequate
            if sender_balance < amount {
                ink_env::debug_println!("Insufficient balance");
                return false
            }

            // update balances
            let recipient_balance = self.balance_of(recipient);
            self.balances.insert(sender, &(&sender_balance - amount));
            self.balances.insert(recipient, &(&recipient_balance + amount));

            // emit Transfer event
            Self::env().emit_event(Transfer {
                from: Some(sender),
                to: Some(recipient),
                amount: amount,
            });

            true
        }

        /// approve method
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, amount: u32) -> bool {

            // get caller information
            let owner = self.env().caller();

            // add/update approval amount
            self.allowances.insert((&owner, &spender), &amount);

            // emit Approval event
            self.env().emit_event(Approval {
                owner: Some(owner),
                spender: Some(spender),
                amount,
            });

            true
        }

        /// transfer from method
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: u32,
        ) -> bool {

            // get owner balance
            let from_balance = self.balance_of(from);

            // make sure balance is adequate
            if from_balance < amount {
                ink_env::debug_println!("Insufficient balance");
                return false
            }

            // update balances
            self.balances.insert(from, &(from_balance - amount));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + amount));

            // emit Transfer event
            Self::env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                amount: amount,
            });

            true
        }

    }

/////////////////////////////////////////////////////////////////////////


    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        use ink_env::Clear;
        use ink_env::topics::PrefixedValue;

        type Event = <INTRTOKENERC20 as ::ink_lang::reflect::ContractEventBase>::Type;


        /// test if the default constructor does its job
        #[ink::test]
        fn constructor_works() {
            let INTRtokenERC20 = INTRTOKENERC20::new_token(100_000);

            // Transfer event triggered during initial construction.
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100000,
            );

            assert_eq!(INTRtokenERC20.name(), "Interlock Network");
            assert_eq!(INTRtokenERC20.symbol(), "INTR");
            assert_eq!(INTRtokenERC20.decimals(), 18);
            assert_eq!(INTRtokenERC20.total_supply(), 100_000);

        }

        /// test if name getter does its job
        #[ink::test]
        fn name_works() {
            let INTRtokenERC20 = INTRTOKENERC20::new_token(100_000);
            assert_eq!(INTRtokenERC20.name(), "Interlock Network");
        }

        /// test if symbol getter does its job
        #[ink::test]
        fn symbol_works() {
            let INTRtokenERC20 = INTRTOKENERC20::new_token(100_000);
            assert_eq!(INTRtokenERC20.symbol(), "INTR");
        }
        
        /// test if decimals getter does its job
        #[ink::test]
        fn decimals_works() {
            let INTRtokenERC20 = INTRTOKENERC20::new_token(100_000);
            assert_eq!(INTRtokenERC20.decimals(), 18);
        }

        /// test if total supply getter does its job
        #[ink::test]
        fn total_supply_works() {
            let INTRtokenERC20 = INTRTOKENERC20::new_token(100_000);
            assert_eq!(INTRtokenERC20.total_supply(), 100_000);
        }

        /// test if balance getter does its job
        #[ink::test]
        fn balance_of_works() {

            // construct contract and initialize accounts
            let INTRtokenERC20 = INTRTOKENERC20::new_token(100_000);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice owns all the tokens on contract instantiation
            assert_eq!(INTRtokenERC20.balance_of(accounts.alice), 100_000);

            // Bob does not owns tokens
            assert_eq!(INTRtokenERC20.balance_of(accounts.bob), 0);
        }

        /// test if allowance getter does its job
        #[ink::test]
        fn allowance_works() {

            // construct contract and initialize accounts
            let mut INTRtokenERC20 = INTRTOKENERC20::new_token(100_000);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice has not yet approved Bob
            assert_eq!(INTRtokenERC20.allowance(accounts.alice, accounts.bob), 0);

            // Alice approves Bob for tokens
            assert_eq!(INTRtokenERC20.approve(accounts.bob, 10), true);

            // Bob's new allowance reflects this approval
            assert_eq!(INTRtokenERC20.allowance(accounts.alice, accounts.bob), 10);
        }

        /// test if the transfer doer does its job
        #[ink::test]
        fn transfer_works() {

            // construct contract and initialize accounts
            let mut INTRtokenERC20 = INTRTOKENERC20::new_token(100_000);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice transfers tokens to Bob
            assert_eq!(INTRtokenERC20.transfer(accounts.bob, 10), true);

            // Alice balance reflects transfer
            assert_eq!(INTRtokenERC20.balance_of(accounts.alice), 99_990);

            // Bob balance reflects transfer
            assert_eq!(INTRtokenERC20.balance_of(accounts.bob), 10);

            // Alice attempts transfer too large
            assert_eq!(INTRtokenERC20.transfer(accounts.bob, 100_000), false);

            // check all events that happened during the previous calls
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);

            // check the transfer event relating to the actual trasfer
            assert_transfer_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        /// test if the approve does does its job
        #[ink::test]
        fn approve_works() {

            // construct contract and initialize accounts
            let mut INTRtokenERC20 = INTRTOKENERC20::new_token(100_000);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice approves bob to spend tokens
            assert_eq!(INTRtokenERC20.approve(accounts.bob, 10), true);

            // Bob is approved to spend tokens owned by Alice
            assert_eq!(INTRtokenERC20.allowance(accounts.alice, accounts.bob), 10);

            // check all events that happened during previous calls
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);

            // check the approval event relating to the actual approval
            assert_approval_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        /// test if the transfer-from doer does its job
        #[ink::test]
        fn transfer_from_works() {

            // construct contract and initialize accounts
            let mut INTRtokenERC20 = INTRTOKENERC20::new_token(100_000);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // set the contract as callee and Bob as caller
            let contract = ink_env::account_id::<ink_env::DefaultEnvironment>();
            ink_env::test::set_callee::<ink_env::DefaultEnvironment>(contract);
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.bob);

            // Alice approves Bob for token transfers on her behalf
            assert_eq!(INTRtokenERC20.approve(accounts.bob, 10), true);

            // Bob transfers tokens from Alice to Eve
            assert_eq!(INTRtokenERC20.transfer_from(accounts.alice, accounts.eve, 10), true);

            // Eve received the tokens
            assert_eq!(INTRtokenERC20.balance_of(accounts.eve), 10);

            // Bob attempts a transferfrom too large
            assert_eq!(INTRtokenERC20.transfer_from(accounts.alice, accounts.eve, 100_000), false);

            // check all events that happened during the previous calls
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);

            // check that Transfer event was emitted        
            assert_transfer_event(
                &emitted_events[2],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x05; 32])),
                10,
            );
        }

        /// check that a transfer event is good
        fn assert_transfer_event(
            event: &ink_env::test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_amount: u32,
        ) {

            // decode Event object
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");

            // check event is expected transfer event
            if let Event::Transfer(Transfer { from, to, amount }) = decoded_event {

                // make sure accounts match what we expect
                assert_eq!(from, expected_from, "encountered invalid Transfer.from");
                assert_eq!(to, expected_to, "encountered invalid Transfer.to");
                assert_eq!(amount, expected_amount, "encountered invalid Trasfer.amount");
            } else {
                panic!("encountered unexpected event kind: expected a Transfer event")
            }

            // define expected topics for Transfer event
            let expected_topics = vec![
                encoded_into_hash(&PrefixedValue {
                    value: b"INTRTOKENERC20::Transfer",
                    prefix: b"",
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"INTRTOKENERC20::Transfer::from",
                    value: &expected_from,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"INTRTOKENERC20::Transfer::to",
                    value: &expected_to,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"INTRTOKENERC20::Transfer::amount",
                    value: &expected_amount,
                }),
            ];

            // get actual topics for event
            let topics = event.topics.clone();

            // check that actual topics match expected topics
            for (n, (actual_topic, expected_topic)) in
                topics.iter().zip(expected_topics).enumerate()
            {
                let mut topic_hash = Hash::clear();
                let len = actual_topic.len();
                topic_hash.as_mut()[0..len].copy_from_slice(&actual_topic[0..len]);
                assert_eq!(topic_hash, expected_topic, "encountered invalid topic at {}", n);
            }
        }

        /// check that an approval event is good
        fn assert_approval_event(
            event: &ink_env::test::EmittedEvent,
            expected_owner: Option<AccountId>,
            expected_spender: Option<AccountId>,
            expected_amount: u32,
        ) {

            // decode Event object
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");

            // check event is expected approval event
            if let Event::Approval(Approval { owner, spender, amount }) = decoded_event {
                assert_eq!(owner, expected_owner, "encountered invalid Approval.owner");
                assert_eq!(spender, expected_spender, "encountered invalid Approval.spender");
                assert_eq!(amount, expected_amount, "encountered invalid Approval.amount");
            } else {
                panic!("encountered unexpected event kind: expected a Approval event")
            }

            // define expected topics for Approval event
            let expected_topics = vec![
                encoded_into_hash(&PrefixedValue {
                    value: b"INTRTOKENERC20::Approval",
                    prefix: b"",
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"INTRTOKENERC20::Approval::owner",
                    value: &expected_owner,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"INTRTOKENERC20::Approval::spender",
                    value: &expected_spender,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"INTRTOKENERC20::Approval::amount",
                    value: &expected_amount,
                }),
            ];

            // get actual topics for event
            let topics = event.topics.clone();

            // check that actual topics match expected topics
            for (n, (actual_topic, expected_topic)) in
                topics.iter().zip(expected_topics).enumerate()
            {
                let mut topic_hash = Hash::clear();
                let len = actual_topic.len();
                topic_hash.as_mut()[0..len].copy_from_slice(&actual_topic[0..len]);
                assert_eq!(topic_hash, expected_topic, "encountered invalid topic at {}", n);
            }
        }

        /// this is a painful hashing function for use in event assert functions
        fn encoded_into_hash<T>(entity: &T) -> Hash
        where
            T: scale::Encode,
        {
            use ink_env::{
                hash::{
                    Blake2x256,
                    CryptoHash,
                    HashOutput,
                },
                Clear,
            };
            let mut result = Hash::clear();
            let len_result = result.as_ref().len();
            let encoded = entity.encode();
            let len_encoded = encoded.len();
            if len_encoded <= len_result {
                result.as_mut()[..len_encoded].copy_from_slice(&encoded);
                return result
            }
            let mut hash_output =
                <<Blake2x256 as HashOutput>::Type as Default>::default();
            <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
            let copy_len = core::cmp::min(hash_output.len(), len_result);
            result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
            result
        }
    }

}
