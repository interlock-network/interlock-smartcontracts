<img align="right" width="300" height="300" src="https://assets-global.website-files.com/64d9930f57641d176ab09b78/64dde3b1459a01ddf7b4a529_interlock-logo-large.webp">

# Interlock Network Solidity Smart Contracts

Welcome! This repo hosts development of all solidity smart contracts that actualize our [Interlock token model](https://github.com/interlock-network/interlock-models) and [DeSec](https://docs.interlock.network) infrastructure. It exists to be transparent about our contract development.

Contracts are written in solidity and are currently hosted on the [Arbitrum blockchain](https://arbitrum.io).

## SECURITY NOTICE:

If you find something wrong or concerning, open an issue. If the finding is a sensitive security concern, then for now the best course of action is to email smartcontract POC Blair Munro directly, or reach out on discord: [blair@interlock.network](mailto:blair@interlock.network) & @blairmunroakusa.

## Multichain:

Interlock Network has been and always will be, blockchain agnostic. The first phase of our web3 launch involves positioning for future security staking, as well as maximizing the likelihood of successful token value hike.

The ILOCK token will span two ecosystems as mentioned: Polkadot (the Aleph Zero Substrate fork), and Ethereum (the Arbitrum layer 2 EVM). Vesting and rewards will be maintained on both. Security staking will likely exist primarily on Aleph Zero, due to the superior transaction fee and throughput metrics. If security staking does NOT need to be high-frequency, then we may also implement on Arbitrum.

The choice in these two chains has been informed by active work being done to bridge the two ecosystems. We would not have chosen these chains to launch if we did not know that there would be a bridge in the near future.

## Contracts:

#### [$ILOCK](./ilockmvp.sol)

This is our EVM (ERC20) token contract written for [Arbitrum](https://arbitrum.io). Vesting will be managed in external contracts provided by [TokenOps](https://tokenops.xyz). This contract employs Solidity, drawing directly from the standard (audited) OpenZeppelin 5 contract suite. To avoid needing to perform a new audit, reward functionality will simply employ the `TransferFrom` ERC20 standard method.

## Extra and other:

We have enabled [discussions](https://github.com/interlock-network/interlock-smartcontracts-solidity/discussions) and will be open to feedback throughout the development process. Please feel free to chime in on a discussion or open an [issue](https://github.com/interlock-network/interlock-smartcontracts-solidity/issues), or reach out directly to blockchain engineer @blairmunroakusa on TG, Discord, or Github.

Likewise, feel free to comment on issues as they come up. We use an org-wide labeling system, but will have specific labels for this repository to denote contract type, client tooling, etc.

Enjoy!

# Contributing

We are open to contributions from the community. Please open an issue or pull request if you have any suggestions or improvements.

Packages are managed with [pnpm](https://pnpm.io/). To install dependencies, run:

```bash
pnpm install
```
