///
/// INTERLOCK NETWORK MVP SMART CONTRACT UNIT TESTS
///  - PSP22 TOKEN
///  - REWARDS
///
///
/// #### To view debug prints and assertion failures run test via:
///
/// cargo +nightly test -- --show-output
/// 
/// #### To view debug for specific method run test via:
/// 
/// cargo nightly+ test <test_function_here> -- --nocapture
///

use crate::ilockmvp::*;
use openbrush::{
    contracts::psp22::PSP22,
    traits::AccountId,
};
use ink::{
    codegen::Env
};

/// - Test if the default constructor does its job
/// - and check months_passed()
/// - and check cap().
#[ink::test]
fn new_token_works() {

    let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
    let ILOCKmvpPSP22 = ILOCKmvp::new_token(
        200_000,
        accounts.bob,
        accounts.charlie,
        ).unwrap();

    println!("{:?}", AccountId::from([0_u8; 32]));

    assert_eq!(ILOCKmvpPSP22.vest.monthspassed, ILOCKmvpPSP22.months_passed());
    assert_eq!(ILOCKmvpPSP22.vest.nextpayout, ILOCKmvpPSP22.env().block_timestamp() + ONE_MONTH);
    assert_eq!(ILOCKmvpPSP22.total_supply(), 0);
    assert_eq!(ILOCKmvpPSP22.metadata.name, Some("Interlock Network".as_bytes().to_vec()));
    assert_eq!(ILOCKmvpPSP22.metadata.symbol, Some("ILOCK".as_bytes().to_vec()));
    assert_eq!(ILOCKmvpPSP22.metadata.decimals, 18);

    // this checks that token numbers have been entered accurately into POOLS PoolData
    let mut total_tokens: u128 = 0;
    for pool in 0..POOL_COUNT {

        total_tokens += POOLS[pool].tokens * DECIMALS_POWER10;
    }
    assert_eq!(total_tokens, ILOCKmvpPSP22.cap());
    assert_eq!(ILOCKmvpPSP22.ownable.owner, ILOCKmvpPSP22.env().caller());
}

/// HAPPY REGISTER_STAKEHOLDER & STAKEHOLDER_DATA
/// - Test if register_stakeholder and stakeholder_data functions works correctly.
/// - Registration should succeed as long as stakeholder share > 0.
/// - Payremaining should accurately reflect distribution to stakeholder given share.
#[ink::test]
fn happy_register_stakeholder_data() {

}

/* THIS NEED TO BE REWRITTEN

/// - Test if create_port() and port() functions correctly.
/// - Test if tax_port_transfer() functions correctly.
#[ink::test]
fn happy_create_get_port_tax_transfer() {

    let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
    let mut ILOCKmvpPSP22 = ILOCKmvp::new_token(
        200_000,
        accounts.bob,
        accounts.charlie,
        );

    let codehash: Hash = Default::default(); // offchain environment doesn't support
    let tax: Balance = 1_000; // 0.1% tax  // .own_code_hash()
    let cap: Balance = 1_000_000;
    let locked: bool = true;
    let number: u16 = 2;
    let owner: AccountId = accounts.bob;
    let overwrite = false;
    let function = "CREATE_PORT";

    let _ = ILOCKmvpPSP22.create_port(
        codehash,
        tax,
        cap,
        locked,
        number,
        owner,
        overwrite,
        function.to_string(),
    );

    let

    let mut port: Port = ILOCKmvpPSP22.port(number);
println!("{:}", port.cap);
    assert_eq!(port, Port {
        application: codehash,
        tax: tax,
        cap: cap,
        locked: locked,
        paid: 0,
        collected: 0,
        owner: owner,
    });

    ILOCKmvpPSP22.balances[CIRCULATING as usize] += 1_000_000;

    let test_socket: Socket = Socket {

        operator: accounts.eve,
        portnumber: 2,
    };

    let _ = ILOCKmvpPSP22.tax_port_transfer(
        
        test_socket,
        port,
        cap,
    );

    port = ILOCKmvpPSP22.app.ports.get(number).unwrap();

    assert_eq!(port.paid, 1_000_000 - 1_000); // 999_000
    assert_eq!(port.collected, 0 + 1_000);
    assert_eq!(ILOCKmvpPSP22.balances[PROCEEDS as usize], 0 + 1_000);
    assert_eq!(ILOCKmvpPSP22.total_supply(), 1_000_000 - 1_000);
}
*/
/// SAD TAX_PORT_TRANSFER
/// - Not sure there is much to do here.
#[test]
fn sad_tax_port_transfer() {

}

/*************************  THIS TEST IS SLOW, THUS COMMENTED OUT UNLESS NEEDED

/// - Test to make sure month increment doesn't happen too soon.
#[ink::test]
fn happy_check_time() {

    let mut ILOCKmvpPSP22 = ILOCKmvp::new_token();

    for _time in 0..432_000_001 { // number of advances needed to span month

        ink::env::test::advance_block::<ink::env::DefaultEnvironment>();
    }
    let timestamp: Timestamp = ink::env::block_timestamp::<ink::env::DefaultEnvironment>();

    assert!(ILOCKmvpPSP22.vest.nextpayout < timestamp);
    assert_eq!(ILOCKmvpPSP22.vest.monthspassed, 0);
    let _ = ILOCKmvpPSP22.check_time();
    assert_eq!(ILOCKmvpPSP22.vest.monthspassed, 1);
}

**************************/


