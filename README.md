# Interlock Smart Contracts

Welcome to Interlock! In this repository, we host development of all smart contracts that actualize our **token model** and **DeSec** infrastructure. For the time being, a given contract will be one of two types: an Ethereum smart contract, or a Solana program _(note that, a Solana smart contract is called a **program** in the literature)_. In addition to one directory per blockchain's contracts, we also host a directory for each that contains resources such as references, docs, plans, etc. We include a directory devoted to client side scripts and tooling that we use to test and validate these contracts. Finally, we include a directory dedicated to testing/validation results, procedures, difficulties, etc.

Wormhole is the message-passing bridge we are using to pass INTR between Ethereum and Solana blockchains. Because Wormhole is a _protocol (validated by 19 guardian nodes)_ that bridges smart contracts across chains, the logic underpinning any Wormhole functionality must be build into the Solana and Ethereum contracts themselves. (In other words, there is no directory for Wormhole contracts, because this is not a thing.) We will however, maintain a Wormhole resource directory with references, docs, plans, etc.

The purpose of all this is to be transparent about our contract development (which, all smart contracts a public anyways, so why not.)

We have enabled discussions and will be open to feedback throughout the development process. Please feel free to chime in on a discussion or issue, or reach out directly to blockchain engineer @blairmunroakusa on TG, Discord, or Github if you have anything direct.

Likewise feel free to comment on issues as they come up. We use an org-wide labeling system, but will have specific labels for this repository to denote contract type, client tooling, etc.

[In parting, may we leave you with this gift, oh fellow dev traveller -- a Solana program template repo for arbitrarily complex contracts with **no** framework.](https://github.com/interlock-network/solana-program-template)

Enjoy!
