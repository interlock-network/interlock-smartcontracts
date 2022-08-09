# Comparing blockchains, 080922

This is the most recent blockchain comparison. To keep things simple, this document will focus primarily on Solana, Ethereum, Avalanche, and Wormhole.

The thesis of this document is that it behooves us to replace Ethereum with Avalanche, altering the original plan by minting token on Avalanche, and bridging token to Solana for higher-frequency Security Staking micro transactions.

## First a note on centralization

A large criticism of the Solana ecosystem is that the Solana blockchain is too centralized. Before moving forward, it will be best to address this.

There are not many good ways to quantify decentralization. The primary way to do so is by measuring something called the Nakamoto coefficient. Although it is difficult to find a consistent definition of this coefficient, a rough one is that the Nakamoto coefficient is the number of validating nodes which need to collude if they want to shut down the network. The source for this report takes a conservative approach by defining this as 34% of the network. This information comes from a Tweet thread [here](https://twitter.com/larry0x/status/1422480942711689229).

The thread compares the top 8 proof of stake blockchains. The thread also contains a tweet from Vitalik Butering stating the Nakamoto coefficient for Ethereum is 12, for a 34% attack. (To confuse matters, an academic study [here](https://arxiv.org/pdf/2101.10699.pdf) reports the Nakamoto coefficient for Ethereum and Bitcoin as hovering between two and five, meaning only between two and five mining pools need be taken over in order to control the network.

This source concludes that Solana and Avalanche are the most decentralized, Solana with a coefficient of 18 and Avalanche with a coefficient of 26. This is more decentralized than 12 reported by founder for Ethereum network.

An important point though, is that there is very little data and analysis regarding Nakamoto coefficients. The statements above should be taken with caution, but the conclusion is that Solana is actually very decentralized compared to other PoS blockchains.

## Looking into Avalanche

A big design problem at the moment is minting and bridging gas fees between Ethereum and whereever we choose to host higher-frequency security staking features. The issue is that if we are using Solana for Security Staking features, then anytime somebody wants to 'cash out', they will need to bridge their tokens to Ethereum, then perform whatever transfers they wish to. This will cost between 1-5$ for each transaction to cash out. Likewise, it will cost the same for users to buy ILOCK on an exchange then bridge them to Solana for security staking. So, we are talking about gas fees between 10-50 ILOCK tokens, at initial ILOCK price, just to enter and exit tokens from the security staking portion of the Interlock network.

Taking a look at Avalanche however, we find that Avalanche as its own Ethereum Virtual Machine (EVM) with same API are Ethereum network. It is also possible for users to synchronize their Ethereum address with the Avalanche EVM. (Meaning, a metamask user could use their same Ethereum account address on the Avalanche chain. It also bears reminding that an EVM means all the Solidity contracts that run on Ethereum will also run on Avalanche.

As it turns out, Avalanche has much lower transaction fees than Ethereum, and also has much higher transaction rate. Also, according to the centralization analysis above -- if accurate -- Avalanche is way more decentralized than Ethereum. Comparing costs, the same operations on Avalanche should be 100 to 1000 fold cheaper than Ethereum, so the price of entry and exit is on the order of 0.1-1.0 ILOCK.

## Avalanche and Solana

It would be tempting to just use Avalanche instead of Ethereum <-> Wormhole <-> Solana, but the problem is that for higher-frequency security staking (and in micro amounts) Avalanche transaction fees are still too high. Whereas typical Avalanche transactions cost between 0.1 and 0.01$, a typical Solana transaction costs between 0.0005 and 0.00025$. If typical security stakes are between 1 and 10 ILOCK, this would mean on Avalanche one staking operation will cost the user between 10 and 100% of their stake. Solana fees on the other hand would cost the user between 0.5 and 0.25% of their stake. Security Staking on Avalanche would therefor be cost prohibitive and disincentivize staking.

So falling back to the original architecture (Ethereum compatible chain for minting, then bridging to high tx rate low fee chain) it turns out that wormhole also works between Avalanche and Solana. A (potentially) viable alternative to the original plan that would save on prohibitive minting/entry/exit fees is Avalanche <-> Wormhole <-> Solana.

!!! Where my personal knowledge fails, this design option will only be possible if we can verify that we can list on the same DEX by implementing an ERC20 token on Avalanche !!!

## Cardano

Cardano is a mature PoS blockchain with market cap greater than Solana or Avalanche at 17B$. Cardano may possibly serve as an all-in-one alternative to the bridging architecture.

The down side is that Cardano has no clear EVM capability, and their smart contract languages are relatively obscure. There seems to be no way to achieve address clones between Ethereum and Cardano because the address formats are different. I cannot find transaction fee data on Cardano Explorer (bug/error in explorer), so no real way to tell how much it would cost to operate on that blockchain.

I don't think Cardano will be a good fit for what we are trying to accomplish.

## Closing thoughts on Solana

Since the wallet hack last week, the price of Solana has essentially recovered, resuming its trend to match other peers' prices in the market.

It should be known, that security staking on Solana would not be vulnerable to a wallet hack like last week (unless users want to pay directly with SOL. (This is because ILOCK for security staking will just be a number stored in state account variables. Such a number representing ILOCK tokens will not be accessible via wallets looking for the STP or ERC20 style tokens.)
