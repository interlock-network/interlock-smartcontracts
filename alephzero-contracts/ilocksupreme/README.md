# ILOCKsupreme Smart Contracts

ILOCKsupreme is the main collection of Interlock Network smart contracts on Aleph Zero. For now this consists of one ERC20-style token contract, and one contract to manage user rewards for surfing the web with browser extension. Future contracts will be responsible for Phase 2, where we implement the security staking model for users to actively earn rewards by staking tokens on questionable websites.

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
cargo install cargo-contract --force --locked
```
### Finally
Run
```
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

## How to build

First navigate to ilocktoken directory.

Run
```
cargo +nightly contract build
```
to build token portion of ilocksupreme.

Do the same thing in the ilockrewardsdata directory.

Finally navigate to the ilockrewards directory and again, do the same thing.

To run a contract, upload to [testnet.alephzero.org](https://testnet.alephzero.org).
