//
// INTERLOCK NETWORK - VERIFY WALLET
// PSP34 ACCESS NFT AUTHENTICATION
//

// imports
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');
const path = require('path');
const fork = require('child_process').fork;

// constants
const access_metadata = require('./access_metadata.json');
const access_contract = '5EVdCVKBs3X3NHd33f6KZkmpkw2qeKqMHUVjzdD7me5m2JCS';
const OWNER_MNEMONIC = require('./.mnemonic.json');
const OWNER_mnemonic = OWNER_MNEMONIC.mnemonic;
const TRUE = '0x74727565';
const FALSE = '0x66616c7365';
const ISAUTHENTICATED = '0x697361757468656e74696361746564';

async function main(message) {

  try {
    // setup session
    const wsProvider = new WsProvider('wss://ws.test.azero.dev');
    const keyring = new Keyring({type: 'sr25519'});
    const api = await ApiPromise.create({ provider: wsProvider });
    const contract = new ContractPromise(api, access_metadata, access_contract);
    const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

    let notAuthenticated = false;
    let notAuthenticatedId;

    // get NFT collection for wallet
    let { gasRequired, storageDeposit, result, output } =
      await contract.query['ilockerCollection'](
        OWNER_pair.address, {}, message.wallet);

    // check if the call was successful
    if (result.isOk) {

      // find nft to authenticate
      const collection = JSON.parse(JSON.stringify(output));
      for (nft in collection.ok) {

        let { gasRequired, storageDeposit, result, output } =
          await contract.query['psp34Metadata::getAttribute'](
            OWNER_pair.address, {}, {u64: collection.ok[nft].u64}, ISAUTHENTICATED);
        let authenticated = JSON.parse(JSON.stringify(output));
        if (FALSE == authenticated) {
          notAuthenticated = true;
          notAuthenticatedId = collection.ok[nft].u64;
        }
      }

      // not nfts to authenticate
      if (notAuthenticated == false) {
        process.send('nfts already authenticated')
        process.exit();

      // or authenticate one of the unauthenticated nfts
      } else if (notAuthenticated == true) {

        const listen = path.resolve('listenForTransfer.js');
        const listenChild = fork(listen);

        listenChild.send({amount: message.amount, wallet: message.wallet});
        listenChild.on('message', message => {
          console.log('status:', message);
          if (message == 'wallet authenticated') {
            const set = path.resolve('setAuthenticated.js');
            const setChild = fork(set);
            setChild.send({id: notAuthenticatedId});
            
            setChild.on('message', message => {
              if (message == 'nft authenticated') {
                process.send({
                  type: "authentication complete",
                  wallet: message.wallet,
                  id: notAuthenticatedId
                });
                process.exit();
              };
            });

          };
        });
      }
    }

  } catch(error) {

    console.log(error);
    process.send('no nfts present');
    process.exit();
  }
}

process.on('message', message => {
  main(message).catch((error) => {
    console.error(error);
    process.exit(-1);
  });
});
