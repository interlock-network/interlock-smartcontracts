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

// byte array representing SHA256('test_username')
const TEST_USERNAME_ARRAY: [u8; 32] = [ 204, 221, 179,  10, 141,  56,  15, 156,
                                          2, 209, 187,  54, 104,  62,  98, 214,
                                        103, 214,  46,  36,  77,  66, 122, 252,
                                         68,  10, 183, 131, 110, 216,  20, 240 ];

// byte array representing SHA256('test_password')
const TEST_PASSWORD_ARRAY: [u8; 32] = [  16, 166, 230, 204, 131,  17, 163, 226,
                                        188, 192, 155, 246, 193, 153, 173, 236,
                                        213, 221,  89,  64, 140,  52,  62, 146,
                                        107,  18, 156,  73,  20, 243, 203,   1 ];

/// - Test if customized transfer function works correctly.
/// - When transfer, credentials are revoked.
/// - Test that register function works correctly.
/// - Test that transfer events are properly emitted.
/// - Test that get_credential() and get_collection() works..
#[ink_e2e::test(additional_contracts = "../contract_ilockmvp/Cargo.toml")]
async fn happy_mint_register_transfer(
    mut client: ink_e2e::Client<C, E>,
) -> E2EResult<()> {

    let test_username_hash: Hash = Hash::from(TEST_USERNAME_ARRAY);
    let test_password_hash: Hash = Hash::from(TEST_PASSWORD_ARRAY);

    let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
    let charlie_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie);

    let ilock_constructor = ilockmvp::ILOCKmvpRef::new_token();
    let ilock_contract_acct_id = client
        .instantiate("ilockmvp", &ink_e2e::alice(), ilock_constructor, 0, None)
        .await.expect("instantiate failed").account_id;

    let constructor = Psp34NftRef::new(
        "Interlock Network Universal Access NFT".to_string(),
        "ILOCK-UANFT".to_string(),
        "GENERAL-ACCESS".to_string(),
        10_000,
        100,
        ilock_contract_acct_id,
    );
    let uanft_contract_acct_id = client
        .instantiate("uanft", &ink_e2e::alice(), constructor, 0, None)
        .await.expect("instantiate failed").account_id;

    let mint_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.mint_to(bob_account.clone()));
    let mint_response = client
        .call(&ink_e2e::alice(), mint_msg, 0, None).await.unwrap();

    // filter for transfer mint event
    let contract_emitted_transfer = mint_response
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
           .contains("Psp34Nft::Transfer")
        })
        .expect("Expect ContractEmitted event")
        .unwrap();

    // decode to the expected event type (skip field_context)
    let transfer_event = contract_emitted_transfer.field_bytes();
    let decoded_transfer =
        <Transfer as scale::Decode>::decode(&mut &transfer_event[34..]).expect("invalid data");

    // destructor decoded eapproval
    let Transfer { from, to, id } = decoded_transfer;

    // assert with the expected value
    assert_eq!(from, None, "encountered invalid Transfer.to");
    assert_eq!(to, Some(bob_account), "encountered invalid Transfer.from");
    assert_eq!(id, Id::U64(1), "encountered invalid Transfer.id");

    let owner_of_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.owner_of(Id::U64(1)));
    let owner = client
        .call_dry_run(&ink_e2e::alice(), &owner_of_msg, 0, None).await.return_value().unwrap();
    assert_eq!(owner, bob_account.clone());

    let get_bob_collection_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.get_collection(bob_account.clone()));
    let bob_collection = client
        .call_dry_run(&ink_e2e::alice(), &get_bob_collection_msg, 0, None).await.return_value().unwrap();
    assert_eq!(bob_collection, [Id::U64(1)]);

    let mint_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.mint_to(bob_account.clone()));
    let _mint_result = client
        .call(&ink_e2e::alice(), mint_msg, 0, None).await;

    let bob_collection = client
        .call_dry_run(&ink_e2e::alice(), &get_bob_collection_msg, 0, None).await.return_value().unwrap();
    assert_eq!(bob_collection, [Id::U64(1), Id::U64(2)]);

    let register_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.register(Id::U64(2), test_username_hash, test_password_hash));
    let _register_result = client
        .call(&ink_e2e::bob(), register_msg, 0, None).await;

    let bob_get_credential_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.get_credential(test_username_hash));
    let bob_credential = client
        .call_dry_run(&ink_e2e::bob(), &bob_get_credential_msg, 0, None).await.return_value().unwrap();
    assert_eq!(bob_credential.0, test_password_hash);
    assert_eq!(bob_credential.1, Id::U64(2));

    let transfer_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.transfer(
            charlie_account.clone(), Id::U64(2), Default::default()));
    let transfer_result = client
        .call(&ink_e2e::bob(), transfer_msg, 0, None).await.unwrap();

    // filter for transfer event
    let contract_emitted_transfer = transfer_result
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
               .contains("Psp34Nft::Transfer")
        })
        .expect("Expect ContractEmitted event")
        .unwrap();

    // decode to the expected event type (skip field_context)
    let transfer_event = contract_emitted_transfer.field_bytes();
    let decoded_transfer =
        <Transfer as scale::Decode>::decode(&mut &transfer_event[35..]).expect("invalid data");

    // destructor decoded eapproval
    let Transfer { from, to, id } = decoded_transfer;

    // assert with the expected value
    assert_eq!(from, Some(bob_account), "encountered invalid Transfer.to");
    assert_eq!(to, Some(charlie_account), "encountered invalid Transfer.from");
    assert_eq!(id, Id::U64(2), "encountered invalid Transfer.id");

    let bob_collection = client
        .call_dry_run(&ink_e2e::alice(), &get_bob_collection_msg, 0, None).await.return_value().unwrap();
    assert_eq!(bob_collection, [Id::U64(1)]);

    let get_charlie_collection_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.get_collection(charlie_account.clone()));
    let charlie_collection = client
        .call_dry_run(&ink_e2e::alice(), &get_charlie_collection_msg, 0, None)
        .await.return_value().unwrap();
    assert_eq!(charlie_collection, [Id::U64(2)]);

    let owner_of_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.owner_of(Id::U64(2)));
    let owner = client
        .call_dry_run(&ink_e2e::alice(), &owner_of_msg, 0, None).await.return_value().unwrap();
    assert_eq!(owner, charlie_account.clone());

    let bob_get_credential_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.get_credential(test_username_hash));
    let bob_credential = client
        .call_dry_run(&ink_e2e::bob(), &bob_get_credential_msg, 0, None).await.return_value();
    assert_eq!(bob_credential,
        // Error: collection does not exist
        Err(PSP34Error::Custom([67, 114, 101, 100, 101, 110, 116,
                               105, 97, 108, 115, 32, 110, 111, 110,
                               101, 120, 105, 115, 116, 101, 110, 116, 46].to_vec())));

    let set_credential_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.set_credential(Id::U64(1), test_username_hash, test_password_hash));
    let _set_credential_result = client
        .call(&ink_e2e::alice(), set_credential_msg, 0, None).await;

    let is_authenticated_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.is_authenticated(Id::U64(1)));
    let status = client
        .call_dry_run(&ink_e2e::alice(), &is_authenticated_msg, 0, None).await.return_value().unwrap();
    assert_eq!(status, true);

    let revoke_access_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.revoke_access(test_username_hash));
    let _revoke_access_result = client
        .call(&ink_e2e::alice(), revoke_access_msg, 0, None).await;

    let is_authenticated_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.is_authenticated(Id::U64(1)));
    let status = client
        .call_dry_run(&ink_e2e::alice(), &is_authenticated_msg, 0, None).await.return_value().unwrap();
    assert_eq!(status, false);

    Ok(())
}

