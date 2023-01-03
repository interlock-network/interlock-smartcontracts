const path = require('path');
const prompt = require('prompt-sync')({sigint: true});

const fork = require('child_process').fork;
const program = path.resolve('verifyWallet.js');


const amount = 1;
const wallet = '5EtTSfiarDaXaDiKfVkQii3eCDnbHtEdwggyGX3Zbe45mXH7';


// VVVV loop here to listen for request from webpage VVVV
const child = fork(program);

child.send({amount: amount, wallet: wallet});

child.on('message', message => {
	console.log('status:', message);
	if (message.type == "authentication complete") {

		//////////////////////////////////
		//
		// insert database entry here
		//
		//////////////////////////////////

		//////////////////////////////////
		//
		// prompt webpage to prompt user to enter credentials
		//
		//////////////////////////////////

		//////////////////////////////////
		//
		// listen for credentials and enter in DB?
		//
		//////////////////////////////////

		child.close();
	};
});
// ^^^^ loop here to listen for request from webpage ^^^^



