//
// INTERLOCK NETWORK - SET AUTHENTICATED
// PSP34 ACCESS NFT AUTHENTICATION
//

// imports
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');

// constants
const access_metadata = require('./access_metadata.json');
const access_contract = '5EVdCVKBs3X3NHd33f6KZkmpkw2qeKqMHUVjzdD7me5m2JCS';
const OWNER_MNEMONIC = require('./.mnemonic.json');
const OWNER_mnemonic = OWNER_MNEMONIC.mnemonic;

// constants
const MEG = 1000000;
const gasLimit = 100000 * MEG;
const storageDepositLimit = null;

async function main(message) {

  try {

    // setup session
    const wsProvider = new WsProvider('wss://ws.test.azero.dev');
    const keyring = new Keyring({type: 'sr25519'});
    const api = await ApiPromise.create({ provider: wsProvider });
    const contract = new ContractPromise(api, access_metadata, access_contract);
    const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

    // perform dry run to check for errors
    const { gasRequired, storageDeposit, result, output } =
      await contract.query['setAuthenticated']
	(OWNER_pair.address, {}, {u64: message.id});

    // too much gas required?
    if (gasRequired > gasLimit) {
      console.log('tx aborted, gas required is greater than the acceptable gas limit.');
      process.exit();
    }

    // too much storage required?
    if (storageDeposit > storageDepositLimit) {
      console.log('tx aborted, storage required is greater than the acceptable storage limit.');
      process.exit();
    }

    // did the contract revert due to any errors?
    if (result.toHuman().Ok.flags == 'Revert') {
      let error = output.toHuman().Err;
      console.log(`Transaction reverts due to error: ${error}`);
      process.exit();
    }

    // submit doer tx
    let extrinsic = await contract.tx['setAuthenticated']
      ({ storageDepositLimit, gasLimit }, {u64: message.id})
      .signAndSend(OWNER_pair, result => {
        if (result.status.isInBlock) {
          console.log('in a block');
        } else if (result.status.isFinalized) {
          process.send('nft authenticated');
          process.exit();
        }
      });

  } catch(error) {

    console.log(error);
    process.exit();
  }
}

process.on('message', message => {
  main(message).catch((error) => {
    console.error(error);
    process.exit(-1);
  });
});
