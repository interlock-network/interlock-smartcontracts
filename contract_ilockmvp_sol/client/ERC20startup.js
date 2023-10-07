// SPDX-License-Identifier: MIT
//
// Interlock ERC-20 ILOCK Token Mint Platform
// 	   testing startup script
//
// Contributors:
// blairmunroakusa
// ...
//

const fs		=	require('fs');
const Web3 		=	require('web3');
const web3 		=	new Web3('http://localhost:8545');

const ERC20startup	=	require('./ERC20startup.json');
const gasprice 		=	ERC20startup['gasprice'];
const from 		=	ERC20startup['from'];
const tokens		=	ERC20startup['tokens'];
const payments 		=	ERC20startup['payments'];
const cliffs 		=	ERC20startup['cliffs'];
const members 		=	ERC20startup['members'];
const constructor	= [
				tokens,
				payments,
				cliffs,
				members ];


const bytecode = fs.readFileSync('claimmethod_ERC20ILOCK_sol_ERC20ILOCK.bin').toString();
const abi = JSON.parse(fs.readFileSync('claimmethod_ERC20ILOCK_sol_ERC20ILOCK.abi').toString());



async function startup() {

	try {

		ERC20ILOCK = new web3.eth.Contract(abi);
		await ERC20ILOCK
			.deploy({data: bytecode, arguments: constructor})
			.send({
				from: from,
				gas: 5000000,
				gasPrice:web3.utils.toWei(gasprice, 'ether')})
			.then((newContractInstance) =>
				{ERC20ILOCK.options.address = newContractInstance.options.address});

	} catch(error) {
		console.log('Error: ' + error);
	}

}

startup();
