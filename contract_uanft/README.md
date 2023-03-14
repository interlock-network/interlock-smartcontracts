# Universal Access NFT Smart Contract

Beyond simple collectibility, access contract may be deployed for any different class of access NFT. For now, the first class is a line of software licenses to approve Discord moderators to use the Interlock Bouncer Discord bot in their Discord community's server. If a discord moderator would like to purchase a license NFT to use for their community server, they will then need to authenticate their ownership of the NFT on the Interlock license authentication page. If the Discord moderator can prove that they posses the wallet that owns the license NFT (via a micropayment transfer specified by Interlock during authentication process), then the moderator's Bouncer bot instance will be activated for their server. If the license changes hands, then the authentication process must be repeated for the new wallet's owner.

The other class of the access NFT is the vip membership class. If a stellar interlocker community member has contributed to our community in a meaningful manner, we may grant them a VIP membership certificate in the form of an NFT. By the same process outlined above, this member may attain access to privileged spaces within the Interlock ecosystem by authenticating their ownership of the wallet that contains the membership NFT. (In this case, choosing a login ID/password pair upon successful authentication.)

This access NFT is funded in part by the Aleph Zero grant program, ultimately to contribute to the Aleph Zero community at large to empower others to launch their own acces NFTs.


## How to get setup and build:

#### See [[DOCUMENTATION]](https://interlock-network.github.io/interlock-smartcontracts/contract_uanft/docs/uanft/).

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


