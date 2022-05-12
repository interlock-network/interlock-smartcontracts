// SPDX-License-Identifier: MIT
//
// Interlock ERC-20 INTR Token Mint Platform
// 	   contract manipulation 
//
// Contributors:
// blairmunroakusa
// ...

const signData = 	require('./ERC20signDataEIP712.js');

const memberData = 	require('./memberData.json');
const WALLET =		memberData['WALLET'];
const SHARE =		memberData['SHARE'];
const POOL =		memberData['POOL'];

const signerData =	require('./signerData.json');
const CONTRACT = 	signerData['CONTRACT'];
const PRIKEY = 		signerData['PRIKEY'];

// setup
const Web3 = require('web3');

// connect
//const web3 = new Web3(ENDPOINT);

async function main() {

	try {



	} catch (error) {
		
		console.log("Error: " + error);
		process.exit();

	}

}
