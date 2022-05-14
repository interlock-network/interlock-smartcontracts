// SPDX-License-Identifier: MIT
//
// Interlock ERC-20 INTR Token Mint Platform
// 	   contract manipulation 
//
// Contributors:
// blairmunroakusa
// ...

// using ethereumjs-util 7.1.3
const ethUtil		= 	require('ethereumjs-util');

const signData		= 	require('./ERC20signDataEIP712.js');
const ERC20abi		=	require('./ERC20abi.json');

const memberData	= 	require('./memberData.json');
const WALLET		=	memberData['WALLET'];
const SHARE		=	memberData['SHARE'];
const POOL		=	memberData['POOL'];

const signerData	=	require('./signerData.json');
const CONTRACT		= 	signerData['CONTRACT'];
const PRIKEY		= 	ethUtil.keccakFromString('cow', 256); //signerData['PRIKEY'];
const ADDRESS		=	ethUtil.privateToAddress(PRIKEY);

const web3		=	require('web3');


//const ENDPOINT = ?;

// connect
//const web3 = new Web3(ENDPOINT);

async function validate() {

	try {

		const validation = {
			'wallet': WALLET,
			'share': SHARE,
			'pool': POOL
		};
		const signature = signData(
			PRIKEY,
			CONTRACT,
			WALLET,
			SHARE,
			POOL
		);
		let ERC20 = new web3.eth.Contract(ERC20abi, CONTRACT);
		let Validate = await ERC20.methods.validate(validation, signature);
		var ValidateTX = {
			to: CONTRACT,
			from: ADDRESS,
			gas: 3000000,
			data: Validate,
		};
		signedTX = await web3.eth.accounts.signTransaction(ValidateTX, PRIKEY);
		await web3.eth.sendSignedTransaction(signedTX.rawTransaction, function(error, hash) {
			if (!error) {console.log("Validate TX hash: ", hash);
			} else { console.log("Something went wrong while submitting your transaction:", error) }
		});




	} catch (error) {
		
		console.log("Error: " + error);
		process.exit();

	}

}
