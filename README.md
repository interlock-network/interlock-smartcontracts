<img align="right" width="300" height="300" src="https://uploads-ssl.webflow.com/6293b370c2da3eda80121e92/6293d7cffa42ae33001294d1_interlock-visual-hero.png">

# Interlock Network Smart Contracts

Welcome! This repo hosts development of all smart contracts that actualize our [**Interlock token model**](https://github.com/interlock-network/interlock-models) and [**DeSec**](https://docs.interlock.network) infrastructure. It exists to be transparent about our contract development.

Contracts are written in [ink!](https://use.ink), employ [Openbrush](https://openbrush.io), and are hosted on the [Aleph Zero blockchain](https://alephzero.org).

## Contracts:

#### [$ILOCK PSP22 Rewards Smart Contract](./contract_ilockmvp) ~ [[DOCUMENTATION]](https://interlock-network.github.io/interlock-smartcontracts/contract_ilockmvp/docs/ilockmvp/)

This is our primary PSP22 token contract, which contains provisions for rewarding Interlockers, enforcing a token vesting schedule, higher-resolution book keeping, and connecting approved external application contracts for 'superuser' privileged access to internal contract functionalities and single-transaction purchases paid in $ILOCK token. (Read more in Universal Access NFT and Port/Socket Application Template.) This contract employs ink! 4 and Openbrush 3.

#### [Universal Access NFT](./contract_uanft) ~ [[DOCUMENTATION]](https://interlock-network.github.io/interlock-smartcontracts/contract_uanft/docs/uanft/)

This is a PSP34 token contract that is capable of managing general access credentials, be them username/password pairs, API keys, two-factor authentication, and even applications such as traditional software licenses. This contract employs ink! 4, Openbrush 3, and is fully compatible with the Art Zero marketplace.

#### [Port/Socket Application Template](./contract_application) ~ [[DOCUMENTATION]](https://interlock-network.github.io/interlock-smartcontracts/contract_application/docs/application/)

This is a template for our port/socket contract application formalism, to be an integral part in the upcoming Interlock Network _Community Node_ architecture. The primary purpose of this contract template is to provide approved entities with a way to interact with our PSP22 token contract internals but _without_ needing Interlock Network to act as a transaction relay for only-owner functionalities (for example, minting an Interlock-owned NFT in exchange for $ILOCK). This contract employs ink! 4.

## Extras:

Before our move to Aleph Zero, we were developing for an Ethereum-Wormhole-Solana blockchain stack. All this work is available in `ARCHIVE`, for transparency and future reference.

We have enabled [discussions](https://github.com/interlock-network/INTR-smartcontracts/discussions) and will be open to feedback throughout the development process. Please feel free to chime in on a discussion or open an [issue](https://github.com/interlock-network/INTR-smartcontracts/issues), or reach out directly to blockchain engineer @blairmunroakusa on TG, Discord, or Github.

Likewise, feel free to comment on issues as they come up. We use an org-wide labeling system, but will have specific labels for this repository to denote contract type, client tooling, etc.

Enjoy!
