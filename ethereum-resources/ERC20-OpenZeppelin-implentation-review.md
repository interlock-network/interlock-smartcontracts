# ERC-20 Implementation review, OpenZeppelin

##### ERC20 stands for Ethereum Request for Comments #20, published as an Ethereum Improvement Proposal (EIP20) back in 2015.

##### This document contains design information regarding the INTR ERC20 implementation, as it relates to INTR high-level token model and ~iintegration needs with other blockchains, like Solana.~

#### This document turned into a feasibility review of OpenZeppelin as a candidate for the ERC20 INTR mint contract scaffolding.

## Starting point

First some references:

[EIP-20: Token Standard](https://eips.ethereum.org/EIPS/eip-20) (the official ERC20 standard)

Recommended by Ethereum as implementations:

[OpenZeppelin](https://docs.openzeppelin.com/contracts/4.x/erc20)

[ConsenSys](https://github.com/ConsenSys/Tokens/tree/fdf687c69d998266a95f15216b1955a4965a0a6d/contracts/eip20)

[ERC-20 TOKEN STANDARD](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/)

## thought stream on OpenZeppelin ERC20 implementation

We will be fine using the 18 decimal standard. Solana has minor difficulty with u128 division, but nothing insurmountable.

[For now, this review is revolving around OpenZeppelin, an authority in the ERC20 implementation.](https://docs.openzeppelin.com/contracts/4.x/api/token/erc20#IERC20)

The interfaces and contracts listed at this link will serve as the building blocks we need in our ERC20 implementation, whether it is drawn from OpenZeppelins implementation or not. I will address each of these according to their relevant functionality:

#### IERC20 (I for interface)

Core interface that defines ERC20 token, includes 6 functions and 2 events. Can't do without these, by definition.

Functions: totalSupply(), balanceOf(account), transfer(to, amount), allowance(owner, spender), approve(spender, amount), transferFrom(from, to, amount)

Events: Transfer(from, to, value), Approval(owner, spender, value)

#### IERC20Metadata (I for interface)

Extends core interface to include optional name(), symbol(), and decimals() functions. This will be valuable for integrating with other parties such as exchanges and wallets, plus whatever else we may need the metadata for.

#### ERC20Burnable (extension)

We will need to employ this extension to ensure we have the ability to burn token in a **transparent, verifiable way**. The alternative is just to send token to a burner account with no prikey, but this is not as transparent.

#### ERC20Capped (extension)

This is another function to provide verifiability of INTR features. A key feature of the INTR emission schedule is a 1,000,000,000 (1B) token supply cap. The extension provides a cap constructor that enforces a cap on the \_mint function upon contract ERC20 contract creation (plus a public getter cap() function for verifying the cap).

#### ERC20Pausable (extension)

The Pausable extension will allow us to build in a sefety mechanism where if we discover a large bug in auxilary contracts, we can freeze all token transfers to prevent people from losing money. We can also use it to prevent trades until the end of an evaluation period, though I do not believe this is our intention with INTR. [?]

#### ERC20Snapshot (extension)

I am not sure we will need this one. It's purpose is to ostensibly roll back transfers. Up for discussion.

#### ERC20Votes (extension)

I am not sure we will be implementing voting on Ethereum, for we would rather reserve that for Solana due to lower tx fees (gas). Up for discussion.

#### ERC20VotesComp (extension)

We only need this if we need exact compatibility with COMP. COMP is the ERC20 asset that empowers community governance of the Compound Protocol. Pretty sure we don't need this, but worth a discussion.

#### ERC20Wrapper (extension)

This allows the token to wrap around other tokens, or for other tokens to wrap around it. (vaguely) I am not sure we need this. Up for discussion.

#### ERC20FlashMint (extension)

WE SHOULD STAY AWAY FROM FLASH MINTING LIKE THE PLAGUE. Flash-anything leads to dynamics beyond our immediate understandings...many 'undocumented features'.

#### SafeERC20 (utility)

Safe-wrapped operations make it so we don't have to deal with improper (unreturned) failure from a non-compliant ERC20 token if we have to interact with one. I do not believe this will ever happen, so be probably don't need it.

#### TokenTimeLock (utility)

We may absolutely need this utility, because it gives us the ability to 'vest' tokens held by different categories of holders.

#### supply

**We must put these piece together in a way that allows us to mint INTR according to the emission schedule, capped, with vesting, and multiple recipients / pools.**

To do this, we must write our own extension of ERC20 containing the logic needed to mint over time.

#### path of least resistance / most flexibility

We will want to start from the ERC20PresetMinterPauser, and build in/out from there. We will want to extend to capped and TokenTimeLocked from there. Beyond this, precise implementation details depend on details from our token model and emission schedule.

## thought stream on ConsenSys ERC20 implementation

No. ConsenSys as an implementation is more of a toy reference. OpenZeppelin is very well established with a reputation for secure, battle hardened implementations.

## concluding thoughts for review session:

#### OpenZeppelin should be used by Interlock to build token contracts on Ethereum. They are highly reputable, and will help with establishing our token as robust and secure, contractually.
