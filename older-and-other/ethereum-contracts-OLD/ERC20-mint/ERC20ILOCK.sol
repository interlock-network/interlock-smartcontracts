/***************************************************************************/
/***************************************************************************/
/***************************************************************************/
// SPDX-License-Identifier: MIT
//
// Interlock ERC-20 ILOCK Token Mint Platform
//
// Contributors:
// blairmunroakusa
// ...
/***************************************************************************/
/***************************************************************************/
/***************************************************************************/

 /** derived from from oz:
 * functions should revert instead returning `false` on failure.
 * This behavior is nonetheless conventional and does not conflict
 * with the expectations of ERC20 applications.
 *
 * An {Approval} event is emitted on calls to {transferFrom}.
 * This allows applications to reconstruct the allowance for all accounts just
 * by listening to said events.
 **/

pragma solidity ^0.8.0;

import "./IERC20.sol";
import "./POOL.sol";
import "./Messenger.sol";

contract ERC20ILOCK is IERC20 {

/***************************************************************************/
/***************************************************************************/
/***************************************************************************/
	/**
	* declarations
	**/
/***************************************************************************/
/***************************************************************************/
/***************************************************************************/

	/** @dev **/

		// divisibility factor
	uint8 private _decimals = 18;
	uint256 private _DECIMAL = 10 ** _decimals;

		// pools
	string[12] public poolNames = [
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
		uint8 vests;
		uint8 cliff;
		uint32 members; }
	PoolData[] public pool;
	address[] public pools;

	address public tokenlockPool;

		// keeping track of members
	struct MemberStatus {
		uint256 owes;
		uint256 paid;
		uint256 share;
		address account;
		uint8 cliff;
		uint8 pool;
		uint8 payouts; }
	mapping(address => MemberStatus) private _members;

		// core token balance and allowance mappings
	mapping(address => uint256) private _balances;
	mapping(address => mapping(address => uint256)) private _allowances;


		// basic token data
	string private _name = "Interlock Network";
	string private _symbol = "ILOCK";
	uint256 private _totalSupply = 1000000000 * _DECIMAL;
	address private _owner;

		// tracking time
	uint256 public nextPayout;
	uint8 public monthsPassed; 

		// keeping track of irreversible actions
	bool public TGEtriggered = false;
	bool public supplySplit = false;

		// relevant token contract addresses, and other
	IERC20 USDT = IERC20(0xdAC17F958D2ee523a2206206994597C13D831ec7); // USD tether
	IERC20 WETH = IERC20(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2); // wrapped ETH

		// these are prices at TGE, meant to verify investors have contributed what they owe
	uint256 public priceUSDT;
	uint256 public priceWETH;
	uint256 public priceETH;


	event MoreDepositNeeded(
		address depositor,
		uint256 owed );

	event SentTokens(
		address from,
		bytes32 pubkeyTo,
		uint256 amount );

	event ReceivedTokens(
		bytes32 pubkeyFrom,
		address to,
		uint256 amount );

	
/***************************************************************************/
/***************************************************************************/
/***************************************************************************/
	/**
	* init
	**/
/***************************************************************************/
/***************************************************************************/
/***************************************************************************/

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
			pool.push(
				PoolData(
					poolNames[i],
					poolTokens_[i],
					monthlyPayments_[i],
					poolCliffs_[i],
					poolMembers_[i] ) );

		// establish pool to lock bridged tokens in
		tokenlockPool = address(new POOL());
		_balances[tokenlockPool] = 0;
		
		}
	}

/***************************************************************************/
/***************************************************************************/
/***************************************************************************/
	/**
	* modifiers
	**/
/***************************************************************************/
/***************************************************************************/
/***************************************************************************/

		// only allows owner to call
	modifier isOwner(
	) {
		require(
			msg.sender == _owner,
			"only owner can call"
		);
		_;
	}

/*************************************************/

		// verifies zero address was not provied
	modifier noZero(
		address _address
	) {
		require(
			_address != address(0),
			"zero address where it shouldn't be"
		);
		_;
	}

