const path = require('path');
const prompt = require('prompt-sync')({sigint: true});
const fork = require('child_process').fork;
const program = path.resolve('verifyWallet.js');


const amount = prompt(`Please enter amount: `);
const wallet = prompt(`Please enter wallet: `);

const child = fork(program);

child.send({amount: amount, wallet: wallet});

child.on('message', message => {
	console.log('status:', message);
});

child.on('close', () => {
	console.log('done')
});

