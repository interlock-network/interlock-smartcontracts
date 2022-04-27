# ERC-20 Design Doc

##### ERC20 stands for Ethereum Request for Comments #20, published as an Ethereum Improvement Proposal (EIP20) back in 2015.

##### This document contains design information regarding the INTR ERC20 implementation, as it relates to INTR high-level token model and integration needs with other blockchains, like Solana.

### Starting point

First some references:

[EIP-20: Token Standard](https://eips.ethereum.org/EIPS/eip-20) (the official ERC20 standard)

[OpenZeppelin](https://docs.openzeppelin.com/contracts/4.x/erc20)

[ConsenSys](https://github.com/ConsenSys/Tokens/tree/fdf687c69d998266a95f15216b1955a4965a0a6d/contracts/eip20)

[ERC-20 TOKEN STANDARD](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/)

### Thought stream

We will be fine using the 18 decimal standard. Solana has minor difficulty with u128 division, but nothing insurmountable.

[For now, this review is revolving around OpenZeppelin, an authority in the ERC20 implementation.](https://docs.openzeppelin.com/contracts/4.x/api/token/erc20#IERC20)

The interfaces and contracts listed at this link will serve as the building blocks we need in our ERC20 implementation, whether it is drawn from OpenZeppelins implementation or not. I will address each of these according to their relevant functionality:

#### IERC20

Core interface that defines ERC20 token, includes 6 functions and 2 events. Can't do without these, by definition.

Functions: totalSupply(), balanceOf(account), transfer(to, amount), allowance(owner, spender), approve(spender, amount), transferFrom(from, to, amount)

Events: Transfer(from, to, value), Approval(owner, spender, value)

#### IERC20Metadata

Extends core interface to include name(), symbol(), and decimals() functions. This will be valuable for integrating with other parties such as exchanges and wallets, plus whatever else we may need the metadata for.

... [working]
