<img align="right" width="300" height="300" src="https://assets-global.website-files.com/64d9930f57641d176ab09b78/64df168bc9c81b986abd0e9b_img-ilock-token.png">

# $ILOCK ERC20 Vesting and Rewards Smart Contract

## Some Notes:

The `ilockmvp` is the main Interlock Network smart contract to be hosted on Arbitrum. It is an Open Zeppelin ERC20 token contract, version 5. There are no special functionalities added to this contract, with the exception of that the `Total Supply` increments when tokens are transferred away from the contract address (via rewards or TokenOps vesting transfers) and it decrements when tokens are deposited at the contract address.

To enforce the vesting schedule, there is only one mint operation, which occurs on TGE (hence the `Total Supply` increment/decrement approach).

The contract employs the Open Zeppelin transparent proxy pattern for upgradeability.

We use Defender 2.0 to manage and monitor the contract and its activity.

## How to get setup and build:

This is a standard Hardhat project. If you are unfamiliar with Hardhat, [then start here](https://hardhat.org/hardhat-runner/docs/getting-started#overview).

### (But for now, compile and run stuff lest not to forget...)

Compile contracts:
```
npx hardhat compile
```

Run local EVM node (blockchain):
```
npx hardhat node
```

Run script:
```
npx hardhat run --network localhost scripts/ILOCKdeploy.ts
```
