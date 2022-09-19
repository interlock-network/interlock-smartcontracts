// SPDX-License-Identifier: MIT
//
// Interlock ERC-20 ILOCK Token Mint Platform
// 		(containing)
// components from OpenZeppelin v4.6.0 contract (token/ERC20/ERC20.sol)
//
// Contributors:
// blairmunroakusa
// ...

pragma solidity ^0.8.0;

import "./IERC20.sol";
import "./IWORMHOLE.sol";
import "./POOL.sol";
import "./utils/ECDSA.sol";

 /** derived from from oz:
 * functions should revert instead returning `false` on failure.
 * This behavior is nonetheless conventional and does not conflict
 * with the expectations of ERC20 applications.
 *
 * An {Approval} event is emitted on calls to {transferFrom}.
 * This allows applications to reconstruct the allowance for all accounts just
 * by listening to said events.
 *
 * Finally, the non-standard {decreaseAllowance} and {increaseAllowance}
 * functions have been added to mitigate the well-known issues around setting
 * allowances.
 **/

contract ERC20ILOCK is IERC20 {

	/** @dev **/

/*************************************************/
	/**
	* declarations
	**/
/*************************************************/

		// libraries
    using ECDSA for bytes32;

		// divisibility factor
	uint8 private _decimals = 18;
	uint256 private _DECIMAL = 10 ** _decimals;

		// pools
	string[12] private _poolNames = [
		"earlyvc",
		"ps1",
		"ps2",
		"ps3",
		"team",
		"ov",
		"advise",
		"reward",
		"founder",
		"partner",
		"white",
		"public" ];
	uint8 constant private _poolNumber = 12;

		// keeping track of pools
	struct PoolData {
		string name;
		uint256 tokens;
		uint8 payments;
		uint8 cliff;
		uint32 members; }
	PoolData[] private _pool;
	address[] private _pools;

		// keeping track of members
	struct MemberStatus {
		uint256 paid;
		uint256 share;
		address account;
		uint8 cliff;
		uint8 pool;
		uint8 payments; }
	mapping(address => MemberStatus) private _members;

		// EIP712 signature implementation
	address private _validationKey = address(0);
	bytes32 public DOMAIN_SEPARATOR;
	bytes32 constant EIP712DOMAIN_TYPEHASH = keccak256(
		"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract");
	bytes32 constant VALIDATION_TYPEHASH = keccak256(
		"Validation(address wallet,unit256 share,uint8 pool)");

		// domain separator
	struct EIP712Domain {
        string  name;
        string  version;
        uint256 chainId;
        address verifyingContract; }

		// data struct for validation claim
	struct Validation {
		address wallet;
		uint256 share;
		uint8 pool; }

		// core token balance and allowance mappings
	mapping(address => uint256) private _balances;
	mapping(address => mapping(
		address => uint256)) private _allowances;

		// basic token data
	string private _name = "Interlock Network";
	string private _symbol = "ILOCK";
	uint256 private _totalSupply = 1000000000 * _DECIMAL;
	address private _owner;
	// decimals = 18 by default

		// tracking time
	uint256 public nextPayout;
	uint8 public monthsPassed; 

		// keeping track of irreversible actions
	bool public TGEtriggered = false;
	bool public supplySplit = false;
	
/*************************************************/
	/**
	* init
	**/
/*************************************************/

		 // owned by msg.sender
		// initializes contract
	constructor(
		uint256[_poolNumber] memory poolTokens_,
		uint8[_poolNumber] memory monthlyPayments_,
		uint8[_poolNumber] memory poolCliffs_,
		uint32[_poolNumber] memory poolMembers_
	) {
		_owner = msg.sender;
		_balances[address(this)] = 0; 

			// iterate through pools to create struct array
		for (uint8 i = 0; i < _poolNumber; i++) {
			poolTokens_[i] *= _DECIMAL;
			_pool.push(
				PoolData(
					_poolNames[i],
					poolTokens_[i],
					monthlyPayments_[i],
					poolCliffs_[i],
					poolMembers_[i] ) ); }

			// initiate EIP712 standard for member validation
		DOMAIN_SEPARATOR = keccak256(abi.encode(
			EIP712DOMAIN_TYPEHASH,
			keccak256(bytes("Validator")),
			keccak256(bytes("1")),
			1,
			0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC ) ); }

/*************************************************/
	/**
	* modifiers
	**/
/*************************************************/

		// only allows owner to call
	modifier isOwner(
	) {
		require(msg.sender == _owner,
			"only owner can call");
		_; }

/*************************************************/

		// verifies zero address was not provied
	modifier noZero(
		address _address
	) {
		require(_address != address(0),
			"zero address where it shouldn't be");
		_; }

/*************************************************/

		// verifies there exists enough token to proceed
	modifier isEnough(
		uint256 _available,
		uint256 _amount
	) {
		require(_available >= _amount,
			"not enough tokens available");
		_; }

/*************************************************/
	/**
	* setup methods
	**/
/*************************************************/

		// creates account for each pool
	function splitSupply(
	) public isOwner {
		
		// guard
		require(supplySplit == false,
			"supply split already happened");

		// create pool accounts and initiate
		for (uint8 i = 0; i < _poolNumber; i++) {
			address Pool = address(new POOL());
			_pools.push(Pool);
			_balances[Pool] = 0;
			_allowances[address(this)][Pool] = 0; }

		// this must never happen again...
		supplySplit = true; }

/*************************************************/

		// generates all the tokens
	function triggerTGE(
	) public isOwner {

		// guards
		require(supplySplit == true,
			"supply not split");
		require(TGEtriggered == false,
			"TGE already happened");

		// mint
		_balances[address(this)] = _totalSupply;
		_approve(address(this), msg.sender, _totalSupply);
		emit Transfer(address(0), address(this), _totalSupply);

		// start the clock for time vault pools
		nextPayout = block.timestamp + 30 days;
		monthsPassed = 0;

		// apply the initial round of token distributions
		_poolDistribution();

		// this must never happen again...
		TGEtriggered = true; }

/*************************************************/
	/**
	* payout methods
	**/
/*************************************************/							
			
		// distribute tokens to pools on schedule
	function _poolDistribution(
	) internal {

		// iterate through pools
		for (uint8 i = 0; i < _poolNumber; i++) {
			if (_pool[i].cliff <= monthsPassed &&
				monthsPassed >= (_members[_pools[i]].cliff + _members[_pools[i]].payments)
				) {
				// transfer month's distribution to pools
				transferFrom(
					address(this),
					_pools[i],
					_pool[i].tokens/_pool[i].payments );
				_approve(
					_pools[i],
					msg.sender,
					_pool[i].tokens/_pool[i].payments ); } } }

/*************************************************/

		// makes sure that distributions do not happen too early
	function _checkTime(
	) internal returns (bool) {

		// test time
		if (block.timestamp > nextPayout) {
			nextPayout += 30 days;
			monthsPassed++;
			return true; }

		// not ready
		return true; }
			
/*************************************************/

		// renders contract as ownerLESS
	function disown(
	) public isOwner {

		//disown
		_owner = address(0); }

/*************************************************/

		// changes the contract owner
	function changeOwner(
		address newOwner
	) public isOwner {

		// reassign
		_owner = newOwner; }

/*************************************************/
	/**
	* member validation methods
	**/
/*************************************************/

		// sets to serverside whitelist signing key
	function setValidationKey(
		address newKey
	) public isOwner {
		_validationKey = newKey; }

/*************************************************/

		// generate DOMAIN_SEPARATOR part of digest
	function hash(
		EIP712Domain memory eip712Domain
	) internal pure returns (bytes32) {
		return keccak256(abi.encode(
			EIP712DOMAIN_TYPEHASH,
			keccak256(bytes(eip712Domain.name)),
			keccak256(bytes(eip712Domain.version)),
			eip712Domain.chainId,
			eip712Domain.verifyingContract ) ); }

		// generate VALIDATION part of digest
	function hash(
		Validation memory validation
	) internal pure returns (bytes32) {
		return keccak256(abi.encode(
			VALIDATION_TYPEHASH,
			validation.wallet,
			validation.share,
			validation.pool ) ); }

		event debug(address sig, address key);
		   // unpacks validation data and stores new member record
		  // reverts if _validationKey not set
		 // reverts if recovered key != _validationKey
		// checks incoming signature packet

	function validate(
		Validation calldata validation,
		uint8 v,
		bytes32 r,
		bytes32 s
	) public {

		//  guard
		require(_validationKey != address(0),
			"validation key has not been set");



		// recreate digest
		bytes32 digest = keccak256(abi.encodePacked(
			"\x19\x01",
			DOMAIN_SEPARATOR,
			hash(validation) ) );
		emit debug( ecrecover(digest, v, r, s), _validationKey);

		// verify signature
		//require(digest.recover(signature) == _validationKey,

		//:w
	//	"test");
		
		// pack calldata into new member record
		MemberStatus memory member;
		member.pool = validation.pool;
		member.account = validation.wallet;
		member.share = validation.share * _DECIMAL;
		member.cliff = _pool[member.pool].cliff;
		member.payments = _pool[member.pool].payments;
		_members[member.account] = member; }

/*************************************************/
	/**
	* getter methods
	**/
/*************************************************/

		// gets token name (Interlock Network)
	function name(
	) public view override returns (string memory) {
		return _name; }

/*************************************************/

		// gets token symbol (ILOCK)
	function symbol(
	) public view override returns (string memory) {
		return _symbol; }

/*************************************************/

		// gets token decimal number
	function decimals(
	) public view override returns (uint8) {
		return _decimals; }

/*************************************************/

		// gets tokens minted
	function totalSupply(
	) public view override returns (uint256) {
		return _totalSupply; }

/*************************************************/

		// gets account balance (tokens payable)
	function balanceOf(
		address account
	) public view override returns (uint256) {
		return _balances[account]; }

/*************************************************/

		// gets tokens spendable by spender from owner
	function allowance(
		address owner,
		address spender
	) public view virtual override returns (uint256) {
		return _allowances[owner][spender]; }

/*************************************************/

		// gets total tokens paid out in circulation
	function circulation() public view returns (uint256) {
		return _totalSupply - _balances[address(this)]; }

/*************************************************/
	/**
	* doer methods
	**/
/*************************************************/

		   // emitting Transfer, reverting on failure
		  // where caller balanceOf must be >= amount
		 // where `to` cannot = zero  address
		// increases spender allowance
	function transfer(
		address to,
		uint256 amount
	) public override returns (bool) {

		address owner = msg.sender;
		_transfer(owner, to, amount);
		return true;
	}

		// internal implementation of transfer() above
	function _transfer(
		address from,
		address to,
		uint256 amount
	) internal virtual noZero(from) noZero(to) isEnough(_balances[from], amount) {

		_beforeTokenTransfer(from, to, amount);
		unchecked {
			_balances[from] = _balances[from] - amount;}
		_balances[to] += amount;
		emit Transfer(from, to, amount);
		_afterTokenTransfer(from, to, amount);
	}

/*************************************************/

		  // emitting Approval, reverting on failure
		 // (=> no allownance delta when TransferFrom)
		// defines tokens available to spender from msg.sender
	function approve(
		address spender,
		uint256 amount
	) public override returns (bool) {
		address owner = msg.sender;
		_approve(owner, spender, amount);
		return true; }

		// internal implementation of approve() above 
	function _approve(
		address owner,
		address spender,
		uint256 amount
	) internal virtual noZero(owner) noZero(spender) {
		_allowances[owner][spender] = amount;
		emit Approval(owner, spender, amount);
	}

/*************************************************/

		     // emitting Approval, reverting on failure
		    // where msg.sender allowance w/`from` must be >= amount
		   // where `from` balance must be >= amount
		  // where `from` and `to` cannot = zero address
		 // which does not update allowance if allowance = u256.max
		// pays portion of spender's allowance with owner to recipient
	function transferFrom(
		address from,
		address to,
		uint256 amount
	) public override returns (bool) {
		address spender = msg.sender;
		_spendAllowance(from, spender, amount);
		_transfer(from, to, amount);
		return true; }

/*************************************************/

		  // emitting Approval, reverting on failure
		 // where `spender` cannot = zero address
		// atomically increases spender's allowance
	function increaseAllowance(
		address spender,
		uint256 addedValue
	) public returns (bool) {
		address owner = msg.sender;
		_approve(owner, spender, allowance(owner, spender) + addedValue);
		return true; }

 /* Above and below are alternatives to {approve} that can be used
 *  as a mitigation for problems described in {IERC20-approve}.
 *
 * ?? Why is there no owner balance check for increaseAllowance() ??
 **/
		   // emitting Approval, reverting on failure
		  // where `spender` must have allowance >= `subtractedValue`
		 // where `spender` cannot = zero address
		// atomically decreases spender's allowance
	function decreaseAllowance(
		address spender,
		uint256 amount
	) public isEnough(allowance(msg.sender, spender), amount) returns (bool) {
		address owner = msg.sender;
		unchecked {
			_approve(owner, spender, allowance(owner, spender) - amount);}
		return true; }

/*************************************************/

		   // emitting Transfer, reverting on failure
		  // where `account` must have >= burn amount
		 // where `account` cannot = zero address
		// decreases token supply by deassigning from account
	function _burn(
		address account,
		uint256 amount
	) internal noZero(account) isEnough(_balances[account], amount) {
		_beforeTokenTransfer(account, address(0), amount);
		unchecked {
			_balances[account] = _balances[account] - amount;}
		_totalSupply -= amount;
		emit Transfer(account, address(0), amount);
		_afterTokenTransfer(account, address(0), amount); }

/*************************************************/

		   // emitting Approval if finite, reverting on failure 
		  // will do nothing if infinite allowance
		 // used strictly internally
		// deducts from spender's allowance with owner
	function _spendAllowance(
		address owner,
		address spender,
		uint256 amount
	) internal isEnough(allowance(owner, spender), amount) {
		unchecked {
			_approve(owner, spender, allowance(owner, spender) - amount);}}

/*************************************************/

		    // where `from` && `to` != zero account => to be regular xfer
		   // where `from` = zero account => `amount` to be minted `to`
		  // where `to` = zero account => `amount` to be burned `from`
		 // where `from` && `to` = zero account => impossible
		// hook that inserts behavior prior to transfer/mint/burn
	function _beforeTokenTransfer(
		address from,
		address to,
		uint256 amount
	) internal virtual {}

/*************************************************/

		    // where `from` && `to` != zero account => was regular xfer
		   // where `from` = zero account => `amount` was minted `to`
		  // where `to` = zero account => `amount` was burned `from`
		 // where `from` && `to` = zero account => impossible
		// hook that inserts behavior prior to transfer/mint/burn
	function _afterTokenTransfer(
		address from,
		address to,
		uint256 amount
	) internal virtual {}

/*************************************************/

event debughash(bytes32 digest);

function test(bytes calldata signature, Validation calldata validation) public {
	bytes32 digest = keccak256(
		abi.encodePacked(
			"\x19\x01",
			DOMAIN_SEPARATOR,
			keccak256(abi.encode(
				VALIDATION_TYPEHASH,
				validation.wallet,
				validation.share,
				validation.pool
				))
		)
	);
	emit debug(digest.recover(signature), _validationKey);
}

/*************************************************/
	//
	// wormhole
	//
/*************************************************/

    // Hardcode the Wormhole Core Bridge contract address
    // In a real contract, we would set this in a constructor or Setup
    address a = address(0xC89Ce4735882C9F0f0FE26686c53074E09B0D550);
    IWORMHOLE _wormhole = IWORMHOLE(a);

    mapping(bytes32 => bool) _completedMessages;
    mapping(uint16 => bytes32) _bridgeContracts;

    // sendStr sends bytes to the wormhole.
    function sendStr(bytes memory str, uint32 nonce) public returns (uint64 sequence) {
        sequence = _wormhole.publishMessage(nonce, str, 1);
        return sequence;
    }

    function bytes32ToString(bytes32 _bytes32) public pure returns (string memory) {
        uint8 i = 0;
        bytes memory bytesArray = new bytes(64);
        for (i = 0; i < bytesArray.length; i++) {
            bytesArray[i] = toByte((_bytes32[i/2] >> 4) & 0x0f);
            i = i + 1;
            bytesArray[i] = toByte(_bytes32[i/2] & 0x0f);
        }
        return string(bytesArray);
    }

    function toByte(bytes1 _b1) public pure returns (bytes1) {
        uint8 _b  = uint8(_b1);
        return (_b < 10)? bytes1(_b + 48): bytes1(_b + 87);
    }

    // receiveStr confirms VAA and processes message on the receiving chain.
    // Returns true when bytes are seen first time.
    function receiveBytes(bytes memory encodedVm, uint32 /*nonce*/) public {
        (IWORMHOLE.VM memory vm, bool valid, string memory reason) = _wormhole.parseAndVerifyVM(encodedVm);
        // 1. check wormhole signatures/
        require(valid, reason);

        // 2. Check if emtter chain contract is registered.
        // Print incoming emitter address.
        // string memory smsg = string(abi.encodePacked(" invalid em ", Strings.toString(vm.emitterChainId), "-", bytes32ToString(vm.emitterAddress)));
        // Print map entry address.
        // string memory smsg = string(abi.encodePacked(" invalid_emitter ", Strings.toString(vm.emitterChainId), "+", bytes32ToString(_bridgeContracts[vm.emitterChainId])));
        
        require(verifyBridgeVM(vm), "invalid emitter");

        // 3. Drop duplicate VAA.
        require(!_completedMessages[vm.hash], " message already received");
        _completedMessages[vm.hash] = true;

        // Action place..
        // At this point payload is good to be used for what actual contract needs to do. Like transfer(s) etc
    }

    // Check if receiveBytes emmiter is actually registered chan.
    function verifyBridgeVM(IWORMHOLE.VM memory vm) internal view returns (bool){
        return (_bridgeContracts[vm.emitterChainId] == vm.emitterAddress);
    }
    // We register chain,bridge in [mpn run register] command.
    function registerChain(uint16 chainId_, bytes32 bridgeContract_) public isOwner {
        _bridgeContracts[chainId_] = bridgeContract_;
    }

    function wormhole() public view returns (IWORMHOLE) {
        return _wormhole;
    }

}




