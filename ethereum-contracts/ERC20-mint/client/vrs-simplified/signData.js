// using ethereumjs-util 7.1.3
const ethUtil = require('ethereumjs-util@7.1.3');

// using ethereumjs-abi 0.6.9
const abi = require('ethereumjs-abi');


// The purpose of this script is to be painfully explicit for the sake
// of showing work, to ask for help.


// generate keys

prikey = ethUtil.keccakFromString('cow', 256);
signingAddress = ethUtil.privateToAddress(prikey);
	// 0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826

// data

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
		wallet: '0xeeBA65D9C7E5832918d1F4277DE0a78b78efEC43',
		share: 1000,
		pool: 5,
	},
};

// create domain struct hash

const encodedDomainType = 'EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)';
const domainTypeHash = ethUtil.keccakFromString(encodedDomainType, 256);

var encTypes = [];
var encValues = [];

    	// add typehash
    	encTypes.push('bytes32');
    	encValues.push(domainTypeHash);

    	// add name
    	encTypes.push('bytes32');
    	encValues.push(ethUtil.keccakFromString(typedData.domain.name, 256));

    	// add version
    	encTypes.push('bytes32');
    	encValues.push(ethUtil.keccakFromString(typedData.domain.version, 256));

    	// add chainId
    	encTypes.push('uint256');
    	encValues.push(typedData.domain.chainId);
	
    	// add chainId
    	encTypes.push('address');
    	encValues.push(typedData.domain.verifyingContract);

	// computer final hash
	domainStructHash = abi.rawEncode(encTypes, encValues);

// create validation struct hash
	
const encodedValidationType = 'Validation(address wallet,uint256 share,uint256 pool)';
const validationTypeHash = ethUtil.keccakFromString(encodedValidationType, 256);

encTypes = [];
encValues = [];

    	// add typehash
    	encTypes.push('bytes32');
    	encValues.push(validationTypeHash);

    	// add wallet address
    	encTypes.push('address');
    	encValues.push(typedData.message.wallet);

    	// add share
    	encTypes.push('uint256');
    	encValues.push(typedData.message.share);

    	// add pool
    	encTypes.push('uint256');
    	encValues.push(typedData.message.pool);

	// computer final hash
	validationStructHash = abi.rawEncode(encTypes, encValues);

// now finally create final signature hash

signatureHash = ethUtil.keccak256(
	Buffer.concat([
		Buffer.from('1901', 'hex'),
			domainStructHash,
			validationStructHash,
		]),
	);

// and finally, sign

signature = ethUtil.ecsign(signatureHash, prikey);

// convert r, s, and signingAddress into hex strings to pass to remix

console.log(signature.v);

var r = ''
function pad2(s) {return s.length < 2 ? "0" + s : s}; 
	for(i = 0; i < signature.r.length; i++) {
		r += pad2(signature.r[i].toString(16)); }
console.log('0x' + r); // r bytes

var s = ''
function pad2(s) {return s.length < 2 ? "0" + s : s}; 
	for(i = 0; i < signature.s.length; i++) {
		s += pad2(signature.s[i].toString(16)); }
console.log('0x' + s); // s bytes

var str = '';
function pad2(s) {return s.length < 2 ? "0" + s : s};
	for(i = 0; i < signingAddress.length; i++) {
		str += pad2(signingAddress[i].toString(16)); }
console.log('0x' + str); // signingAddress bytes


