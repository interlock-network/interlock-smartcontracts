# ilockmvp Smart Contract

`ilockmvp` is the main Interlock Network smart contract to be hosted on Aleph Zero. For now this consists of one ERC20-style token (PSP22) contract with functionality to manage interlocker rewards for surfing the web with browser extension. Future contracts will be responsible for Phase 2, where we implement the security staking model for interlockers to actively earn rewards by staking tokens on questionable websites.

## How to get setup

[If any issues should arise, refer the the primary source at this link for troubleshooting tips.](https://ink.substrate.io/getting-started/setup)

### Install Rust and Cargo

[Do this according to this site.](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Install cargo-dylint
Run
```
cargo install cargo-dylint dylint-link
```
### Install cargo-contract
Now run
```
cargo install cargo-contract --force --locked --version=2.2.0-beta
```
### Finally
Run
```
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

### How to build

In this directory, run:
```
cargo +nightly contract build
```
to build `ilockmvp`.

To deploy a contract to testnet, `upload metadata.json` and `ilockmvp.wasm` (in `target`) to [testnet.alephzero.org](https://testnet.alephzero.org).

### To build and run tests

In this directory, first set up a blockchain dev node. Install if you haven't already:
```
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git
```
Then you can first up a node in a separate terminal instance:
```
substrate-contracts-node
```
Finally, you can run the end-to-end and unit test suite:
```
cargo +nightly test --features e2e-tests -- --show-output
```
