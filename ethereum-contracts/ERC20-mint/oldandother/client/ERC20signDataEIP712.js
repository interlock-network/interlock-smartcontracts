// SPDX-License-Identifier: MIT
//
// Interlock ERC-20 INTR Token Mint Platform
// 	   EIP712 data signer 
//
// Contributors:
// blairmunroakusa
// ...


// using ethereumjs-util 7.1.3
const ethUtil = require('ethereumjs-util');

// using ethereumjs-abi 0.6.9
const abi = require('ethereumjs-abi');

// using chai 4.3.4
const chai = require('chai');

	// imports data parameters and signs
function signData(
	privateKey,
	contractAddress,
	memberWallet,
	share,
	pool,) {

	const typedData = {
    		types: {
        		EIP712Domain: [
            			{ name: 'name', type: 'string' },
	            		{ name: 'version', type: 'string' },
        	    		{ name: 'chainId', type: 'uint256' },
            			{ name: 'verifyingContract', type: 'address' },
	        	],
			Validation: [
            			{ name: 'wallet', type: 'address' },
            			{ name: 'share', type: 'uint256' },
	            		{ name: 'pool', type: 'uint8' }
        		],
	    	},
	    	primaryType: 'Validation',
    		domain: {
        		name: 'Validator',
	        	version: '1',
        		chainId: 1,
        		verifyingContract: '0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC',
	    	},
		message: {
			wallet: memberWallet,
			share: share,
			pool: pool,
		} };

	const types = typedData.types;

		// Recursively finds all the dependencies of a type
	function dependencies(primaryType, found = []) {
		if (found.includes(primaryType)) {
        		return found;
	    	}
    		if (types[primaryType] === undefined) {
        		return found;
	    	}
    		found.push(primaryType);
	    	for (let field of types[primaryType]) {
        		for (let dep of dependencies(field.type, found)) {
            			if (!found.includes(dep)) {
                			found.push(dep);
	            		}
        		}
 	   	}
    		return found;
	}

		// encodes the struct types
	function encodeType(primaryType) {
    		
		// Get dependencies primary first, then alphabetical
		let deps = dependencies(primaryType);
		deps = deps.filter(t => t != primaryType);
		deps = [primaryType].concat(deps.sort());
	
		// Format as a string with fields
	    	let result = '';
    		for (let type of deps) {
	        	result += `${type}(${types[type]?.map(({ name, type }) => `${type} ${name}`).join(',')})`;
    		}
	    	return result;
	}

		// takes hash of struct types
	function typeHash(primaryType) {
    		return ethUtil.keccakFromString(encodeType(primaryType), 256);
	}
		// encodes struct data
	function encodeData(primaryType, data) {
    		let encTypes = [];
	    	let encValues = [];

    		// Add typehash
	    	encTypes.push('bytes32');
    		encValues.push(typeHash(primaryType));

	    	// Add field contents
	    	for (let field of types[primaryType]) {
        		let value = data[field.name];
        		if (field.type == 'string' || field.type == 'bytes') {
	            		encTypes.push('bytes32');
        	    		value = ethUtil.keccakFromString(value, 256);
            			encValues.push(value);
	        	} else if (types[field.type] !== undefined) {
        	    		encTypes.push('bytes32');
            			value = ethUtil.keccak256(encodeData(field.type, value));
            			encValues.push(value);
	       		} else if (field.type.lastIndexOf(']') === field.type.length - 1) {
        	    		throw 'TODO: Arrays currently unimplemented in encodeData';
        		} else {
	            		encTypes.push(field.type);
        	    		encValues.push(value);
	        	}
    		}

    		return abi.rawEncode(encTypes, encValues);
	}

		// creates hash derived from primaryType struct
	function structHash(primaryType, data) {
    		return ethUtil.keccak256(encodeData(primaryType, data));
	}

		// creates the final hash to be signed
	function signHash() {
    		return ethUtil.keccak256(
        		Buffer.concat([
	            	Buffer.from('1901', 'hex'),
        	    	structHash('EIP712Domain', typedData.domain),
            		structHash(typedData.primaryType, typedData.message),
	        	]),
    		);
	}

	//const privateKey = ethUtil.keccakFromString('cow', 256);
	//const address = ethUtil.privateToAddress(privateKey);

	const sig = ethUtil.ecsign(signHash(), privateKey);


	// display results for dev purposes
		//
	console.log(`encode domain typehash ${encodeType('EIP712Domain')}`);
	console.log(`domain type hash = ${ethUtil.bufferToHex(typeHash('EIP712Domain'))}`);

		//
	console.log(`r = ${ethUtil.bufferToHex(sig.r)}`);
	console.log(`s = ${ethUtil.bufferToHex(sig.s)}`);
	console.log(`v = ${sig.v}`);
	console.log(``);

	console.log(`encode validation typehash = ${encodeType('Validation')}`);
	console.log(`validation type hash = ${ethUtil.bufferToHex(typeHash('Validation'))}`);
	console.log(``);
	console.log(`encoded data = ${ethUtil.bufferToHex(encodeData(typedData.primaryType, typedData.message))}`);
	console.log(``);
	console.log(`struct hash = ${ethUtil.bufferToHex(structHash(typedData.primaryType, typedData.message))}`);
	console.log(`domain hash = ${ethUtil.bufferToHex(structHash('EIP712Domain', typedData.domain))}`);
	console.log(`sign hash = ${ethUtil.bufferToHex(signHash())}`);

	console.log(sig);
	return sig;

}

module.exports = {signData};



