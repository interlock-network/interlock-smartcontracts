# Mint Call Design Document

### Objective

The INTR mint call will involve various stakeholders belonging to several different distribution pools and emission schedules. The objective of this design doc is to outline the implementation details of the stakeholder mint call at TGE.

### Constrained objectives

1. Decentralize ETH gas expense, to distribute cost of mint call.

2. Accomodate upwards of 10,000-50,000 individual stakeholders. 

3. Make mint call claiming process secure, avoiding double-dipping.

4. Make mint call stakeholder record persistent for monthly emission schedule.

### Working design
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
