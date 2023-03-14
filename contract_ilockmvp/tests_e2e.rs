//
// INTERLOCK NETWORK MVP SMART CONTRACT END-TO-END TESTS
//
// End to end tests are used extensively becaus using the Openbrush
// PSP22 framework involves cross-contract invocations under the hood.
// EG/IE, If I want to reward an Interlocker, that involves an internal
// call of the OpenBrush PSP22 transfer message. I know of no way to
// get around this fact for testing besides using end-to-end tests.
//
// ##### to setup for e2e testin, run
//
// substrate-contracts-node
// 
// ##### after installing by running
//
// cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git
//
// ##### To view debug prints and assertion failures run test via:
//      
// cargo +nightly test --features e2e-tests -- --show-output
//
// ##### To view debug for specific method run test via:
//      
// cargo nightly+ test <test_function_here> -- --nocapture
//
// ! NB ! During test build and runtime, if you ever come across errors
//        saying 'Metadata artifacts not generated' or 'Once instance
//        has previously been poisoned', then you need to run `cargo clean`
//        or delete the `target` directory the build/run from scratch.
//        OR
//        Save both the lib.rs file AND this tests_e2e.rs file, then reattempt.
//

use crate::ilockmvp::*;

#[cfg(all(test, feature = "e2e-tests"))]
use ink_e2e::build_message;

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;


        use openbrush::{
            contracts:: psp22::{
            psp22_external::PSP22,
            extensions::burnable::psp22burnable_external::PSP22Burnable,
            },
            traits::Balance,
        };


        /// HAPPY TRANSFER
        /// - Test if customized transfer function works correctly.
        /// - When transfer from contract owner, circulating supply increases.
        /// - When transfer to contract owner, circulating supply decreases
        /// and rewards pool increases.
        #[ink_e2e::test]
        async fn happye2e_transfer(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

            let constructor = ILOCKmvpRef::new_token();
            let contract_acct_id = client
                .instantiate("ilockmvp", &ink_e2e::alice(), constructor, 0, None)
                .await.expect("instantiate failed").account_id;

            // alice is contract owner
            // transfers 1000 ILOCK from alice to bob and check for resulting Transfer event
            let alice_transfer_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.transfer(bob_account.clone(), 1000, Vec::new()));
            let transfer_response = client
                .call(&ink_e2e::alice(), alice_transfer_msg, 0, None).await.unwrap();

            // filter for transfer event
            let contract_emitted_transfer = transfer_response
                .events
                .iter()
                .find(|event| {
                    event
                        .as_ref()
                        .expect("expected event")
                        .event_metadata()
                        .event()
                        == "ContractEmitted" &&
                        String::from_utf8_lossy(
                            event.as_ref().expect("bad event").bytes()).to_string()
                       .contains("ILOCKmvp::Transfer")
                })
                .expect("Expect ContractEmitted event")
                .unwrap();

            // Decode to the expected event type (skip field_context)
            let transfer_event = contract_emitted_transfer.field_bytes();
            let decoded_transfer =
                <Transfer as scale::Decode>::decode(&mut &transfer_event[35..]).expect("invalid data");

            // Destructor decoded event
            let Transfer { from, to, amount } = decoded_transfer;

            // Assert with the expected value
            assert_eq!(from, Some(alice_account), "encountered invalid Transfer.from");
            assert_eq!(to, Some(bob_account), "encountered invalid Transfer.to");
            assert_eq!(amount, 1000, "encountered invalid Transfer.amount");

            // checks that bob has expected resulting balance
            let bob_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.balance_of(bob_account.clone()));
            let bob_balance = client
                .call_dry_run(&ink_e2e::bob(), &bob_balance_msg, 0, None).await.return_value();
            assert_eq!(0 + 1000, bob_balance);

            // checks that alice has expected resulting balance
            let alice_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.balance_of(alice_account.clone()));
            let alice_balance = client
                .call_dry_run(&ink_e2e::alice(), &alice_balance_msg, 0, None).await.return_value();
            assert_eq!(SUPPLY_CAP - 1000, alice_balance);

            // checks that circulating supply increased appropriately
            let total_supply_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.total_supply());
            let mut total_supply = client
                .call_dry_run(&ink_e2e::alice(), &total_supply_msg, 0, None).await.return_value();
            assert_eq!(0 + 1000, total_supply);

            // transfers 500 ILOCK from bob to alice
            let bob_transfer_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.transfer(alice_account.clone(), 500, Vec::new()));
            let _result = client
                .call(&ink_e2e::bob(), bob_transfer_msg, 0, None).await;

            // checks that circulating supply decreased appropriately
            total_supply = client
                .call_dry_run(&ink_e2e::alice(), &total_supply_msg, 0, None).await.return_value();
            assert_eq!(1000 - 500, total_supply);

            // check that rewards supply increased appropriately
            let rewards_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.pool_balance(REWARDS));
            let rewards_balance = client
                .call_dry_run(&ink_e2e::alice(), &rewards_balance_msg, 0, None).await.return_value().1;
            assert_eq!(POOLS[REWARDS as usize].tokens * DECIMALS_POWER10 + 500, rewards_balance);

            Ok(())
        }

        /// SAD TRANSFER
        /// - Test if customized transfer function fails correctly.
        ///
        /// - Return
        ///     InsufficientBalance     - When caller has allowance < amount
        ///     ZeroRecipientAddress    - when to's address is AccountId::from([0_u8; 32])
        ///     ZeroSenderAddress       - When caller's address is AccountId::from([0_u8; 32])
        ///                               (zero address has known private key..however that works)
        #[ink_e2e::test]
        async fn sade2e_transfer(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            Ok(())
        }

        /// HAPPY TRANSFER_FROM
        /// - Test if customized transfer_from function works correctly.
        /// - When transfer from contract owner, circulating supply increases.
        /// - Transfer and Approval events are emitted.
        /// - When transfer to contract owner, circulating supply decreases
        /// - When caller transfers, their allowace with from decreases
        ///   and rewards pool increases
        #[ink_e2e::test]
        async fn happye2e_transfer_from(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            let charlie_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie);

            let constructor = ILOCKmvpRef::new_token();
            let contract_acct_id = client
                .instantiate("ilockmvp", &ink_e2e::alice(), constructor, 0, None)
                .await.expect("instantiate failed").account_id;

            // alice approves bob 1000 ILOCK
            let alice_approve_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.approve(bob_account.clone(), 1000));
            let _approval_result = client
                .call(&ink_e2e::alice(), alice_approve_msg, 0, None).await;

            // bob transfers 1000 ILOCK from alice to charlie
            let bob_transfer_from_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.transfer_from(
                    alice_account.clone(), charlie_account.clone(), 1000, Vec::new())
            );
            let transfer_from_response = client
                .call(&ink_e2e::bob(), bob_transfer_from_msg, 0, None).await.unwrap();

            // filter for approval event
            let contract_emitted_approval = transfer_from_response
                .events
                .iter()
                .find(|event| {
                    event
                        .as_ref()
                        .expect("expected event")
                        .event_metadata()
                        .event()
                        == "ContractEmitted" &&
                        String::from_utf8_lossy(
                            event.as_ref().expect("bad event").bytes()).to_string()
                       .contains("ILOCKmvp::Approval")
                })
                .expect("Expect ContractEmitted event")
                .unwrap();

            // decode to the expected event type (skip field_context)
            let approval_event = contract_emitted_approval.field_bytes();
            let decoded_approval =
                <Approval as scale::Decode>::decode(&mut &approval_event[35..]).expect("invalid data");

            // destructor decoded eapproval
            let Approval { owner, spender, amount } = decoded_approval;

            // assert with the expected value
            assert_eq!(owner, Some(alice_account), "encountered invalid Approval.owner");
            assert_eq!(spender, Some(bob_account), "encountered invalid Approval.spender");
            assert_eq!(amount, 1000 - 1000, "encountered invalid Approval.amount");

            // filter for transfer event
            let contract_emitted_transfer = transfer_from_response
                .events
                .iter()
                .find(|event| {
                    event
                        .as_ref()
                        .expect("expected event")
                        .event_metadata()
                        .event()
                        == "ContractEmitted" &&
                        String::from_utf8_lossy(
                            event.as_ref().expect("bad event").bytes()).to_string()
                       .contains("ILOCKmvp::Transfer")
                })
                .expect("Expect ContractEmitted event")
                .unwrap();

            // decode to the expected event type (skip field_context)
            let transfer_event = contract_emitted_transfer.field_bytes();
            let decoded_transfer =
                <Transfer as scale::Decode>::decode(&mut &transfer_event[35..]).expect("invalid data");

            // destructor decoded transfer
            let Transfer { from, to, amount } = decoded_transfer;

            // assert with the expected value
            assert_eq!(from, Some(alice_account), "encountered invalid Transfer.from");
            assert_eq!(to, Some(charlie_account), "encountered invalid Transfer.to");
            assert_eq!(amount, 1000, "encountered invalid Transfer.amount");

            // checks that charlie has expected resulting balance
            let charlie_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.balance_of(charlie_account.clone()));
            let charlie_balance = client
                .call_dry_run(&ink_e2e::charlie(), &charlie_balance_msg, 0, None).await.return_value();
            assert_eq!(0 + 1000, charlie_balance);

            // checks that circulating supply increased appropriately
            let total_supply_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.total_supply());
            let mut total_supply = client
                .call_dry_run(&ink_e2e::alice(), &total_supply_msg, 0, None).await.return_value();
            assert_eq!(0 + 1000, total_supply);

            // checks that bob's allowance decreased appropriately
            let bob_allowance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.allowance(alice_account.clone(), bob_account.clone()));
            let bob_allowance = client
                .call_dry_run(&ink_e2e::alice(), &bob_allowance_msg, 0, None).await.return_value();
            assert_eq!(1000 - 1000, bob_allowance);

            // charlie approves bob 1000 ILOCK
            let charlie_approve_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.approve(bob_account.clone(), 1000));
            let _approval_result = client
                .call(&ink_e2e::charlie(), charlie_approve_msg, 0, None).await;

            // bob transfers 1000 ILOCK from charlie to alice
            let bob_transfer_from_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.transfer_from(
                    charlie_account.clone(), alice_account.clone(), 1000, Vec::new()));
            let _transfer_from_result = client
                .call(&ink_e2e::bob(), bob_transfer_from_msg, 0, None).await;

            // checks that circulating supply decreased appropriately
            total_supply = client
                .call_dry_run(&ink_e2e::alice(), &total_supply_msg, 0, None).await.return_value();
            assert_eq!(1000 - 1000, total_supply);

            // check that rewards supply increased appropriately
            let rewards_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.pool_balance(REWARDS));
            let rewards_balance = client
                .call_dry_run(&ink_e2e::alice(), &rewards_balance_msg, 0, None).await.return_value().1;
            assert_eq!(POOLS[REWARDS as usize].tokens * DECIMALS_POWER10 + 1000, rewards_balance);

            Ok(())
        }

        /// SAD TRANSFER_FROM
        /// - Test if customized transfer_from function fails correctly.
        ///
        /// - Return
        ///     InsufficientBalance     - When caller has allowance < amount
        ///     InsufficientAllowance   - When caller specs amount > from's balance
        ///     ZeroRecipientAddress    - when to's address is AccountId::from([0_u8; 32])
        ///     ZeroSenderAddress       - When from's address is AccountId::from([0_u8; 32])
        #[ink_e2e::test]
        async fn sade2e_transfer_from(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            Ok(())
        }

        /// HAPPY BURN
        /// - Test if customized burn function works correctly.
        /// - When burn occurs, donor balance decreases.
        /// - When burn occurs, circulating supply (total_supply()) decreases
        #[ink_e2e::test]
        async fn happye2e_burn(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

            let constructor = ILOCKmvpRef::new_token();
            let contract_acct_id = client
                .instantiate("ilockmvp", &ink_e2e::alice(), constructor, 0, None)
                .await.expect("instantiate failed").account_id;

            // alice transfers 1000 ILOCK to bob (to check !owner burn)
            let alice_transfer_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.transfer(
                    bob_account.clone(), 1000, Vec::new()));
            let _transfer_result = client
                .call(&ink_e2e::alice(), alice_transfer_msg, 0, None).await;

            // alice burns 1000 tokens
            let alice_burn_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.burn(alice_account.clone(), 1000));
            let burn_response = client
                .call(&ink_e2e::alice(), alice_burn_msg, 0, None).await.unwrap();


            let contract_emitted_transfer = burn_response
                .events
                .iter()
                .find(|event| {
                    event
                        .as_ref()
                        .expect("expected event")
                        .event_metadata()
                        .event()
                        == "ContractEmitted" &&
                        String::from_utf8_lossy(
                            event.as_ref().expect("bad event").bytes()).to_string()
                       .contains("ILOCKmvp::Transfer")
                })
                .expect("Expect ContractEmitted event")
                .unwrap();

            // decode to the expected event type (skip field_context)
            let transfer_event = contract_emitted_transfer.field_bytes();
            let decoded_transfer =
                <Transfer as scale::Decode>::decode(&mut &transfer_event[34..]).expect("invalid data");

            // Destructor decoded event
            let Transfer { from, to, amount } = decoded_transfer;

            // Assert with the expected value
            assert_eq!(from, Some(alice_account), "encountered invalid Transfer.fromr");
            assert_eq!(to, None, "encountered invalid Transfer.to");
            assert_eq!(amount, 1000, "encountered invalid Transfer.amount");

            // checks that alice has expected resulting balance
            let alice_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.balance_of(alice_account.clone()));
            let alice_balance = client
                .call_dry_run(&ink_e2e::alice(), &alice_balance_msg, 0, None).await.return_value();
            assert_eq!(SUPPLY_CAP - 1000 - 1000, alice_balance);

            // checks that reward pool decreased appropriately
            let rewards_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.pool_balance(REWARDS));
            let rewards_balance = client
                .call_dry_run(&ink_e2e::alice(), &rewards_balance_msg, 0, None).await.return_value().1;
            assert_eq!(POOLS[REWARDS as usize].tokens * DECIMALS_POWER10 - 1000, rewards_balance);

            // bob burns 500 tokens
            let bob_burn_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.burn(bob_account.clone(), 500));
            let _bob_burn_result = client
                .call(&ink_e2e::alice(), bob_burn_msg, 0, None).await;

            // checks that circulating supply decreased appropriately
            let total_supply_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.total_supply());
            let total_supply = client
                .call_dry_run(&ink_e2e::alice(), &total_supply_msg, 0, None).await.return_value();
            assert_eq!(1000 - 500, total_supply);

            // checks that bob has expected resulting balance
            let bob_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.balance_of(bob_account.clone()));
            let bob_balance = client
                .call_dry_run(&ink_e2e::charlie(), &bob_balance_msg, 0, None).await.return_value();
            assert_eq!(1000 - 500, bob_balance);

            Ok(())
        }

        /// SAD BURN
        /// - Test if customized transfer_from function fails correctly.
        ///
        /// - Return
        ///     CallerNotOwner          - When caller does not own contract
        ///     InsufficientBalance     - When donor's balance < burn amount
        #[ink_e2e::test]
        async fn sade2e_burn(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            Ok(())
        }

        /// HAPPY DISTRIBUTE_TOKENS
        /// - Test if token distribution works as intended per vesting schedule.
        /// - Cycle through entire vesting period.
        /// - Includes optional print table for inspection
        /// - Includes register_stakeholder().
        /// - Includes distribute_tokens().
        #[ink_e2e::test]
        async fn happye2e_distribute_tokens(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            // fire up contract
            let constructor = ILOCKmvpRef::new_token();
            let contract_acct_id = client
                .instantiate("ilockmvp", &ink_e2e::alice(), constructor, 0, None)
                .await.expect("instantiate failed").account_id;

            // register accounts
            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let stakeholder_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            let stakeholder_share = 1_000_000_000;
            let pool_size = POOLS[TEAM_FOUNDERS as usize].tokens * DECIMALS_POWER10;

            // register stakeholder
            let register_stakeholder_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.register_stakeholder(
                    stakeholder_account.clone(), stakeholder_share, TEAM_FOUNDERS));
            let _register_stakeholder_result = client
                .call(&ink_e2e::alice(), register_stakeholder_msg, 0, None).await;

            let cliff = POOLS[TEAM_FOUNDERS as usize].cliffs;
            let vests = POOLS[TEAM_FOUNDERS as usize].vests;
            let schedule_end = vests + cliff - 1;
            let schedule_period = vests;
            let payout = 1_000_000_000 / vests as Balance; // 27_777_777
            let last_payout = payout + 1_000_000_000 % vests as Balance; // 27_777_805

            // check stakeholder_data()
            let stakeholder_data_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.stakeholder_data(stakeholder_account.clone()));
            let stakeholder_data = client
                .call_dry_run(&ink_e2e::alice(), &stakeholder_data_msg, 0, None).await.return_value();
            assert_eq!(stakeholder_data.0.share, stakeholder_share);
            assert_eq!(stakeholder_data.1, stakeholder_data.0.share);
            assert_eq!(stakeholder_data.2, payout);
            assert_eq!(stakeholder_data.3, "team+founders".to_string());

            // iterate through one vesting schedule
            for month in 0..(schedule_end + 2) {

                if month >= cliff && month <= schedule_end {

                    let distribute_tokens_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                        .call(|contract| contract.distribute_tokens(stakeholder_account.clone()));
                    let _distribute_tokens_result = client
                        .call(&ink_e2e::alice(), distribute_tokens_msg, 0, None).await;
                }

                let stakeholder_data_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                    .call(|contract| contract.stakeholder_data(stakeholder_account.clone()));
                let stakeholder_paid = client
                    .call_dry_run(&ink_e2e::alice(), &stakeholder_data_msg, 0, None)
                    .await.return_value().0.paid;

                let stakeholder_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                    .call(|contract| contract.balance_of(stakeholder_account.clone()));
                let stakeholder_balance = client
                    .call_dry_run(&ink_e2e::alice(), &stakeholder_balance_msg.clone(), 0, None)
                    .await.return_value();

                let pool_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                    .call(|contract| contract.pool_balance(TEAM_FOUNDERS));
                let pool_balance = client
                    .call_dry_run(&ink_e2e::alice(), &pool_balance_msg.clone(), 0, None)
                    .await.return_value().1;

                let owner_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                    .call(|contract| contract.balance_of(alice_account.clone()));
                let owner_balance = client
                    .call_dry_run(&ink_e2e::alice(), &owner_balance_msg.clone(), 0, None)
                    .await.return_value();

                let increment_month_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                    .call(|contract| contract.TESTING_increment_month());
                let _increment_month_result = client
                    .call(&ink_e2e::alice(), increment_month_msg, 0, None).await;

                /* // visual proof of workee
                println!("{:?}", month_result);
                println!("{:?}", stakeholder_paid);
                println!("{:?}", stakeholder_balance);
                println!("{:?}", pool_balance);
                println!("{:?}", owner_balance);
                */
                if month < cliff {

                    assert_eq!(stakeholder_paid, 0);
                    assert_eq!(stakeholder_balance, 0);
                    assert_eq!(owner_balance, SUPPLY_CAP);
                    assert_eq!(pool_balance, pool_size);

                } else if month >= cliff && month < schedule_end {

                    assert_eq!(stakeholder_paid, (month - cliff + 1) as Balance * payout);
                    assert_eq!(stakeholder_balance, (month - cliff + 1) as Balance * payout);
                    assert_eq!(owner_balance, SUPPLY_CAP - (month - cliff + 1) as Balance * payout);
                    assert_eq!(pool_balance, pool_size - (month - cliff + 1) as Balance * payout);

                } else if month >= schedule_end {

                    assert_eq!(stakeholder_paid, (schedule_period - 1) as Balance * payout + last_payout);
                    assert_eq!(stakeholder_balance, (schedule_period - 1) as Balance * payout + last_payout);
                    assert_eq!(owner_balance,
                               SUPPLY_CAP - (schedule_period - 1) as Balance * payout - last_payout);
                    assert_eq!(pool_balance,
                               pool_size - (schedule_period - 1) as Balance * payout - last_payout);
                }
            }
            Ok(())
        }

        /// SAD DISTRIBUTE_TOKENS
        /// - Check to make sure distribute_tokens fails as expected.
        ///
        /// - Return
        ///     CallerNotOwner               - When caller does not own contract
        ///     StakeholderNotFound          - when stakeholder specified isn't registered
        ///     CliffNotPassed               - when pool's vesting cliff isn't passed
        ///     StakeholderSharePaid         - when stakeholder has already been paid entire share
        ///     PayoutTooEarly               - when next month's payment isn't ready
        #[ink_e2e::test]
        async fn sade2e_distribute_tokens(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            Ok(())
        }

        /// HAPPY PAYOUT_TOKENS
        /// - Check to make sure payout_tokens works as expected.
        /// - Checks PARTNERS, WHITELIST, and PUBLIC_SALE pools.
        /// - Checks resulting balances for three pools and recipients.
        #[ink_e2e::test]
        async fn happye2e_payout_tokens(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

            let constructor = ILOCKmvpRef::new_token();
            let contract_acct_id = client
                .instantiate("ilockmvp", &ink_e2e::alice(), constructor, 0, None)
                .await.expect("instantiate failed").account_id;

            // messages the pay from various pools
            let partners_pay_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.payout_tokens(
                    bob_account.clone(), 1000, "PARTNERS".to_string()));
            let whitelist_pay_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.payout_tokens(
                    bob_account.clone(), 1000, "WHITELIST".to_string()));
            let publicsale_pay_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.payout_tokens(
                    bob_account.clone(), 1000, "PUBLIC_SALE".to_string()));

            // alice pays 1000 ILOCK to bob from PARTNERS pool
            let _partners_pay_result = client
                .call(&ink_e2e::alice(), partners_pay_msg, 0, None).await;

            // checks that alice has expected resulting balance
            let alice_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.balance_of(alice_account.clone()));
            let mut alice_balance = client
                .call_dry_run(&ink_e2e::alice(), &alice_balance_msg, 0, None).await.return_value();
            assert_eq!(SUPPLY_CAP - 1000, alice_balance);

            // checks that bob has expected resulting balance
            let bob_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.balance_of(bob_account.clone()));
            let mut bob_balance = client
                .call_dry_run(&ink_e2e::alice(), &bob_balance_msg, 0, None).await.return_value();
            assert_eq!(0 + 1000, bob_balance);

            // checks that pool has expected resulting balance
            let mut pool_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.pool_balance(PARTNERS));
            let mut pool_balance = client
                .call_dry_run(&ink_e2e::alice(), &pool_balance_msg, 0, None).await.return_value().1;
            assert_eq!(POOLS[PARTNERS as usize].tokens * DECIMALS_POWER10 - 1000, pool_balance);

            // alice pays 1000 ILOCK to bob from WHITELIST pool
            let _whitelist_pay_result = client
                .call(&ink_e2e::alice(), whitelist_pay_msg, 0, None).await;

            // checks that alice has expected resulting balance
            alice_balance = client
                .call_dry_run(&ink_e2e::alice(), &alice_balance_msg, 0, None).await.return_value();
            assert_eq!(SUPPLY_CAP - 1000 - 1000, alice_balance);

            // checks that bob has expected resulting balance
            bob_balance = client
                .call_dry_run(&ink_e2e::alice(), &bob_balance_msg, 0, None).await.return_value();
            assert_eq!(0 + 1000 + 1000, bob_balance);

            // checks that pool has expected resulting balance
            pool_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.pool_balance(WHITELIST));
            pool_balance = client
                .call_dry_run(&ink_e2e::alice(), &pool_balance_msg, 0, None).await.return_value().1;
            assert_eq!(POOLS[WHITELIST as usize].tokens * DECIMALS_POWER10 - 1000, pool_balance);

            // alice pays 1000 ILOCK to bob from PUBLIC_SALE pool
            let _publicsale_pay_result = client
                .call(&ink_e2e::alice(), publicsale_pay_msg, 0, None).await;

            // checks that alice has expected resulting balance
            alice_balance = client
                .call_dry_run(&ink_e2e::alice(), &alice_balance_msg, 0, None).await.return_value();
            assert_eq!(SUPPLY_CAP - 1000 - 1000 - 1000, alice_balance);

            // checks that bob has expected resulting balance
            bob_balance = client
                .call_dry_run(&ink_e2e::alice(), &bob_balance_msg, 0, None).await.return_value();
            assert_eq!(0 + 1000 + 1000 + 1000, bob_balance);

            // checks that pool has expected resulting balance
            pool_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.pool_balance(PUBLIC_SALE));
            pool_balance = client
                .call_dry_run(&ink_e2e::alice(), &pool_balance_msg, 0, None).await.return_value().1;
            assert_eq!(POOLS[PUBLIC_SALE as usize].tokens * DECIMALS_POWER10 - 1000, pool_balance);

            Ok(())
        }

        /// SAD PAYOUT_TOKENS
        /// - Checks to make sure payout_tokens function fails as expected.
        ///
        /// - Return
        ///     CallerNotOwner               - when caller does not own contract
        ///     InvalidPool                  - when pool isn't (PARTNERS|WHITELIST|PUBLIC_SALE)
        ///     PaymentTooLarge              - when specified payment amount is more than pool
        #[ink_e2e::test]
        async fn sade2e_payout_tokens(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            Ok(())
        }

        /// HAPPY REWARD_INTERLOCKER
        /// - Test if rewarding functionality works.
        /// - Update rewardedtotal.
        /// - Transfer reward amount from rewards pool to Interlocker.
        /// - Update individual rewardedinterlockertotal
        /// - Emit reward event.
        /// - Return new interlocker rewarded total.
        /// - Test that rewarded_total() works.
        /// - Test that rewarded_interlocker_total() works.
        #[ink_e2e::test]
        async fn happye2e_reward_interlocker(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

            let constructor = ILOCKmvpRef::new_token();
            let contract_acct_id = client
                .instantiate("ilockmvp", &ink_e2e::alice(), constructor, 0, None)
                .await.expect("instantiate failed").account_id;

            // alice rewards bob the happy interlocker 1000 ILOCK
            let alice_reward_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.reward_interlocker(1000, bob_account.clone()));
            let reward_response = client
                .call(&ink_e2e::alice(), alice_reward_msg, 0, None).await.unwrap();

            // filter for reward event
            let contract_emitted_reward = reward_response
                .events
                .iter()
                .find(|event| {
                    event
                        .as_ref()
                        .expect("expected event")
                        .event_metadata()
                        .event()
                        == "ContractEmitted" &&
                        String::from_utf8_lossy(
                            event.as_ref().expect("bad event").bytes()).to_string()
                       .contains("ILOCKmvp::Reward")
                })
                .expect("Expect ContractEmitted event")
                .unwrap();

            // decode to the expected event type (skip field_context)
            let reward_event = contract_emitted_reward.field_bytes();
            let decoded_reward =
                <Reward as scale::Decode>::decode(&mut &reward_event[34..]).expect("invalid data");

            // destructor decoded transfer
            let Reward { to, amount } = decoded_reward;

            // assert with the expected value
            assert_eq!(to, Some(bob_account), "encountered invalid Reward.to");
            assert_eq!(amount, 1000, "encountered invalid Reward.amount");

            // checks that alice has expected resulting balance
            let alice_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.balance_of(alice_account.clone()));
            let alice_balance = client
                .call_dry_run(&ink_e2e::alice(), &alice_balance_msg, 0, None).await.return_value();
            assert_eq!(SUPPLY_CAP - 1000, alice_balance);

            // checks that pool has expected resulting balance
            let pool_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.pool_balance(REWARDS));
            let pool_balance = client
                .call_dry_run(&ink_e2e::alice(), &pool_balance_msg, 0, None).await.return_value().1;
            assert_eq!(POOLS[REWARDS as usize].tokens * DECIMALS_POWER10 - 1000, pool_balance);

            // checks that bob has expected resulting balance
            let bob_balance_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.balance_of(bob_account.clone()));
            let bob_balance = client
                .call_dry_run(&ink_e2e::alice(), &bob_balance_msg, 0, None).await.return_value();
            assert_eq!(0 + 1000, bob_balance);

            // checks that circulating supply was properly incremented
            let total_supply_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.total_supply());
            let total_supply = client
                .call_dry_run(&ink_e2e::alice(), &total_supply_msg, 0, None).await.return_value();
            assert_eq!(0 + 1000, total_supply);

            // checks that total rewarded (overall) is correct
            let total_rewarded_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.rewarded_total());
            let total_rewarded = client
                .call_dry_run(&ink_e2e::alice(), &total_rewarded_msg, 0, None).await.return_value();
            assert_eq!(0 + 1000, total_rewarded);

            // checks that total rewarded (to interlocker) is correct
            let total_rewarded_interlocker_msg = build_message::<ILOCKmvpRef>(contract_acct_id.clone())
                .call(|contract| contract.rewarded_interlocker_total(bob_account.clone()));
            let total_rewarded_interlocker = client
                .call_dry_run(&ink_e2e::alice(), &total_rewarded_interlocker_msg, 0, None).await.return_value();
            assert_eq!(0 + 1000, total_rewarded_interlocker);

            Ok(())
        }

        /// SAD REWARD_INTERLOCKER
        /// - Test if rewarding functionality fails correctly.
        ///
        /// - Return
        ///     CallerNotOwner               - when caller does not own contract
        ///     PaymentTooLarge              - when arithmetic over or underflows
        ///
        ///     ... maybe check the over/underflows?
        #[ink_e2e::test]
        async fn sade2e_reward_interlocker(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {

            Ok(())
        }
    

