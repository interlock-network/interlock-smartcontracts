// SPDX-License-Identifier: MIT
//
// Interlock ERC-20 INTR Token Mint Platform
// 		(containing)
// components from OpenZeppelin v4.6.0 contract (token/ERC20/ERC20.sol)
//
// Contributors:
// blairmunroakusa
// ...

pragma solidity ^0.8.0;

import "./IERC20.sol";
import "./utils/Context.sol";

 /**
 * This implementation is agnostic to the way tokens are created. This means
 * that a supply mechanism has to be added in a derived contract using {_mint}.
 * For a generic mechanism see {ERC20PresetMinterPauser}.
 *
 * We have followed general OpenZeppelin Contracts guidelines: functions revert
 * instead returning `false` on failure. This behavior is nonetheless
 * conventional and does not conflict with the expectations of ERC20
 * applications.
 *
 * Additionally, an {Approval} event is emitted on calls to {transferFrom}.
 * This allows applications to reconstruct the allowance for all accounts just
 * by listening to said events. Other implementations of the EIP may not emit
 * these events, as it isn't required by the specification.
 *
 * Finally, the non-standard {decreaseAllowance} and {increaseAllowance}
 * functions have been added to mitigate the well-known issues around setting
 * allowances. See {IERC20-approve}.
 **/

