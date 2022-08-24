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
 *
 * Finally, the non-standard {decreaseAllowance} and {increaseAllowance}
 * functions have been added to mitigate the well-known issues around setting
 * allowances.
 **/

pragma solidity ^0.8.0;

import "./IERC20.sol";
import "./POOL.sol";

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

		// keeping track of members
	struct MemberStatus {
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

		// (completely (almost)) eliminating double withdrawal vulnerability
	mapping(address => bool) public owes;
	mapping(address => mapping(address => uint256)) private _allowanceTotalIncrease;
	mapping(address => mapping(address => uint256)) private _transferFromTotal;
	struct Credits {
		address creditor;
		uint256 owed; }
	Credits[] private _credits;
	mapping(address => _credits) private _creditorsOwed;

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
	) internal {

		// iterate through pools
		for (uint8 i = 0; i < _poolNumber; i++) {
			if (pool[i].cliff <= monthsPassed &&
				monthsPassed <= (_members[pools[i]].cliff + pool[i].vests)) {
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
		uint256 poolnumber,
		bytes32[] calldata merkleProof
	) public {

		// see if we need to update time
		_checkTime();

        	require(
			!isClaimed(index),
			"MerkleDistributor: stake already claimed");

        	// verify the merkle proof
        	bytes32 node = keccak256(abi.encodePacked(index, account, share, poolnumber));
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
	
        	emit Claimed(
			index,
			account,
			share,
			poolnumber);
    	}

/*************************************************/

		// claim stake for vest period
	function claimStake(
	) public returns (bool) {

		// member must have claimed wallet
		require(
			_members[msg.sender].share != 0,
			"member has not claimed wallet, or claim is fully collected");

		// see if we need to update time
		_checkTime();

		// transfer tokens to member from pool
		// number of payouts must not surpass number of vests
		require(
			_members[msg.sender].payouts < monthsPassed - pool[_members[msg.sender].pool].cliff,
			"member already collected entire token share");

		require(
			monthsPassed > pool[_members[msg.sender].pool].cliff,
			"too soon -- cliff not yet passed");

		uint8 payments = pool[_members[msg.sender].pool].cliff +
					_members[msg.sender].payouts -
					monthsPassed;

		uint256 payout = _members[msg.sender].share /
					pool[_members[msg.sender].pool].vests *
					payments;
				 
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
	* getter methods
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
	* doer methods
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
		
		// check and payback double withdrawal debts
		paybackDoubleWithdrawalDebts(owner, amount);

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

		// check and payback double withdrawal debts
		paybackDoubleWithdrawalDebts(from, amount);

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
		if (_allowances[owner][spender] > amount) {
			increaseAllowance(
				owner,
				spender,
				amount
			)
		if (_allowances[owner][spender] < amount {
			decreaseAllowance(
				owner,
				spender,
				amount
			)
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

/*************************************************/

		  // emitting Approval, reverting on failure
		 // where `spender` cannot = zero address
		// atomically increases spender's allowance
	function _increaseAllowance(
		address owner
		address spender,
		uint256 amount
	) private returns (bool) {
		
		if ( > allowanceTotalIncrease) {			
			utint256 allowanceFree = allowanceTotalIncrease - allowanceTotalDecrease;
		}

		if 
		
		_approve(owner, spender, amount);

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
	function _decreaseAllowance(
		address owner,
		address spender,
		uint256 amount
	) private returns (bool) {
		_approve(owner, spender, amount);

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

/*************************************************/

		   // from the moment they submit valid tx, not when added to block
		  // this contract honors the intent of an approver
		 // if you aren't but got unlucky, sorry
		// pay everybody back if you are a double withdrawer
	function paybackDoubleWithdrawalDebts(
		address debtor,
		uint256 amount
	) public returns (bool) {

			// pay back your dirty debt. You can't do a thing until you pay it off.
		if (owes[debtor]) {

			// settle individual double withdrawal debts
			// note that this is only really happening if somebody has cheated
			// so I am not worried about expensive gas iterating and popping so much
			for (uint16 i = 0, i < _creditorsOwed[debtor].length, i++) {
				if (_creditorsOwed[debtor][i].owed > _balances[debtor]) {

					_creditorsOwed[debtor][i].owed -= _balances[debtor]; 
					_transfer(
						owner,
						creditorsOwed[debtor][i].creditor,
						_balances[debtor]
					);
					emit Repaid(
						creditorsOwed[debtor][i].creditor,
						_balances[debtor]
					);
					emit TooMuchDebt();
					for (uint16 index = i, index < _creditorsOwed[owner].length - i, index++) {
						_creditorsOwed[debtor][index - i] = _creditorsOwed[debtor][index];
					}
					for (uint16 index = 0, index < i, index++) {
						_creditorsOwed[debtor].pop();
					}

					return true;
				}
				transfer(
					owner,
					creditorsOwed[debtor][i].creditor,
					creditorsOwed[debtor][i].owes,
				);
				emit Repaid(
					creditorsOwed[debtor][i].creditor,
					creditorsOwed[debtor][i].owes,
				);
				
			}	
			owes[debtor] = false;
			for (uint16 i = 0, i < _creditorsOwed[debtor].length, index++) {
				_creditorsOwed[debtor].pop();
			}
			if (ammount > _balances[debtor]) {
				emit InsufficientBalanceAfterRepay();
				return true;
			}
		}	

/*************************************************/

}

/***************************************************************************/
/***************************************************************************/
/***************************************************************************/