/*************************************************/

		// verifies there exists enough token to proceed
	modifier isEnough(
		uint256 _available,
		uint256 _amount
	) {
		require(
            		_available >= _amount,
			"not enough tokens available"
		);
		_;
	}

/***************************************************************************/
/***************************************************************************/
/***************************************************************************/
	/**
	* setup methods
	**/
/***************************************************************************/
/***************************************************************************/
/***************************************************************************/

		// creates account for each pool
	function splitSupply(
	) public isOwner {
		
		// guard
		require(
			supplySplit == false,
			"supply split already happened");
		// create pool accounts and initiate
		for (uint8 i = 0; i < _poolNumber; i++) {
			address Pool = address(new POOL());
			pools.push(Pool);
			_balances[Pool] = 0;
		}
		// this must never happen again...
		supplySplit = true;
	}

/*************************************************/

		// generates all the tokens
	function triggerTGE(
	) public isOwner {

		// guards
		require(
			supplySplit == true,
			"supply not split");
		require(
			TGEtriggered == false,
			"TGE already happened");
		// mint
		_balances[address(this)] = _totalSupply;
		_approve(
			address(this),
			msg.sender,
			_totalSupply);
		emit Transfer(
			address(0),
			address(this),
			_totalSupply);
		// start the clock for time vault pools
		nextPayout = block.timestamp + 30 days;
		monthsPassed = 0;
		// apply the initial round of token distributions
		_poolDistribution();
		// this must never happen again...
		TGEtriggered = true;
	}

/***************************************************************************/
/***************************************************************************/
/***************************************************************************/
	/**
	* payout methods
	**/
/***************************************************************************/
/***************************************************************************/
/***************************************************************************/						
			
		// distribute tokens to pools on schedule
	function _poolDistribution(
	) public {

		// iterate through pools
		for (uint8 i = 0; i < _poolNumber; i++) {
			if (pool[i].cliff <= monthsPassed &&
				monthsPassed < (_members[pools[i]].cliff + pool[i].vests)) {
				// transfer month's distribution to pools
				transferFrom(
					address(this),
					pools[i],
					pool[i].tokens/pool[i].vests );
				_approve(
					pools[i],
					msg.sender,
					pool[i].tokens/pool[i].vests);
			}
		}
	}

/*************************************************/

		// makes sure that distributions do not happen too early
	function _checkTime(
	) internal returns (bool) {

		// test time
		if (block.timestamp > nextPayout) {
			nextPayout += 30 days;
			monthsPassed++;
			_poolDistribution;
			return true;
		}

		// not ready
		return false;
	}
			
/*************************************************/

		// renders contract as ownerLESS
	function disown(
	) public isOwner {

		//disown
		_owner = address(0);
	}

/*************************************************/

		// changes the contract owner
	function changeOwner(
		address newOwner
	) public isOwner {

		// reassign
		_owner = newOwner;
	}

/***************************************************************************/
/***************************************************************************/
/***************************************************************************/
	/**
	* merkle distributor member validation methods
	**/
/***************************************************************************/
/***************************************************************************/
/***************************************************************************/


	bytes32 public merkleRoot;

    	// This is a packed array of booleans.
    mapping(uint256 => uint256) public claimedBitMap;

/*************************************************/

		// sets serverside Merkle root
	function setMerkleRoot(
		bytes32 newRoot
	) public isOwner {

		merkleRoot = newRoot;
	}

/*************************************************/

		 // returning boolean to indicate whether or member has alreaduy claimed stake
		// searches claimedBitMap for bitflag representing claimed boolean
   	function isClaimed(
		uint256 index
	) public view returns (bool) {

        	uint256 claimedWordIndex = index / 256;
        	uint256 claimedBitIndex = index % 256;
        	uint256 claimedWord = claimedBitMap[claimedWordIndex];
        	uint256 mask = (1 << claimedBitIndex);
        	return claimedWord & mask == mask;
    	}

