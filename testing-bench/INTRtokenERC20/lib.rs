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

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct IntRtokenErc20 {
        /// Stores a single `bool` value on the storage.
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: u32,
        balances: Mapping<AccountId, u32>,
        allowances: Mapping<(AccountId, AccountId), u32>,
    }


    /// Specify ERC-20 error type.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Return if the balance cannot fulfill a request.
        InsufficientBalance,
    }

    /// Specify the ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    /// transfer event
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        amount: u32,
    }

    /// approve event
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: Option<AccountId>,
        #[ink(topic)]
        spender: Option<AccountId>,
        amount: u32,
    }


    impl IntRtokenErc20 {


        /// Constructor that initializes the `bool` value to the given `init_value`
        #[ink(constructor)]
        pub fn new_token(supply: u32) -> Self {
            initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.balances.insert(&caller, &supply);
                contract.total_supply = supply;
                Self::env().emit_event(Transfer {
                    from: None,
                    to: Some(caller),
                    amount: supply,
                });
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
            let sender = self.env().caller();
            let sender_balance = self.balance_of(sender);
            if sender_balance < amount {
                ink_env::debug_println!("Insufficient balance");
                return false
            }
            let recipient_balance = self.balance_of(recipient);
            self.balances.insert(sender, &(&sender_balance - amount));
            self.balances.insert(recipient, &(&recipient_balance + amount));
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
            let owner = self.env().caller();
            // here, make sure owner and spender addresses are not 0 address
            self.allowances.insert((&owner, &spender), &amount);
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
            let from_balance = self.balance_of(from);
            if from_balance < amount {
                return false
            }
            self.balances.insert(from, &(from_balance - amount));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + amount));
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

        type Event = <IntRtokenErc20 as ::ink_lang::reflect::ContractEventBase>::Type;


        /// We test if the default constructor does its job.

        #[ink::test]
        fn constructor_works() {
            let INTRtokenERC20 = IntRtokenErc20::new_token(100_000);

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

        #[ink::test]
        fn name_works() {
            let INTRtokenERC20 = IntRtokenErc20::new_token(100_000);
            assert_eq!(INTRtokenERC20.name(), "Interlock Network");
        }

        #[ink::test]
        fn symbol_works() {
            let INTRtokenERC20 = IntRtokenErc20::new_token(100_000);
            assert_eq!(INTRtokenERC20.symbol(), "INTR");
        }
        
        #[ink::test]
        fn decimals_works() {
            let INTRtokenERC20 = IntRtokenErc20::new_token(100_000);
            assert_eq!(INTRtokenERC20.decimals(), 18);
        }

        #[ink::test]
        fn total_supply_works() {
            let INTRtokenERC20 = IntRtokenErc20::new_token(100_000);
            assert_eq!(INTRtokenERC20.total_supply(), 100_000);
        }
        #[ink::test]
        fn balance_of_works() {
            let INTRtokenERC20 = IntRtokenErc20::new_token(100_000);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice owns all the tokens on contract instantiation
            assert_eq!(INTRtokenERC20.balance_of(accounts.alice), 100_000);

            // Bob does not owns tokens
            assert_eq!(INTRtokenERC20.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut INTRtokenERC20 = IntRtokenErc20::new_token(100_000);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice transfers 10 tokens to Bob.
            assert_eq!(INTRtokenERC20.transfer(accounts.bob, 10), true);
            assert_eq!(INTRtokenERC20.balance_of(accounts.alice), 99_990);
            assert_eq!(INTRtokenERC20.balance_of(accounts.bob), 10);

            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);

            // Check the transfer event relating to the actual trasfer.
            assert_transfer_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        #[ink::test]
        fn approve_works() {
            let mut INTRtokenERC20 = IntRtokenErc20::new_token(100_000);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Alice approves bob to spend 10 tokens
            assert_eq!(INTRtokenERC20.approve(accounts.bob, 10), true);
            assert_eq!(INTRtokenERC20.allowance(accounts.alice, accounts.bob), 10);

            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);

            // Check the transfer event relating to the actual trasfer.
            assert_approval_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        #[ink::test]
        fn allowance_works() {
            let mut INTRtokenERC20 = IntRtokenErc20::new_token(100_000);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            assert_eq!(INTRtokenERC20.allowance(accounts.alice, accounts.bob), 0);
            assert_eq!(INTRtokenERC20.approve(accounts.bob, 10), true);
            assert_eq!(INTRtokenERC20.allowance(accounts.alice, accounts.bob), 10);
        }

        #[ink::test]
        fn transfer_from_works() {
            let mut INTRtokenERC20 = IntRtokenErc20::new_token(100_000);
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();

            // Set the contract as callee and Bob as caller.
            let contract = ink_env::account_id::<ink_env::DefaultEnvironment>();
            ink_env::test::set_callee::<ink_env::DefaultEnvironment>(contract);
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.bob);

            // Alice approves Bob for token transfers on her behalf.
            assert_eq!(INTRtokenERC20.approve(accounts.bob, 10), true);

            // Bob transfers tokens from Alice to Eve.
            assert_eq!(
                INTRtokenERC20.transfer_from(accounts.alice, accounts.eve, 10),
                true
            );
            // Eve owns tokens.
            assert_eq!(INTRtokenERC20.balance_of(accounts.eve), 10);

            // Check all transfer events that happened during the previous calls:
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);

            // The second event `emitted_events[1]` is an Approve event that we skip checking.
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
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::Transfer(Transfer { from, to, amount }) = decoded_event {
                assert_eq!(from, expected_from, "encountered invalid Transfer.from");
                assert_eq!(to, expected_to, "encountered invalid Transfer.to");
                assert_eq!(amount, expected_amount, "encountered invalid Trasfer.amount");
            } else {
                panic!("encountered unexpected event kind: expected a Transfer event")
            }
            let expected_topics = vec![
                encoded_into_hash(&PrefixedValue {
                    value: b"IntRtokenErc20::Transfer",
                    prefix: b"",
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"IntRtokenErc20::Transfer::from",
                    value: &expected_from,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"IntRtokenErc20::Transfer::to",
                    value: &expected_to,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"IntRtokenErc20::Transfer::amount",
                    value: &expected_amount,
                }),
            ];

            let topics = event.topics.clone();
            for (n, (actual_topic, expected_topic)) in
                topics.iter().zip(expected_topics).enumerate()
            {
                let mut topic_hash = Hash::clear();
                let len = actual_topic.len();
                topic_hash.as_mut()[0..len].copy_from_slice(&actual_topic[0..len]);

                assert_eq!(
                    topic_hash, expected_topic,
                    "encountered invalid topic at {}",
                    n
                );
            }
        }

        /// check that a approval event is good
        fn assert_approval_event(
            event: &ink_env::test::EmittedEvent,
            expected_owner: Option<AccountId>,
            expected_spender: Option<AccountId>,
            expected_amount: u32,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::Approval(Approval { owner, spender, amount }) = decoded_event {
                assert_eq!(owner, expected_owner, "encountered invalid Approval.owner");
                assert_eq!(spender, expected_spender, "encountered invalid Approval.spender");
                assert_eq!(amount, expected_amount, "encountered invalid Approval.amount");
            } else {
                panic!("encountered unexpected event kind: expected a Approval event")
            }
            let expected_topics = vec![
                encoded_into_hash(&PrefixedValue {
                    value: b"IntRtokenErc20::Approval",
                    prefix: b"",
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"IntRtokenErc20::Approval::owner",
                    value: &expected_owner,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"IntRtokenErc20::Approval::spender",
                    value: &expected_spender,
                }),
                encoded_into_hash(&PrefixedValue {
                    prefix: b"IntRtokenErc20::Approval::amount",
                    value: &expected_amount,
                }),
            ];

            let topics = event.topics.clone();
            for (n, (actual_topic, expected_topic)) in
                topics.iter().zip(expected_topics).enumerate()
            {
                let mut topic_hash = Hash::clear();
                let len = actual_topic.len();
                topic_hash.as_mut()[0..len].copy_from_slice(&actual_topic[0..len]);

                assert_eq!(
                    topic_hash, expected_topic,
                    "encountered invalid topic at {}",
                    n
                );
            }
        }


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
