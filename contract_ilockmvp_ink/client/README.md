# Interlock Client Scripts

Clientside scripts for calling ILOCK token and MVP rewards contract live here.

## NOTES

Signer secret mnemonic is stored in `.mnemonic.json`.

Contract target address is stored in `.env`.

You are welcome to tinker with the currently deployed contract, or instantiate your own version to tinker with.

On testnet, this contract is located at `5EZUMgCKeLN8V6ZZ7uqSX9kQGnkVPySzS5B3WZDVZZUqjLpy`.

The code hash is `0xbc72ee5b654454eb3a976f8fe3ad73c90d63b775819bffbd15fcc8f712a292d0`, which you would use to instantiate your own contract instance.

Contract instantiation from code hash should be done here: `https://test.azero.dev`.

Click the `Developer` tab, and click `Contracts` in the dropdown.

To add the public test token contract, click the plus-icon next to `Add an existing contract`.

This will bring up a popup box where you are instructed to add the existing contract address (above), and select the contract ABI from your file system. This ABI is located at `../target/ink/metadata.json` from this README's directory.

If you want to create your own contract to play with, click the `Add an existing code hash` plus-icon.

You will be promped to upload the code bundle, found here `../target/ink/ilocktoken.contract`. Likewise add the ABI, again, here `../target/ink/metadata.json`. Name the code hash. Now you can click on the code hash icon at the bottom of your screen to intantiate your own instance of the ilock token test contract.

(If you are using your own account, you will need to get aleph test token (TZERO) from the faucet here: `https://faucet.test.azero.dev`.)

These contracts may be manipulated from testnet, or from the `sendTransaction.js` and `queryContract.js` scripts in this directory. If you would like to use the scripts here to control the contract from your terminal, refer to the script files for the calling syntax and arguments.
