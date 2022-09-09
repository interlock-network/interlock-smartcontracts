# ILOCKsupreme Smart Contracts

ILOCKsupreme is the collections of all Interlock Network smart contracts on Aleph Zero. For now this consists of one ERC20-style token contract, and one contract to manage user rewards for surfing the web with browser extension.

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

Then navigate to the ilockrewards directory.
