# Interlock Smart Contracts

Welcome! In this repository, we host development of all smart contracts that actualize our **token model** and **DeSec** infrastructure. For the time being, a given contract will be one of two types: an Ethereum smart contract, or a Solana program. (Note that a Solana smart contract is called a **program** in the literature.)

In addition to one directory per blockchain's contracts, we also host a directory for each that contains resources such as references, docs, plans, etc. Client side scripts and tooling that we use to validate and manipulate these contracts are contained in a `client` directory in each respective contract directory. Finally, we include a directory dedicated to testing/validation results, procedures, difficulties, etc.

Wormhole is the message-passing bridge we are using to pass $INTR tokens between Ethereum and Solana blockchains. There is no directory for Wormhole contracts because Wormhole is a **protocol** (validated by 19 guardian nodes) that bridges smart contracts across chains, and the logic underpinning any Wormhole functionality must be built into the Solana and Ethereum contracts themselves. We will however, maintain a Wormhole resource directory with references, docs, plans, etc.

The purpose of all this is to be transparent about our contract development.

We have enabled [discussions](https://github.com/interlock-network/INTR-smartcontracts/discussions) and will be open to feedback throughout the development process. Please feel free to chime in on a discussion or [issue](https://github.com/interlock-network/INTR-smartcontracts/issues), or reach out directly to blockchain engineer @blairmunroakusa on TG, Discord, or Github if you have anything direct.

Likewise, feel free to comment on issues as they come up. We use an org-wide labeling system, but will have specific labels for this repository to denote contract type, client tooling, etc.

In parting, may we leave you with [this gift](https://github.com/interlock-network/solana-program-template), oh fellow dev traveller -- a Solana program template repo for arbitrarily complex contracts with **no** framework.

Enjoy!
