<img align="right" width="300" height="300" src="https://assets-global.website-files.com/64d9930f57641d176ab09b78/64df168bc9c81b986abd0e9b_img-ilock-token.png">

# $ILOCK PSP22 Rewards Smart Contract

The `ilockmvp` is the main Interlock Network smart contract to be hosted on Aleph Zero. For now this consists of one ERC20-style token contract (the Polkadot PSP22 spec) with functionality to manage interlocker rewards for surfing the web with browser extension. Future contracts will be responsible for Phase 2, where we implement the security staking model for interlockers to actively earn rewards by staking tokens on questionable websites.

The `ilockmvp` contract is to be the centerpoint that the entire Interlock Network and ecosystem revolves around. The contract is meant to be simple, with upgradability and extendability in mind.

The core of the contract is a standard Openbrush PSP22 token implementation. Built around the PSP22 token are provisions to enforce a vesting schedule, plus provisions to maintain higher resolution in accounting and bookkeeping than the typical PSP22 alone.

As for the minimum viable product, the purpose of $ILOCK utility token is to incentivize Interlocker browser extension operators to share their browsing data and feedback by issuing rewards. In addition to the basic token and vesting/accounting measures, there is a simple suite of rewards-related function built in. The minimum viable function is simply one that issues a variable reward to a specified interlocker, drawn from the rewards pool.

#### This contract is unique in for two reasons:

1) This contract has no mint function. The fixed cap of 1B tokens is dumped into the contract owner's account (Interlock Foundation) on TGE. For this reason, `total_supply()` is reimplemented to reflect total supply of tokens in _circulation_. Tokens are released into circulation from pools according to vesting schedule and rewards (rewards maximum being limited to the size of the rewards pool). All pool sizes and schedules are preallocated and hard-coded into the contract. One pool for proceeds starts with an empty balance and is used to collect fees and taxes from application contracts interacting with `ilockmvp`. Pool balances do not have entries in the PSP22 balance mapping. The only balance mapping entry is the contract owner's balance, which reflects to total of all non-circulating tokens. The purpose of this strategy is to avoid the `mint()` function, which is not part of the PSP22 spec, but wildly popular in token contracts (and on occasion, vulnerable to exploit).

2) This contract implements Interlock Network's novel [application port/socket formalism](../contract_application/). In short, this formalism provides provisions for treating this PSP22 smart contract like a _computer_. What this accomplishes is to enable versatile connectivity between arbitrary application contracts and this central PSP22 token contract, while simultaneously reducing the need for Interlock Network (or any forking network) to serve as an off-chain transaction relay (a glaring vulnerability for many dApp implementations).

#### This contract underpins and is informed by the [Interlock Token Model](https://github.com/interlock-network/interlock-models), in its current form:

![Interlock Network Token Model](https://raw.githubusercontent.com/interlock-network/interlock-models/main/graphs/mad-4-full-graph.png)

## How to get setup and build:

#### See [[DOCUMENTATION]](https://interlock-network.github.io/interlock-smartcontracts/contract_ilockmvp/docs/ilockmvp/).

## How to test on testnet: 

##### To deploy a contract to testnet, `upload metadata.json` and `ilockmvp.wasm` (in `target`) to [testnet.alephzero.org](https://testnet.alephzero.org).

## How to build and run tests

##### To view debug prints and assertion failures run test via:
```
cargo +nightly test --features e2e-tests -- --show-output
```
##### To view debug for specific method run test via:
```
cargo +nightly test <test_function_here> -- --nocapture
```
##### To run end-to-end tests, first make sure you have the substrate dev node capabilities installed via:
```
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git
```
##### Then run the node:
```
substrate-contracts-node --log info,runtime::contracts=debug 2>&1
```
##### Finally, you can run the end-to-end and unit test suite:
```
cargo +nightly test --features e2e-tests -- --show-output
```
