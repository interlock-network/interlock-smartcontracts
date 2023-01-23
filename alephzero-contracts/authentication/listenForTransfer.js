//
// INTERLOCK NETWORK - LISTEN FOR TRANSFER
// PSP34 ACCESS NFT AUTHENTICATION
//

// imports
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');

// constants
const OWNER_MNEMONIC = require('./.mnemonic.json');
const OWNER_mnemonic = OWNER_MNEMONIC.mnemonic;
const metadata = require('./access_metadata.json');

async function main(message) {

  // establish connection with blockchain
  console.log('');
  const wsProvider = new WsProvider('wss://ws.test.azero.dev');
  const api = await ApiPromise.create({ provider: wsProvider });
  console.log('');

  // create signing keypair
  const keyring = new Keyring({type: 'sr25519'});
  const OWNER_pair = keyring.addFromUri(OWNER_mnemonic);

  // Subscribe to system events via storage
  api.query.system.events((events) => {

    // Loop through the Vec<EventRecord>
    events.forEach((record) => {

      // Extract the phase, event and the event types
      const { event, phase } = record;
      const types = event.typeDef;
      if (event.method == 'Transfer') {

        // check for verification transfers
	//
	// from Interlock
        if ( event.data[0] == '5HYyQHZQ92iGTaWyxeJajvxk6ZGBYdYGQWdJHrU7cH9eBGxC' &&
          event.data[1] == message.wallet) {
          process.send('authentication transfer complete');
          process.send('waiting on verification transfer');
        //
        // from wallet holder
        } else if ( event.data[0] == message.wallet &&
          event.data[1] == '5HYyQHZQ92iGTaWyxeJajvxk6ZGBYdYGQWdJHrU7cH9eBGxC'){
          process.send('verification transfer complete');
          process.send('wallet authenticated');
          process.exit();
        }
      }
    });
  });

  // create transfer object
  const transfer = api.tx.balances.transfer(message.wallet, message.amount);

  // Sign and send the transaction using our account
  const hash = await transfer.signAndSend(OWNER_pair);
  console.log(`Authentication transfer sent with hash: ${hash.toHex()}\n`);
}

process.on('message', message => {
  main(message).catch((error) => {
    console.error(error);
    process.exit(-1);
  });
});
