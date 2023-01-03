//
// INTERLOCK NETWORK - 
//
// LISTEN FOR TRANSFER EVENTS ON CHAIN
//

// imports
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { ContractPromise, CodePromise } = require('@polkadot/api-contract');

async function main () {

  // establish connection with blockchain
  console.log('');
    const wsProvider = new WsProvider('wss://ws.test.azero.dev');
    const api = await ApiPromise.create({ provider: wsProvider });
  console.log('');

  // Subscribe to system events via storage
  api.query.system.events((events) => {

    // Loop through the Vec<EventRecord>
    events.forEach((record) => {

      // Extract the phase, event and the event types
      const { event, phase } = record;
      const types = event.typeDef;

      // Show what we are busy with
      if (event.method == 'Transfer') {

        // check if from Interlock Network test account (OWNER)
        if ( event.data[0] == '5HYyQHZQ92iGTaWyxeJajvxk6ZGBYdYGQWdJHrU7cH9eBGxC' ) {
          console.log('\tTransfer\tfrom\tInterlock Network');
        } else if ( event.data[0] == '5EtTSfiarDaXaDiKfVkQii3eCDnbHtEdwggyGX3Zbe45mXH7' ){
          console.log('\tTransfer\tfrom\tInterlocker #1');
        } else {
          console.log(`\tTransfer\tfrom\t${event.data[0]}`);
        }

        // check if from Test Interlocker #1 (5EtTSfiarDaXaDiKfVkQii3eCDnbHtEdwggyGX3Zbe45mXH7)
        if ( event.data[1] == '5HYyQHZQ92iGTaWyxeJajvxk6ZGBYdYGQWdJHrU7cH9eBGxC' ) {
          console.log('\t\t\tto\tInterlock Network');
        } else if ( event.data[1] == '5EtTSfiarDaXaDiKfVkQii3eCDnbHtEdwggyGX3Zbe45mXH7' ){
          console.log('\t\t\tto\tInterlocker #1');
        } else {
          console.log(`\t\t\tto\t${event.data[1]}`);
        }

        console.log(`\t\t\tamount\t${event.data[2]}\n`);
      }
    });
  });
}

main().catch((error) => {
  console.error(error);
  process.exit(-1);
});

