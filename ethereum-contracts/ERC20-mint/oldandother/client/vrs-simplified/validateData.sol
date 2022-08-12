pragma solidity ^0.8.0

contract validateData {
    
    	struct EIP712Domain {
        	string  name;
        	string  version;
        	uint256 chainId;
        	address verifyingContract;
    	}

    	struct Validation {
        	wallet address;
        	share uint256;
        	pool uint8;
    	}

    	bytes32 constant EIP712DOMAIN_TYPEHASH = keccak256(
        	"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
    	);

    	bytes32 constant VALIDATION_TYPEHASH = keccak256(
        	"Validation(wallet address,share uint256,pool uint8)"
    	);

    	bytes32 DOMAIN_SEPARATOR;

    	constructor () public {
        	DOMAIN_SEPARATOR = hash(EIP712Domain({
            		name: "Validator",
            		version: '1',
            		chainId: 1,
            		verifyingContract: 0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC
        	}));
    	}

    	function hash(EIP712Domain eip712Domain) internal pure returns (bytes32) {
        	return keccak256(abi.encode(
            		EIP712DOMAIN_TYPEHASH,
            		keccak256(bytes(eip712Domain.name)),
            		keccak256(bytes(eip712Domain.version)),
            		eip712Domain.chainId,
            		eip712Domain.verifyingContract
        	));
    	}

    	function hash(Validation validation) internal pure returns (bytes32) {
        	return keccak256(abi.encode(
            		VALIDATION_TYPEHASH,
            		validation.wallet,
            		validation.share,
			validation.pool
        	));
    	}

    	function verify(Validation validation, uint8 v, bytes32 r, bytes32 s) internal view returns (bool) {
        // Note: we need to use `encodePacked` here instead of `encode`.
        	bytes32 digest = keccak256(abi.encodePacked(
            		"\x19\x01",
            		DOMAIN_SEPARATOR,
            		hash(validation)
        	));
        	emit compare(ecrecover(digest, v, r, s), _validationKey);
    	}

	function addValidationKey(key address) public {
		_validationKey = key;
	}
    
}