/*************************************************/

		// flip bit corresponding to index to indicate member has claimed stake
    	function _setClaimed(
		uint256 index
	) private {
        	uint256 claimedWordIndex = index / 256;
        	uint256 claimedBitIndex = index % 256;
        	claimedBitMap[claimedWordIndex] = claimedBitMap[claimedWordIndex] | (1 << claimedBitIndex);
    	}

/*************************************************/

		// member claims stake to tokens and transfers month's batch to member
	function claimWallet(
		uint256 index,
		address account,
		uint256 share,
		uint256 owes,
		uint256 poolnumber,
		bytes32[] calldata merkleProof
	) public {

		// see if we need to update time
		_checkTime();

        	require(
			!isClaimed(index),
			"MerkleDistributor: stake already claimed");

        	// verify the merkle proof
        	bytes32 node = keccak256(abi.encodePacked(index, account, share, owes, poolnumber));
        	require(
			_verify(merkleProof, merkleRoot, node),
			"MerkleDistributor: invalid proof");

        	// mark it claimed
        	_setClaimed(index);

		// setup member entry
		_members[account].share = share;
		_members[account].pool = uint8(poolnumber);
		_members[account].cliff = pool[poolnumber].cliff;
		_members[account].paid = 0;
		_members[account].payouts = 0;
		_members[account].owes = owes;

	
        	emit Claimed(
			index,
			account,
			share,
			poolnumber);
    	}

/*************************************************/

		// claim stake for vest periods accumulated
	function claimStake(
	) public returns (bool) {

		// member must have claimed wallet
		require(
			_members[msg.sender].share != 0,
			"member has not claimed wallet, or claim is fully collected");

		// see if we need to update time
		_checkTime();

		// make sure if investor, they have paid in
		require(
			_members[msg.sender].owes == 0,
			"Investor has not paid in yet."
		);

		// number of payouts must not surpass number of vests
		require(
			_members[msg.sender].payouts < pool[_members[msg.sender].pool].vests,
			"member already collected entire token share");

		// make sure cliff has been surpassed
		require(
			monthsPassed >= pool[_members[msg.sender].pool].cliff,
			"too soon -- cliff not yet passed");

		
		// determine the number of payments claimant has rights to
		uint8 payments;
		// when time has past vesting period, pay out remaining unclaimed payments
		if (pool[_members[msg.sender].pool].cliff +
		    pool[_members[msg.sender].pool].vests <= monthsPassed) {
			
			payments = pool[_members[msg.sender].pool].vests -
				   _members[msg.sender].payouts;

		// don't count months past vests+cliff as payments
		} else {

			payments = 1 + monthsPassed -
				   _members[msg.sender].payouts -
				   pool[_members[msg.sender].pool].cliff;
		}
				
		// use payments to calculate amount to pay out
		uint256 payout = _members[msg.sender].share /
				 pool[_members[msg.sender].pool].vests * payments;

		// if at final payment, add remainder of share to final payment
		if (_members[msg.sender].share -
			_members[msg.sender].paid - payout <
			_members[msg.sender].share / pool[_members[msg.sender].pool].vests) {
			payout += _members[msg.sender].share %
				 	  pool[_members[msg.sender].pool].vests;
		}

		// transfer and make sure it succeeds
		require(
			_transfer(pools[_members[msg.sender].pool], msg.sender, payout),
			"stake claim transfer failed");

		// update member state
		_members[msg.sender].payouts += payments;
		_members[msg.sender].paid += payout;
		
		return true;
	}	

/*************************************************/

     		   // sibling hashes on the branch from the leaf to the root of the tree
		  // each pair of pre-images are assumed to be sorted
		 // a `proof` must be provided, containing pair of leaves 
		// returns true if a `leaf` can be proved to be a part of a Merkle tree
    	function _verify(
        	bytes32[] memory proof,
        	bytes32 root,
        	bytes32 leaf
    	) private pure returns (bool) {

        	return processProof(proof, leaf) == root;
    	}