contract ERC20 is Context, IERC20 {

    	/**
     	* setup
     	**/

    	mapping(address => uint256) private _balances;
    	mapping(address => mapping(address => uint256)) private _allowances;

    	uint256 private _totalSupply;

    	string private _name;
    	string private _symbol;
	string private _decimals;

	
    		// initializes contract
	constructor(
		string memory name_,
		string memory symbol_,
		string memory decimals_
	) {
        	_name = name_;
        	_symbol = symbol_;
		_decimals = decimals_; }


    	/**
     	* getter methods
     	**/
		// gets token name (Interlock Network)
    	function name() public view override returns (string memory) {
        	return _name; }


		// gets token symbol (INTR)
    	function symbol() public view override returns (string memory) {
        	return _symbol; }


		// gets token decimals (18)
    	function decimals() public view override returns (uint8) {
		return _decimals; }
 /**
 * Returns the number of decimals used to get its user representation.
 * For example, if `decimals` equals `2`, a balance of `505` tokens should
 * be displayed to a user as `5.05` (`505 / 10 ** 2`).
 *
 * NOTE: This information is only used for _display_ purposes: it in
 * no way affects any of the arithmetic of the contract, including
 * {IERC20-balanceOf} and {IERC20-transfer}.
 **/


		// gets tokens minted so far (total circulating)
    	function totalSupply() public view override returns (uint256) {
        	return _totalSupply; }


		// gets account balance (tokens payable)
    	function balanceOf(address account) public view override returns (uint256) {
        	return _balances[account]; }


		// gets tokens spendable by spender from owner
    	function allowance(
		address owner,
		address spender
	) public view virtual override returns (uint256) {
        	return _allowances[owner][spender]; }


    	/**
     	* doer methods
     	**/
		   // emitting Transfer, reverting on failure
		  // where caller balanceOf must be >= amount
		 // where `to` cannot = zero  address
		// increases spender allowance
    	function transfer(
		address to,
		uint256 amount
	) public virtual override returns (bool) {
        	address owner = _msgSender();
        	_transfer(owner, to, amount);
        	return true; }

		// internal implementation of transfer() above
    	function _transfer(
        	address from,
        	address to,
        	uint256 amount
    	) internal virtual {
        	require(from != address(0),
			"ERC20: transfer from the zero address");
        	require(to != address(0),
			"ERC20: transfer to the zero address");
        	_beforeTokenTransfer(from, to, amount);
        	uint256 fromBalance = _balances[from];
        	require(fromBalance >= amount,
			"ERC20: transfer amount exceeds balance");
        	unchecked {
            		_balances[from] = fromBalance - amount;
        	}
        	_balances[to] += amount;
        	emit Transfer(from, to, amount);
        	_afterTokenTransfer(from, to, amount); }


		   // emitting Approval, reverting on failure
		  // (=> no allownance delta when TransferFrom)
		 // where amount = u256.max => infinite allowance
		// defines tokens available to spender from msg.sender
	function approve(
		address spender,
		uint256 amount
	) public virtual override returns (bool) {
        	address owner = _msgSender();
        	_approve(owner, spender, amount);
        	return true; }

		// internal implementation of approve() above 
    	function _approve(
        	address owner,
        	address spender,
        	uint256 amount
    	) internal virtual {
        	require(owner != address(0),
			"ERC20: approve from the zero address");
        	require(spender != address(0),
			"ERC20: approve to the zero address");
		_allowances[owner][spender] = amount;
        	emit Approval(owner, spender, amount); }


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
    	) public virtual override returns (bool) {
        	address spender = _msgSender();
        	_spendAllowance(from, spender, amount);
        	_transfer(from, to, amount);
        	return true; }


		  // emitting Approval, reverting on failure
		 // where `spender` cannot = zero address
		// atomically increases spender's allowance
    	function increaseAllowance(
		address spender,
		uint256 addedValue
	) public virtual returns (bool) {
        	address owner = _msgSender();
        	_approve(owner, spender, allowance(owner, spender) + addedValue);
        	return true; }
 /**
 * Above and below are alternatives to {approve} that can be used
 * as a mitigation for problems described in {IERC20-approve}.
 *
 * I believe the reason for needing atomic increase
 * is that the operation ties down allowance getter
 * preventing an allowance access before increase is complete
 *
 * ?? Why is there no owner balance check for increaseAllowance() ??
 * ?? Why unchecked _approve ?? --save on gas
 **/
		   // emitting Approval, reverting on failure
		  // where `spender` must have allowance >= `subtractedValue`
		 // where `spender` cannot = zero address
		// atomically decreases spender's allowance
   	function decreaseAllowance(
		address spender,
		uint256 subtractedValue
	) public virtual returns (bool) {
        	address owner = _msgSender();
        	uint256 currentAllowance = allowance(owner, spender);
        	require(currentAllowance >= subtractedValue,
			"ERC20: decreased allowance below zero");
        	unchecked {
            		_approve(owner, spender, currentAllowance - subtractedValue);
		}
        	return true; }


		   // reverting on failure
		  // emitting Transfer event _from_ zero address
		 // where `account` cannot = zero address
		// increases token supply by assigning to account
    	function _mint(
		address account,
		uint256 amount
	) internal virtual {
        	require(account != address(0),
			"ERC20: mint to the zero address");
        	_beforeTokenTransfer(address(0), account, amount);
        	_totalSupply += amount;
        	_balances[account] += amount;
        	emit Transfer(address(0), account, amount);
        	_afterTokenTransfer(address(0), account, amount); }


		   // emitting Transfer, reverting on failure
		  // where `account` must have >= burn amount
		 // where `account` cannot = zero address
		// decreases token supply by deassigning from account
	function _burn(
		address account,
		uint256 amount
	) internal virtual {
        	require(account != address(0),
			"ERC20: burn from the zero address");
        	_beforeTokenTransfer(account, address(0), amount);
        	uint256 accountBalance = _balances[account];
        	require(accountBalance >= amount,
			"ERC20: burn amount exceeds balance");
        	unchecked {
            		_balances[account] = accountBalance - amount;
        	}
        	_totalSupply -= amount;
        	emit Transfer(account, address(0), amount);
        	_afterTokenTransfer(account, address(0), amount); }


		   // emitting Approval if finite, reverting on failure 
		  // will do nothing if infinite allowance
		 // used strictly internally
		// deducts from spender's allowance with owner
	function _spendAllowance(
        	address owner,
        	address spender,
        	uint256 amount
    	) internal virtual {
        	uint256 currentAllowance = allowance(owner, spender);
        	if (currentAllowance != type(uint256).max) {
            		require(currentAllowance >= amount,
				"ERC20: insufficient allowance");
            		unchecked {
                		_approve(owner, spender, currentAllowance - amount);
            		} } }


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

}
