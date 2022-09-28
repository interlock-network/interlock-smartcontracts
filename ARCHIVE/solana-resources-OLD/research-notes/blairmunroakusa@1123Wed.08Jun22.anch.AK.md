## Researching stake account architecture / topology

Main explorations:

1) extension wallet integration to locally manage (user-account => URL => token-account) mappings

2) cost effective scheme to partition URL grey entity token stakes into stake account 'groups'

```
blairmunroakusa@1157Wed.08Jun22.anch.AK:rc
```

I need to figure out what the token model requires data-wise, when it comes down to staking/rewarding per grey URL.

Reviewing [Solana bootcamp](https://www.youtube.com/watch?v=GwhRWde3Ckw&list=PLilwLeBwGuK7Z2dXft_pmLZ675fuPgkA0&index=2), to scrape reference implmentations.

Back to [cookbook](https://solanacookbook.com/references/token.html#how-to-get-a-token-mint). 

I believe solana-staking-program (from SPL) is just for SOL.

Looking into whether or not associated token accounts may be PDA.

[~ From here ~](https://medium.com/coinmonks/solana-programs-part-2-understanding-spl-associated-token-account-25082b9b5471)
A few take-aways on associated token account (ATA):

- Every ATA is a PDA
- Every ATA is a token account
- Every ATA corresponds to a public key (e.g., wallet address) and a token mint, and is owned by the *wallet address* (see below for )
- ATA may be created by anybody (not necessarily the wallet address’ signer)
- Nested ATA may be created and the RecoverNested instruction can be used to close a nested ATA.

So yes, every ATA is a PDA.

Now exploring the concept of nested ATAs.

.

.

.

So the day's conclusion goes like this (enter, brain-dump):

To simplify *everything*, the extension will need to remember URLs that user stakes on. The extension will also need to act as the aggregator, to provide accurate cross-chain token balances, WHICH IS TO AVOID NEEDING TO UPDATE ETHEREUM TOKEN BALANCES FOR EACH INTR STAKING TRANSACTION ON SOLANA CHAIN (too costly).

NB - the only way to make this work  may be to xfer entire INTR balance from Eth. to Sol. before staking is allowed, making it so that any balance transfered to ethereum account may be withdrawn without impacting maths on solana side.

For example. 'update account' (pseudo command tbd where to place) would wipe ethereum acct balance to zero and transfer to solana holding account. Extension before would show 'vault' (amount INTR untouchable in ethereum acct balance), 'register' (amount INTR available to stake) and 'stakes' (overall, or itemized). So, to make more INTR available to stake, 'update account' would zero out balance on ethereum and xfer to SOlana via wormhole. [[Perhaps thet update account command needs to be variable ito the amount to xfer from one chain to another]] YES...

...Call this commands 'charge' and 'cashout', to be implemented on the extension. Charge allows user to transfer an amount of their choosing from vault to register. Cashout allows user to transfer an amount from register to vault. User will be made aware of the tx fees associated with these actions. The idea is to update ethereum balances ONLY AS NECESSARY.

NB - In thise scheme, ethereum account allowances will need special consideration, which I do not have mental cycles to dig into at the very moment.

Anyways...back to extension. Extension would display total INTR balance, a combination of vault, register, and URL stakes. This would be aggregated via get methods to respective accounts. Vault account address would be stored in extension...This would be one keypair. Register account address would be derived from vault account pubkey. Stake accounts would be derived from register account and URL seeds. Stake URLs would be stored in extension to avoid making API calls more than necessary.

register address = vault-address-seed X username?

stake account address = URL-seed X register-address

This would make it impossible to sniff an aggregate stake for a given URL. [? true] Is this good, or is this bad??

Maybe we need to situate thing so that is is possible to gather coarse data on URL grey entities...

By this configuration, if the user lost the URLs they've staked on, they lose their stake...so if the extension fails to persist, they're screwed.

!!!!! MAY BE POSSIBLE TO USE SOLANA'S NAME SERVICE SPL PROGRAM TO DEAL WITH GREAY ENTITY URLs !!!!! (just a sidenote)

We will need some mechanism to communicate to other stake accounts that a grey entity has be 'determined' this-or-that...this is a pretty big design problem.

Remember, the main considerations here are COST.

Computer dying...back to paper + pen + brain.