/*************************************************/

		 // a `proof` is valid if and only if the rebuilt hash matches the root of the tree
		// returns the rebuilt hash obtained by traversing a Merkle tree up
    	function processProof(
		bytes32[] memory proof,
		bytes32 leaf
	) private pure returns (bytes32) {

        	bytes32 computedHash = leaf;
        	for (uint256 i = 0; i < proof.length; i++) {
            		bytes32 proofElement = proof[i];
            		if (computedHash <= proofElement) {
                		// Hash(current computed hash + current element of the proof)
                		computedHash = _efficientHash(computedHash, proofElement);
            		} else {
                		// Hash(current element of the proof + current computed hash)
                		computedHash = _efficientHash(proofElement, computedHash);
            		}
        	}
        	return computedHash;
    	}

/*************************************************/

		// takes hash of two elements
    	function _efficientHash(
		bytes32 a,
		bytes32 b
	) private pure returns (bytes32 value) {

        	assembly {
            		mstore(0x00, a)
            		mstore(0x20, b)
            		value := keccak256(0x00, 0x40)
        	}
    	}

/***************************************************************************/
/***************************************************************************/
/***************************************************************************/
	/**
	* ERC20 getter methods
	**/
/***************************************************************************/
/***************************************************************************/
/***************************************************************************/

		// gets token name (Interlock Network)
	function name(
	) public view override returns (string memory) {

		return _name;
	}

/*************************************************/

		// gets token symbol (ILOCK)
	function symbol(
	) public view override returns (string memory) {

		return _symbol;
	}

/*************************************************/

		// gets token decimal number
	function decimals(
	) public view override returns (uint8) {

		return _decimals;
	}

/*************************************************/

		// gets tokens minted
	function totalSupply(
	) public view override returns (uint256) {

		return _totalSupply;
	}

/*************************************************/

		// gets account balance (tokens payable)
	function balanceOf(
		address account
	) public view override returns (uint256) {

		return _balances[account];
	}

/*************************************************/

		// gets tokens spendable by spender from owner
	function allowance(
		address owner,
		address spender
	) public view virtual override returns (uint256) {
		return _allowances[owner][spender]; }

/*************************************************/

		// gets total tokens paid out in circulation
	function circulation(
	) public view returns (uint256) {

		return _totalSupply - _balances[address(this)];
	}

/***************************************************************************/
/***************************************************************************/
/***************************************************************************/
	/**
	* ERC20 doer methods
	**/
/***************************************************************************/
/***************************************************************************/
/***************************************************************************/

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

		// internal implementation of transfer() above
	function _transfer(
		address from,
		address to,
		uint256 amount
	) internal virtual noZero(from) noZero(to) isEnough(_balances[from], amount) returns (bool) {
		_beforeTokenTransfer(from, to, amount);
		unchecked {
			_balances[from] = _balances[from] - amount;}
		_balances[to] += amount;
		emit Transfer(from, to, amount);
		_afterTokenTransfer(from, to, amount);

        return true;
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
		emit Approval(owner, spender, amount); }

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

/*************************************************

NOTE REGARDING FRONTRUNNING DOUBLE WITHDRAWAL ATTACK:

THIS ATTACK CAN ONLY BE MITIGATED CLIENT-SIDE, BECAUSE IT IS LITERALLY
IMPOSSIBLE FOR A CONTRACT TO DISCERN BETWEEN AN HONEST WITHDRAWAL, AND
A WITHDRAWAL IN BAD FAITH. (work it out, it is impossible. and in fact, 
trying to mitigate against this attack on contract-side makes it possible for
honest token holders to get screwed over if Alice coincidentally withdraws after Bob
has changed his mind about her allowance, but before Bob gets the chance to implement
that change...)

SETTING ALLOWANCE TO ZERO FIRST IS SILLY, BECAUSE YOU CAN STILL FRONTRUN THAT
TRANSACTION, AND SAID TRANSACTION IS INDISTINGUISHABLE FROM AN HONEST TRANSACTION.

/*************************************************/

		   // emitting Transfer, reverting on failure
		  // where `account` must have >= burn amount
		 // where `account` cannot = zero address
		// decreases token supply by deassigning from account
	function _burn(
		address account,
		uint256 amount
	) internal noZero(account) isEnough(_balances[account], amount) {
		_beforeTokenTransfer(
			account,
			address(0),
			amount);
		unchecked {
			_balances[account] = _balances[account] - amount;
		}
		_totalSupply -= amount;
		emit Transfer(
 			account,
			address(0),
			amount);
		_afterTokenTransfer(
			account,
			address(0),
			amount);
	}

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

