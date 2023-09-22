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

pragma solidity ^0.8.20;

import "./IERC20.sol";
import "./ILOCKpool.sol";

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
	uint256 private _cap = 1000000000;

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
	mapping(address => bool) private ispool;
	mapping(address => uint8) private poolint;

	address public tokenlockPool;

		// keeping track of members
	struct Stake {
		uint256 paid;
		uint256 share;
		uint8 cliff;
		uint8 pool;
		uint8 payouts; }
	mapping(address => Stake[]) private _stakes;

		// core token balance and allowance mappings
	mapping(address => uint256) private _balances;
	mapping(address => mapping(address => uint256)) private _allowances;


		// basic token data
	string private _name = "Interlock Network";
	string private _symbol = "ILOCK";
	uint256 private _totalSupply = 0;
	address private _owner;

		// tracking time
	uint256 public nextPayout;
	uint8 public monthsPassed; 

		// keeping track of irreversible actions
	bool public TGEtriggered = false;
	bool public supplySplit = false;

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
		tokenlockPool = address(new ILOCKpool());
		_balances[tokenlockPool] = 0;

		// mint
		_balances[address(this)] = _totalSupply;
		
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
	modifier onlyOwner(
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
	) public onlyOwner {
		
		// guard
		require(
			supplySplit == false,
			"supply split already happened");
		// create pool accounts and initiate
		for (uint8 i = 0; i < _poolNumber; i++) {
			address Pool = address(new ILOCKpool());
			pools.push(Pool);
			poolexists(Pool) = true;
			_balances[Pool] = 0;
		}
		// this must never happen again...
		supplySplit = true;
	}

/*************************************************/

		// generates all the tokens
	function triggerTGE(
	) public onlyOwner {

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
	) public onlyOwner {

		// reassign
		_owner = newOwner;
	}

/***************************************************************************/
/***************************************************************************/
/***************************************************************************/
	/**
	* stakeholder entry and distribution
	**/
/***************************************************************************/
/***************************************************************************/
/***************************************************************************/

		// register stakeholder
	function registerStake(
		address stakeholder,
		Stake data
	) public noZero(stakeholder) onlyOwner returns (bool) {

		// validate input
		require(
			data != [],
			"no stake provided for stakeholder");
		require(
			data.paid == 0,
			"amount paid must be zero");
		require(
			data.cliff <= pools[pool].cliff,
			"cliff exceeds pool cliff");
		require(
			data.share >= pools[pool].vests,
			"share is too small");
		require(
			data.pool < _poolNumber,
			"invalid pool number");
		require(
			data.payouts == 0,
			"payouts at this point muzt be zero");

		// create stake or append
		_stakes(stakeholder).push(data);

		return true; }

		// claim stake for vest periods accumulated
	function claimStake(
		uint8 stakenumber
	) public returns (bool) {

		// see if we need to update time
		_checkTime();

		// make sure stake number exists
		require(
			_stakes[msg.sender].length > stakenumber,
			"stake does not exist");
		
		Stake stake = _stakes[msg.sender][stakenumber];
		uint8 cliff = pool[stake.pool].cliff;
		uint8 vests = pool[stake.pool].vests;

		// number of payouts must not surpass number of vests
		require(
			stake.payouts < pool[stake.pool].vests,
			"member already collected entire token share");

		// make sure cliff has been surpassed
		require(
			monthsPassed >= pool[stake.pool].cliff,
			"too soon -- cliff not yet passed");

		
		// determine the number of payments claimant has rights to
		uint8 payments;

		// when time has past vesting period, pay out remaining unclaimed payments
		if (cliff + vests <= monthsPassed) {
			
			payments = vests - stake.payouts;

		// don't count months past vests+cliff as payments
		} else {

			payments = 1 + monthsPassed - stake.payouts - cliff;
		}
				
		// use payments to calculate amount to pay out
		uint256 payout = stake.share / vests * payments;

		// if at final payment, add remainder of share to final payment
		if (stake.share - stake.paid - payout < stake.share / vests) {
			
			payout += stake.share % vests;
		}

		// transfer and make sure it succeeds
		require(
			_transfer(pools[stake.pool], msg.sender, payout),
			"stake claim transfer failed");

		// update member state
		_members[msg.sender].payouts += payments;
		_members[msg.sender].paid += payout;
		
		return true; }	

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
	function reserve(
	) public view returns (uint256) {

		return _reserve; }
 
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

		return true; }


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

        return true; }

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
			_approve(owner, spender, allowance(owner, spender) - amount);} }

/*************************************************

/*
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
SHOUTING SHOUTING SHOUTING!
*/

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
	function claimStatus(
		uint8 stakenumber
	) public view returns (
		uint256 timeLeft,
		uint256 paidOut,
		uint256 payRemaining,
		uint256 payAvailable
	) {

		Stake stake = _stakes[msg.sender][stakenumber];
		uint8 cliff = pool[stake.pool].cliff;
		uint8 vests = pool[stake.pool].vests;

		// compute the time left until the next payment is available
		// if months passed beyond last payment, stop counting
		if (monthsPassed >= vests + cliff) {
			
			timeLeft = 0;

		// when cliff hasn't been surpassed, include that time into countdown
		} else if (monthsPassed < cliff) {
			
			timeLeft = (cliff - monthsPassed - 1) * 30 days +
				    nextPayout - block.timestamp;

		// during vesting period, timeleft is only time til next month's payment
		} else {

			timeLeft = nextPayout - block.timestamp;
		}

		// how much has member already claimed
		paidOut = stake.paid;

		// how much does member have yet to collect, after vesting complete
		payRemaining = stake.share - paidOut;

		// computer the pay available to claim at current moment
		// if months passed are inbetween cliff and end of vesting period
		if (monthsPassed >= cliff && monthsPassed < cliff + vests) {
			
			payAvailable = (1 + monthsPassed - cliff - stake.payouts) *
			      	       (stake.share / vests);

		// until time reaches cliff, no pay is available
		} else if (monthsPassed < cliff ){

			payAvailable = 0;

		// if time has passed cliff and vesting period, the entire remaining share is available
		} else {

			payAvailable = stake.share - paidOut;
		}

		// if at final payment, add remainder of share to final payment
		if (stake.share - paidOut - payAvailable < stake.share / vests && payAvailable > 0) {
			
			payAvailable += stake.share % vests;
		}

		return (
			timeLeft,
			paidOut,
			payRemaining,
			payAvailable
		);
	}
}

/***************************************************************************/
/***************************************************************************/
/***************************************************************************/
