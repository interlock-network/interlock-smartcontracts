// SPDX-License-Identifier: MIT
//
// Interlock ERC-20 ILOCK Token Mint Platform
// 	   contract manipulation 
//
// Contributors:
// blairmunroakusa
// ...

// using ethereumjs-util 7.1.3
const ethUtil		= 	require('ethereumjs-util');
const fs		=	require('fs');

const {signData}	= 	require('./ERC20signDataEIP712.js');

const memberData	= 	require('./memberData.json');
const WALLET		=	memberData['WALLET'];
const SHARE		=	memberData['SHARE'];
const POOL		=	memberData['POOL'];

const signerData	=	require('./signerData.json');
const CONTRACT		= 	signerData['CONTRACT'];
const PRIKEY		= 	ethUtil.keccakFromString('cow', 256); //signerData['PRIKEY'];
const ADDRESS		=	ethUtil.privateToAddress(PRIKEY);

const Web3		=	require('web3');
const ENDPOINT		=	'http://localhost:8545';

const web3 = new Web3(ENDPOINT);

const abi = JSON.parse(fs.readFileSync('claimmethod_ERC20ILOCK_sol_ERC20ILOCK.abi').toString());
const ERC20ILOCK = new web3.eth.Contract(abi);
ERC20ILOCK.options.address = CONTRACT;


async function validate() {

	try {

		var validation = {
			'wallet': WALLET,
			//'wallet': ADDRESS,
			'share': SHARE,
			'pool': POOL
		};

	//	validation = [WALLET, SHARE, POOL];

		var signature = signData(
			PRIKEY,
			CONTRACT,
			WALLET,
			//ADDRESS,
			SHARE,
			POOL
		);

		var str = '';
		function pad2(s) {return s.length < 2 ? "0" + s : s}; // helper: pad to 2 digits
		for(i = 0; i < ADDRESS.length; i++) {
			str += pad2(ADDRESS[i].toString(16)); }
		console.log('0x' + str);
		console.log(ADDRESS);
		//
		var r = ''
		function pad2(s) {return s.length < 2 ? "0" + s : s}; // helper: pad to 2 digits
		for(i = 0; i < signature.r.length; i++) {
			r += pad2(signature.r[i].toString(16)); }
		console.log('0x' + r);
		var s = ''
		function pad2(s) {return s.length < 2 ? "0" + s : s}; // helper: pad to 2 digits
		for(i = 0; i < signature.s.length; i++) {
			s += pad2(signature.s[i].toString(16)); }
		console.log('0x' + s);
		//var signature = [signature.r, signature.s, signature.v]
		vbuf = Buffer.from([signature.v]);
		arr = [signature.r, signature.s, vbuf];
		buf = Buffer.concat(arr);
		bytes = Buffer.from(buf).toString("hex");
		bytes = '0x' + bytes;

		console.log(str);

		console.log(signature.v);
		console.log(signature.r);
		console.log(signature.s);

		console.log(validation);

		await ERC20ILOCK
			.methods.setValidationKey('0x' + str)
			.send({
				from: WALLET,
				gas: 5000000,
				gasPrice: web3.utils.toWei('0.00000005','ether') })
			.then(function(result) { console.log(result) });

		//signature = signature['r'].concat(signature['s'], signature['v']);
		console.log(signature.r);

		await ERC20ILOCK
			.methods.validate(validation, signature.v, signature.r, signature.s)
			.send({
				from: WALLET,
				gas: 5000000,
				gasPrice: web3.utils.toWei('0.00000005','ether') })
			.then(function(result) { console.log(result) });

		console.log('chirp');
/*
		let Validate = await ERC20ILOCK.methods.validate(validation, signature).encodeABI();
		var ValidateTX = {
			to: CONTRACT,
			from: ADDRESS,
			gas: 5000000,
			data: Validate,
		};
		signedTX = await web3.eth.accounts.signTransaction(ValidateTX, PRIKEY);
		await web3.eth.sendSignedTransaction(signedTX.rawTransaction, function(error, hash) {
			if (!error) {console.log("Validate TX hash: ", hash);
			} else { console.log("Something went wrong while submitting your transaction:", error) }
		});
*/



	} catch (error) {
		
		console.log("Error: " + error);
		process.exit();

	}

}

validate();