/***************************************************************************/
/***************************************************************************/
/***************************************************************************/
	/**
	* methods to accept currencies for tokens
	**/
/***************************************************************************/
/***************************************************************************/
/***************************************************************************/

	function getWETHowed(
			address investor) public view returns (uint256) {
		return _members[investor].owes / priceWETH;
	}

	function depositWETH(
		uint256 amount
	) public returns (bool) {

		require(
			_members[msg.sender].owes != 0,
			"Already paid dues."
		);

		// this assumes depositor has already manually approved WETH from wallet
		// get WETH from caller
		WETH.transferFrom(msg.sender, address(this), amount);

		// check to see if deposit is enough and adjust accordingly
		if (amount < _members[msg.sender].owes / priceWETH) {
			_members[msg.sender].owes -= amount;
			emit MoreDepositNeeded(msg.sender, _members[msg.sender].owes);
		} else {
			_members[msg.sender].owes = 0;
		}

		return true;
	}

/*************************************************/

	function getUSDTowed(address investor) public view returns (uint256) {
		return _members[investor].owes / priceUSDT;
	}

	function depositUSDT(
		uint256 amount
	) public returns (bool) {
	
		require(
			_members[msg.sender].owes != 0,
			"Already paid dues."
		);

		// this assumes depositor has already manually approved USDT from wallet
		// get USDT from caller
		USDT.transferFrom(msg.sender, address(this), amount);

		// check to see if deposit is enough and adjust accordingly
		if (amount < _members[msg.sender].owes / priceUSDT) {
			_members[msg.sender].owes -= amount;
			emit MoreDepositNeeded(msg.sender, _members[msg.sender].owes);
		} else {
			_members[msg.sender].owes = 0;
		}
		

		return true;

	}

/*************************************************/

	function getETHowed(address investor) public view returns (uint256) {
		return _members[investor].owes / priceETH;
	}

	function depositETH(
	) public payable returns (bool) {
		
		require(
			_members[msg.sender].owes != 0,
			"Already paid dues."
		);

		// check to see if deposit is enough and adjust accordingly
		if (msg.value < _members[msg.sender].owes / priceETH) {
			_members[msg.sender].owes -= msg.value * priceETH;
			emit MoreDepositNeeded(msg.sender, _members[msg.sender].owes);
		} else {
			_members[msg.sender].owes = 0;
		}

		return true;

	}

/***************************************************************************/
/***************************************************************************/
/***************************************************************************/
	/**
	* vesting and staking
	**/
/***************************************************************************/
/***************************************************************************/
/***************************************************************************/

		    // get how much of amount left to pay is available to claim
		   // get amount left to pay
		  // get amount paid so far to member
		 // get amount investor still needs to pay in before claiming tokens
		// get time remaining until next payout ready
	function vestingStatus(
		address vestee
	) public view returns (
		uint256 timeLeft,
		uint256 stillOwes,
		uint256 paidOut,
		uint256 payRemaining,
		uint256 payAvailable
	) {

		// compute the time left until the next payment is available
		// if months passed beyond last payment, stop counting
		if (monthsPassed >= pool[_members[vestee].pool].vests +
				    pool[_members[vestee].pool].cliff) {
			
			timeLeft = 0;

		// when cliff hasn't been surpassed, include that time into countdown
		} else if (monthsPassed < pool[_members[vestee].pool].cliff) {
			
			timeLeft = (pool[_members[vestee].pool].cliff -
				    monthsPassed - 1) * 30 days +
				    nextPayout - block.timestamp;

		// during vesting period, timeleft is only time til next month's payment
		} else {

			timeLeft = nextPayout - block.timestamp;
		}

		// how much does investor still owe before claiming share
		stillOwes = _members[vestee].owes;

		// how much has member already claimed
		paidOut = _members[vestee].paid;

		// how much does member have yet to collect, after vesting complete
		payRemaining = _members[vestee].share - _members[vestee].paid;

		// computer the pay available to claim at current moment
		// if months passed are inbetween cliff and end of vesting period
		if (monthsPassed >= pool[_members[vestee].pool].cliff &&
		    monthsPassed < pool[_members[vestee].pool].cliff +
				   pool[_members[vestee].pool].vests) {
			
			payAvailable = (1 + monthsPassed -
					pool[_members[vestee].pool].cliff -
					_members[vestee].payouts) *
			      	       (_members[vestee].share / pool[_members[vestee].pool].vests);

		// until time reaches cliff, no pay is available
		} else if (monthsPassed < pool[_members[vestee].pool].cliff ){

			payAvailable = 0;

		// if time has passed cliff and vesting period, the entire remaining share is available
		} else {

			payAvailable = _members[vestee].share - _members[vestee].paid;
		}

		// if at final payment, add remainder of share to final payment
		if (_members[msg.sender].share - _members[msg.sender].paid - payAvailable <
			_members[msg.sender].share / pool[_members[msg.sender].pool].vests &&
			payAvailable > 0) {
			
			payAvailable += _members[msg.sender].share %
				 	pool[_members[msg.sender].pool].vests;
		}

		return (
			timeLeft,
			stillOwes,
			paidOut,
			payRemaining,
			payAvailable
		);
	}

/***************************************************************************/
/***************************************************************************/
/***************************************************************************/
	/***
	* wormhole
	**/
/***************************************************************************/
/***************************************************************************/
/***************************************************************************/

	Messenger wormhole = new Messenger();

/*************************************************/

		 // locks tokens in tokenlockPool account
		// sends message to wormhole guardians/bridge
	function sendToken(
		uint256 amount,
		bytes32 pubkey
	) public returns (uint64 sequence) {

		// condition amount to truncate and floor at 128 bits
		// 
		amount = uint128(amount);
	
		// condition message here
		// (68 byte message = 16B amount + 20B sender address + 32B sender pubkey)
		bytes memory message = abi.encode(amount, msg.sender, pubkey);
		
		// send message to transfer token
		sequence = 0;
		sequence = wormhole.sendMsg(message);
		
		// make sure message was properly sent
		require(
			sequence != 0,
			"token bridge transfer failed"
		);
		
		// now move token amount from member balance to locked-pool
		_balances[msg.sender] -= amount
		_balances[tokenlockPool] += amount;

		emit SentTokens(msg.sender, pubkey, amount);

	}

/*************************************************/

	function receiveToken(
		bytes memory encodedMessage
	) public returns (string message) {
	
		// verify message is legit and decode
		message = wormhole.receiveEncodedMsg(encodedMessage);

		// extract message contents from message string
		uint128 amount = uint128(message[0:15]);
		address account = address(message[16:35]);
		bytes32 pubkey = bytes32(message[36:77]);

		// adjust account balances
		_balances[msg.sender] += amount;
		_balances[tokenlockPool] -= amount;

		// make sure token transfer is authorized by function caller
		require(
			account == msg.sender,
			"recipient must be sender"
		);

		emit ReceivedTokens(pubkey, msg.sender, amount);
	}

/*************************************************/

}

/***************************************************************************/
/***************************************************************************/
/***************************************************************************/





