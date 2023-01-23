# Interlock NFT Smart Contracts

This is the main space for Interlock Network NFT smart contracts hosted on Aleph Zero. For now this consists of the contract `access` which manages interlocker access to various kinds of priviledge.

## access NFT

Beyond simple collectibility, access contract may be deployed for any different class of access NFT. For now, the first class is a line of software licenses to approve Discord moderators to use the Interlock Bouncer Discord bot in their Discord community's server. If a discord moderator would like to purchase a license NFT to use for their community server, they will then need to authenticate their ownership of the NFT on the Interlock license authentication page. If the Discord moderator can prove that they posses the wallet that owns the license NFT (via a micropayment transfer specified by Interlock during authentication process), then the moderator's Bouncer bot instance will be activated for their server. If the license changes hands, then the authentication process must be repeated for the new wallet's owner.

The other class of the access NFT is the vip membership class. If a stellar interlocker community member has contributed to our community in a meaningful manner, we may grant them a VIP membership certificate in the form of an NFT. By the same process outlined above, this member may attain access to privileged spaces within the Interlock ecosystem by authenticating their ownership of the wallet that contains the membership NFT. (In this case, choosing a login ID/password pair upon successful authentication.)

This access NFT is funded in part by the Aleph Zero grant program, ultimately to contribute to the Aleph Zero community at large to empower others to launch their own acces NFTs.

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
!!! NOTE:
You may need to install this particular version of cargo-contract:
```
cargo install cargo-contract --version 2.0.0-beta
```
You will only need this if you find that compiling produces this error:
```
ERROR: No 'ink_lang' dependency found
```

### Finally
Run
```
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

## How to build

Run from the `access` directory:
```
cargo contract build
```
To run a contract, `upload metadata.json` and `ilockmvp.wasm` (in `target`) to [testnet.alephzero.org](https://testnet.alephzero.org).

