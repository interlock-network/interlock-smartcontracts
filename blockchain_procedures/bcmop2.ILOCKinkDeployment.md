# MOP for deploying ILOCK solidity contract on Arbitrum

### 0 - Get set up:

- determine multisig signatory number and threshold
- gather multisig signatory addresses
- create multisig Safe with [https://safe.global](https://safe.global)
- transfer $ARB to multisig Safe to pay for transactions
- identify one of these addresses to serve as the contract admin (ie, for registering stakeholder token stakes post-TGE)
- transfer $ARB to this contract admin address (to pay for stake registration and deployment)
- gather stakeholder info, which must be
    - in csv format
    - one file contains all stakes, with columns (stakeholder, share, pool, {and perhaps email})
        - named `STAKE_DATA.csv`
    - the other file contains a single list of stakeholders
        - named `STAKEHOLDER_DATA.csv`

### 1 - Insert data into contract workspace within `contract_ilockmvp_sol`:

- place both stakeholder info files in the directory `contract_ilockmvp_sol/data`
- populate relevant environment variables in `.env.prod` file. You will need:
    - contract owner/admin account address and private key
    - contract multisig Safe address

### 2 - Verify that hardhat works.

(Do stuff here)

### 3 - Deploy the token contract.

```
run ILOCKdeploy.ts here
```

Copy the contractAddress field printed out to terminal, or retrieve from `./data/contractAdmin.json`, then paste it into `./.env.prod` for the `PROXY_ADDRESS` variable. Do not include quotations.

The contract is now live, but there has not yet been the token generation event.

### 4 - Transfer proxy admin to the multisig Safe address.

```
run enableMultisig.ts here
```

You should see the transaction receipt.

### 5 - Register stakes.

This is the intensive step, and may take a great deal of time. Registering the stakeholder stakes constitues an irreversible action, but the most that can go wrong is that a batch fails to complete due to a network error or congestion.

```
run registerStakes.ts here
```

If the script is successful, then there will be a list of stake identifiers in `./data/stakeIdentifiers.json, and there will be one identifier per stake.

```
run checkStakes.ts here
```

If the script is not successful, then there will be fewer identifiers than stakes, in which case you need to complete the registration by running the following:

```
run completeStakeRegistration.ts TODO
```

Again, check progress:

```
run checkStakes.ts here
```

Eventually this process will complete. Perform one final check to make sure each stake is accounted for.

### 6 - Validate contract code on Arbiscan.

This makes our contract inspectible, proving it is what we say it is.

```
run validateContract.ts
(or do manually on Arbiscan, not sure yet)
```

### 7 - Trigger token generation event.

This is it, when TGE is triggered, all 1B tokens will be distributed to their respective pool addresses, the vesting clock will start, the contract admin/owner will assume control of any tokens sent to contract in the future, and the next payout date will be calculated.

### 8 - Perform final sanity checks.

Jump onto Arbiscan and check basic functionality one last time. In particular, fetch list of pool addresses and check their token balances. These should match the token distribution schedule. Check pool data.

### 9 - Publish admin log, stake registration log, and claim receipts.

The repo will be connected already to the Interlock Network Smart Contracts repository. From the contract directory, push the deployment data to the repo.

```
git add .;
git commit -m "ILOCK deployment information";
git push -u origin master
```

### 10 - Distribute stake claim receipts.

This will be simplest done as an email to all stakeholders notifying them of the contract address. Once they visit the address on Arbiscan, they can view their claim identifiers (one per stake) by submiting their address in the `getStakeIdentifiers(address stakeholder)` function. These stakeIdentifiers then can be used to claim each individual stake, assuming the vesting schedule permits the claim.

For simple stake information there are the following view functions for stakeholders to use:


