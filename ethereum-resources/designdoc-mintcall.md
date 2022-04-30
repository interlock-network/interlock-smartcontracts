# Mint Call Design Document

### Objective

The INTR mint call will involve various stakeholders belonging to several different distribution pools and emission schedules. The objective of this design doc is to outline the implementation details of the stakeholder mint call at TGE.

### Constrained objectives

1. Decentralize ETH gas expense, to distribute cost of mint call.

2. Accomodate upwards of 10,000-50,000 individual stakeholders. 

3. Make mint call claiming process secure, avoiding double-dipping.

4. Make mint call stakeholder record persistent for monthly emission schedule.

### Working design


Instead of clumsy formal signing with a ton of Kekkak256 calls on chain and on client, it makes more sense to pregenerate secret key hashes: Kekkak256(32Bwhitelistkey.whitelisterEmail) (do this serverside to avoid exposing whitelist key).

To register whitelister wallet on mint contract, we send whitelister their secret key hash. They visit a simple single page mint call and enter three bits of information: email, secret key hash, and wallet address.

Onchain validation goes like this: concatanate whitelist key with email, compute Kekkak256 hash, if this hash matches secret key hash, then wallet address is an authenticated whiltelister, and is added to the whitelister mapping array.

```
VV     server     VV 
--------------------
(whitelist sig key) 			 (mintcall website)
         .	    out---> whitelisters -->-|
   (email table)			     |	+email	
         ^	  			     |  +hashkey
         |				     |  +wallet address
  Kekkak 256 hash			    ~~~ < submit mintcall tx
--------------------			     V 
		       mint contract >> compute kekkak256(whitelistkey.email)
					compare onchain hash to proffered hashkey
					require(onchainhash == hashkey, "fail")
					add wallet to map table
					transfer tokent to wallet

					happy whitelister
```
Downside...this does not directly integrate wallets...but perhaps this is easy to do on a single page webapp.

^^ UPDATE: blairmunroakusa@1232Sat.30Apr22.anch.AK:gc
VV old
```
pregenerate signatures	-->	distribute signatures to stakeholders
(EIP712 WL strategy)		(requires stakeholder addresses upfront)
						|
						|
						V
				stakeholders make mint call, proffering signature
				which is checked onchain, extracting wallet
				address on success, to pass to mint() function
```
This approach will serve as a modifier in the mint() function.

Downside, we will need wallet addresses ahead of time to make it smooth. [??]

This should work to _initiate_ the emission process. (IE, this is how a stakeholder will 'register' their wallet with the ERC20 mint.

### References

[EIP712-whitelisting(.sol)](https://github.com/msfeldstein/EIP712-whitelisting/blob/main/contracts/EIP712Whitelisting.sol)

[EIP712-whitelisting(.ts)](https://github.com/msfeldstein/EIP712-whitelisting/blob/main/test/signWhitelist.ts#L12)

[EIP712-spec](https://eips.ethereum.org/EIPS/eip-712)