/// - Test that anybody can mint UANFT for themselves using ILOCK.
#[ink_e2e::test(additional_contracts = "../contract_ilockmvp/Cargo.toml")]
async fn happy_self_mint(
    mut client: ink_e2e::Client<C, E>,
) -> E2EResult<()> {

    let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
    let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

    let ilock_constructor = ilockmvp::ILOCKmvpRef::new_token();
    let ilock_contract_acct_id = client
        .instantiate("ilockmvp", &ink_e2e::alice(), ilock_constructor, 0, None)
        .await.expect("instantiate failed").account_id;

    let uanft_constructor = Psp34NftRef::new(
        "Interlock Network Universal Access NFT".to_string(),
        "ILOCK-UANFT".to_string(),
        "GENERAL-ACCESS".to_string(),
        10_000,
        0,
        ilock_contract_acct_id,
    );
    let uanft_contract_acct_id = client
        .instantiate("uanft", &ink_e2e::alice(), uanft_constructor, 0, None)
        .await.expect("instantiate failed").account_id;

    let set_price_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.set_token_price(100));
    let _create_port_result = client
        .call(&ink_e2e::alice(), set_price_msg, 0, None).await;

    // we are assuming this testing contract is safe
    let get_hash_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.contract_hash(uanft_contract_acct_id.clone()));
    let application_hash = client
        .call_dry_run(&ink_e2e::alice(), &get_hash_msg, 0, None).await.return_value();

    let create_port_msg = build_message::<ilockmvp::ILOCKmvpRef>(ilock_contract_acct_id.clone())
        .call(|contract| contract.create_port(application_hash, 0, 0, false, 0, alice_account.clone() ));
    let _create_port_result = client
        .call(&ink_e2e::alice(), create_port_msg, 0, None).await;

    let reward_bob_msg = build_message::<ilockmvp::ILOCKmvpRef>(ilock_contract_acct_id.clone())
        .call(|contract| contract.reward_interlocker(100_000, bob_account.clone()));
    let _reward_result = client
        .call(&ink_e2e::alice(), reward_bob_msg, 0, None).await;

    let create_socket_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.create_socket());
    let _create_socket_result = client
        .call(&ink_e2e::alice(), create_socket_msg, 0, None).await;

    let get_token_price_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.get_token_price());
    let token_price = client
        .call_dry_run(&ink_e2e::alice(), &get_token_price_msg, 0, None).await.return_value();
    assert_eq!(token_price, 100);

    let self_mint_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.self_mint(token_price));
    let _mint_result = client
        .call(&ink_e2e::bob(), self_mint_msg, 0, None).await;

    let get_bob_collection_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
        .call(|contract| contract.get_collection(bob_account.clone()));
    let bob_collection = client
        .call_dry_run(&ink_e2e::alice(), &get_bob_collection_msg, 0, None).await.return_value().unwrap();
    assert_eq!(bob_collection, [Id::U64(1)]);

    let bob_balance_of_msg = build_message::<ilockmvp::ILOCKmvpRef>(ilock_contract_acct_id.clone())
        .call(|contract| contract.balance_of(bob_account.clone()));
    let bob_balance = client
        .call_dry_run(&ink_e2e::alice(), &bob_balance_of_msg, 0, None).await.return_value();
    assert_eq!(bob_balance, 100_000 - 100);

    let supply_msg = build_message::<ilockmvp::ILOCKmvpRef>(ilock_contract_acct_id.clone())
        .call(|contract| contract.total_supply());
    let supply = client
        .call_dry_run(&ink_e2e::alice(), &supply_msg, 0, None).await.return_value();
    assert_eq!(supply, 100_000 - 100);

    Ok(())
}
