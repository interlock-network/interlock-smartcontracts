//!
//! INTERLOCK NETWORK UANFT SMART CONTRACT END-TO-END TESTS
//!
//! End to end tests are used extensively because using the Openbrush
//! PSP34 framework involves cross-contract invocations under the hood.
//! EG/IE, If I want to mint an NFT to interlocker, this involves an internal
//! call of the OpenBrush PSP34 transfer message. I know of no way to
//! get around this fact for testing besides using end-to-end tests.
//!
//! ##### to setup for e2e testin, run
//!
//! ubstrate-contracts-node --log info,runtime::contracts=debug 2>&1
//! 
//! ##### after installing by running
//!
//! cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git
//!
//! ##### To view debug prints and assertion failures run test via:
//!  
//! cargo +nightly test --features e2e-tests -- --show-output
//!
//! ##### To view debug for specific method run test via:
//!  
//! cargo nightly+ test <test_function_here> -- --nocapture
//!
//! ! NB ! During test build and runtime, if you ever come across errors
//!        saying 'Metadata artifacts not generated' or 'Once instance
//!        has previously been poisoned', then you need to run `cargo clean`
//!        or delete the `target` directory the build/run from scratch.
//!        OR
//!        Save both the lib.rs file AND this tests_e2e.rs file, then reattempt.
//!

use crate::uanft::*;

#[cfg(all(test, feature = "e2e-tests"))]
use ink_e2e::build_message;

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use openbrush::contracts::{
    psp34::{
        psp34_external::PSP34,
        PSP34Error,
        Id,
    },
    psp22::psp22_external::PSP22,
};
use ink::primitives::Hash;

/// - Test that anybody can mint UANFT for themselves using ILOCK.
/// - Test that uanft application contract can connect to PSP22 ILOCK contract via socket.
#[ink_e2e::test(additional_contracts = "../contract_ilockmvp/Cargo.toml")]
async fn happy_self_mint(
    mut client: ink_e2e::Client<C, E>,
) -> E2EResult<()> {

    let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
    let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
    let charlie_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie);

    // spin up ILOCK PSP22 token contract
    let ilock_constructor = ilockmvp::ILOCKmvpRef::new_token(
        200_000,
        charlie_account.clone(),
        bob_account.clone(),
        );

    let ilock_contract_acct_id = client
        .instantiate("ilockmvp", &ink_e2e::alice(), ilock_constructor, 0, None)
        .await.expect("instantiate failed").account_id;

    // spin up uanft PSP34 contract
    let uanft_constructor = Psp34NftRef::new(
        "Interlock Network Universal Access NFT".to_string(),
        "ILOCK-UANFT".to_string(),
        "GENERAL-ACCESS".to_string(),
        10_000,
        0,
        ilock_contract_acct_id,
        200_000,
        charlie_account.clone(),
        bob_account.clone(),
    );
    let uanft_contract_acct_id = client
        .instantiate("uanft", &ink_e2e::alice(), uanft_constructor, 0, None)
        .await.expect("instantiate failed").account_id;

    // set the token price in ILOCK
    let set_price_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.set_token_price(100));
    let _create_port_result = client
        .call(&ink_e2e::alice(), set_price_msg, 0, None).await;

    // we are assuming this testing contract is safe, so get its own hash with testing helper
    // message
    let get_hash_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.contract_hash(uanft_contract_acct_id.clone()));
    let application_hash = client
        .call_dry_run(&ink_e2e::alice(), &get_hash_msg, 0, None).await.return_value().unwrap();

    // create a dummy port for PORT 0 on ILOCK token contract
    let create_port_msg = build_message::<ilockmvp::ILOCKmvpRef>(ilock_contract_acct_id.clone())
        .call(|contract| contract.create_port(application_hash, 0, 0, false, 0, alice_account.clone(), false, "CREATE_PORT".to_string()));
    let _create_port_result = client
        .call(&ink_e2e::alice(), create_port_msg, 0, None).await;

    // charge bob's ILOCK account by rewarding him ILOCK
    let reward_bob_msg = build_message::<ilockmvp::ILOCKmvpRef>(ilock_contract_acct_id.clone())
        .call(|contract| contract.reward_interlocker(100_000, bob_account.clone()));
    let _reward_result = client
        .call(&ink_e2e::alice(), reward_bob_msg, 0, None).await;

    // connect this uanft application contract to ILOCK token contract via socket
    let create_socket_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.create_socket());
    let _create_socket_result = client
        .call(&ink_e2e::alice(), create_socket_msg, 0, None).await;

    // check that uanft token price getter works
    let get_token_price_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.get_token_price());
    let token_price = client
        .call_dry_run(&ink_e2e::alice(), &get_token_price_msg, 0, None).await.return_value();
    assert_eq!(token_price, 100);

    // now finally, bob attempts a self-mint uanft in exchange for 100 ILOCK
    let self_mint_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.self_mint(token_price));
    let _mint_result = client
        .call(&ink_e2e::bob(), self_mint_msg, 0, None).await;

    // verify that bob's uanft collection reflects this
    let get_bob_collection_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.get_collection(bob_account.clone()));
    let bob_collection = client
        .call_dry_run(&ink_e2e::alice(), &get_bob_collection_msg, 0, None).await.return_value().unwrap();
    assert_eq!(bob_collection, [Id::U64(1)]);

    // verify that bob successfully paid 100 ILOCK
    let bob_balance_of_msg = build_message::<ilockmvp::ILOCKmvpRef>(ilock_contract_acct_id.clone())
        .call(|contract| contract.balance_of(bob_account.clone()));
    let bob_balance = client
        .call_dry_run(&ink_e2e::alice(), &bob_balance_of_msg, 0, None).await.return_value();
    assert_eq!(bob_balance, 100_000 - 100);

    // verify that circulating ILOCK supply decreased appropriately
    let supply_msg = build_message::<ilockmvp::ILOCKmvpRef>(ilock_contract_acct_id.clone())
        .call(|contract| contract.total_supply());
    let supply = client
        .call_dry_run(&ink_e2e::alice(), &supply_msg, 0, None).await.return_value();
    assert_eq!(supply, 100_000 - 100);

    Ok(())
}
