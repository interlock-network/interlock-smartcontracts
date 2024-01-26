//*************************************************************/
//*************************************************************/
//*************************************************************/
// SPDX-License-Identifier: MIT
//
// Interlock Network ERC-20 ILOCK Token Version 1
//
// Contributors:
// blairmunroakusa
// ...
//
// This contract is from the Open Zeppelin 5 contract suite.
//
// Vesting is managed by external TokenOps vesting contracts.
//
//*************************************************************/
//*************************************************************/
//*************************************************************/

pragma solidity ^0.8.18;

import "./IERC20Upgradeable.sol";
import "./extensions/IERC20MetadataUpgradeable.sol";
import "../../utils/ContextUpgradeable.sol";
import "../../proxy/utils/Initializable.sol";

/** @title A contract for Interlock Network token management. */
/** @notice This contract includes events, state variables, and functions for the Interlock Network ERC20 token. */
contract ILOCKV1 is Initializable,
                    ContextUpgradeable,
                    IERC20Upgradeable,
                    IERC20MetadataUpgradeable {

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * declarations
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/

    /** @dev Emitted when the contract is paused. */
    event Paused(
        address account);
    /** @dev Emitted when the contract is unpaused. */
    event Unpaused(
        address account);
	/** @dev Indicates if the contract is paused. */
    bool private _paused;

	/** @dev Indicates if the TGE (Token Generation Event) has been triggered. */
    bool public tgeTriggered;
	/** @dev Indicates if the contract has been initialized. */
    bool public initialized;

	/** @dev Constant value for token decimals. */
    uint8 constant private _DECIMALS = 18;
	/** @dev Constant value for token name. */
    string constant private _NAME = "Interlock Network";
	/** @dev Constant value for token symbol. */
    string constant private _SYMBOL = "TESTILOCK";

	/** @dev Total supply of tokens. */
    uint256 private _totalSupply;

    /** @dev Constant value for decimal magnitude. */
    uint256 constant private _DECIMAL_MAGNITUDE = 10 ** _DECIMALS;
	/** @dev Constant value for token cap. */
    uint256 constant private _CAP = 1_000_000_000 * _DECIMAL_MAGNITUDE;
	/** @dev Constant value for Aleph token supply. */
    uint256 constant private _ALEPH_SUPPLY = 300_000_000 * _DECIMAL_MAGNITUDE;
	/** @dev Constant value for Rewards Pool. */
    uint256 constant private _REWARDS_POOL = 300_000_000 * _DECIMAL_MAGNITUDE;
	/** @dev Constant value for Azero Rewards Pool. */
    uint256 constant private _AZERO_REWARDS_POOL = 150_000_000 * _DECIMAL_MAGNITUDE;

	/** @dev Address of the contract owner. */
    address public contractOwner;
	/** @dev Address of the multisig safe. */
    address public multisigSafe;

	/** @dev Mapping for tracking balances. */
    mapping(
        address => uint256) private _balances;
	/** @dev Mapping for tracking allowances. */
    mapping(
        address => mapping(
            address => uint256)) private _allowances;

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * init
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/

	/** @dev Initializes the contract, setting the contract owner and marking it as initialized. */
    function initialize(
    ) public initializer {

        contractOwner = _msgSender();

        require(
            initialized == false,
            "contract already initialized");

        //
        //
        // ??? TokenOps: How do we manage supply incrementation
		// Answer: implement issue #242
        _totalSupply = 0;

        initialized = true;
        tgeTriggered = false; }

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * modifiers
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/

    /** @dev Ensures that the function is called only by the contract owner. */
	modifier onlyOwner(
    ) {
        require(
            _msgSender() == contractOwner,
            "only owner can call");
        _; }

//***********************************/

    /** @dev Ensures that the function is called only by the multisig safe. */
    modifier onlyMultisigSafe(
    ) {
        require(
            _msgSender() == multisigSafe,
            "only multisig safe can call");
        _; }

//***********************************/

    /** @dev Ensures that a non-zero address is provided. */
	/** @param _address - Address of interest to check. */
    modifier noZero(
        address _address
    ) {
        require(
            _address != address(0),
            "zero address where it shouldn't be");
        _; }

//***********************************/

    /** @dev Ensures that there are enough tokens available for the operation. */
	/** @param _available - Address of token available. */
	/** @param _amount - Address of token needed. */
    modifier isEnough(
        uint256 _available,
        uint256 _amount
    ) {
        require(
            _available >= _amount,
            "not enough tokens available");
        _; }

//***********************************/

    /** @dev Ensures that the contract is not paused. */
    modifier notPaused(
    ) {
        require(
            !_paused,
            "contract is paused");
        _; }

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * TGE
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/

        // generates all the tokens
    function triggerTGE(
        address multisigSafe_
    ) public 
        onlyOwner
        noZero(multisigSafe_)
    {
        require(
            initialized,
            "contract not initialized");
        require(
            !tgeTriggered,
            "TGE already happened");

        multisigSafe = multisigSafe_;

        // mint the tokens
        _balances[address(this)] = _CAP - _ALEPH_SUPPLY;

        // TODO:
        // here, manually approve all tokenops vesting contracts
        // ...this will be safer and more transparent than
        //    separate approval transaction post-TGE

        // approve the contract owner to issue rewards
        _approve(
            address(this),
            contractOwner,
            _REWARDS_POOL - _AZERO_REWARDS_POOL);

        // this must never happen again...
        tgeTriggered = true; }

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * ownership and pausability
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/                    

        // changes the contract owner
    function changeOwner(
        address newOwner
    ) public 
        onlyMultisigSafe
        noZero(newOwner)
    {
        contractOwner = newOwner; }

//***********************************/

        // returns pause status of contract
    function paused(
    ) public view returns (
        bool isPaused
    ) {
        return _paused; }

//***********************************/

        // pauses any functions requiring unpause
    function pause(
    ) public
        onlyMultisigSafe
    {    
        require(
            !paused(),
            "already paused");
        _paused = true;
        
        emit Paused(_msgSender()); }

//***********************************/

        // resumes operation of functions requiring unpause
    function unpause(
    ) public
        onlyMultisigSafe
    {    
        require(
            paused(),
            "already unpaused");
        _paused = false;
        
        emit Unpaused(_msgSender()); }

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * ERC20 getter methods
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/

        // gets token name (Interlock Network)
    function name(
    ) public pure override returns (
        string memory _name
    ) {
        return _NAME; }

//***********************************/

        // gets token symbol (ILOCK)
    function symbol(
    ) public pure override returns (
        string memory _symbol
    ) {
        return _SYMBOL; }

//***********************************/

        // gets token decimal number
    function decimals(
    ) public pure override returns (
        uint8 _decimals
    ) {
        return _DECIMALS; }

//***********************************/

        // gets tokens minted
    function totalSupply(
    ) public view override returns (
        uint256 _supply
    ) {
        return _totalSupply; }

//***********************************/

        // gets account balance (tokens payable)
    function balanceOf(
        address account
    ) public view override returns (
        uint256 _balance
    ) {
        return _balances[account]; }

//***********************************/

        // gets tokens spendable by spender from owner
    function allowance(
        address owner,
        address spender
    ) public view virtual override returns (
        uint256 _allowance
    ) {
        return _allowances[owner][spender]; }

//***********************************/

        // gets token cap
    function cap(
    ) public pure returns (
        uint256 _cap
    ) {
        return _CAP; }

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * ERC20 doer methods
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/

           // emitting Transfer, reverting on failure
          // where caller balanceOf must be >= amount
         // where `to` cannot = zero  address
        // increases spender allowance
    function transfer(
        address to,
        uint256 amount
    ) public virtual override returns (
        bool success
    ) {
        address owner = _msgSender();
        _transfer(
            owner,
            to,
            amount);
        return true; }

//***********************************/

            // emitting Approval and Transfer, reverting on failure
           // where msg.sender allowance w/`from` must be >= amount
          // where `from` balance must be >= amount
         // where `from` and `to` cannot = zero address
        // pays portion of spender's allowance with owner to recipient
    function transferFrom(
        address from,
        address to,
        uint256 amount
    ) public virtual override returns (
        bool success
    ) {
        address spender = _msgSender();        
        _spendAllowance(
            from,
            spender,
            amount);
        _transfer(
            from,
            to,
            amount);
        return true; }

//***********************************/

           // emitting Transfer, reverting on failure
          // where `from` balance must be >= amount
         // where `from` and `to` cannot = zero address
        // is internal implementation of transfer() above
    function _transfer(
        address from,
        address to,
        uint256 amount
    ) internal virtual
        noZero(from)
        noZero(to)
        isEnough(_balances[from], amount)
    returns (
        bool success
    ) {
        _beforeTokenTransfer(
            from,
            to,
            amount);
        uint256 fromBalance = _balances[from];
        unchecked {
            _balances[from] = fromBalance - amount;
            _balances[to] += amount; }
        emit Transfer(
            from,
            to,
            amount);
        _afterTokenTransfer(
            from, 
            to,
            amount);
        return true; }

//***********************************/

         // emitting Approval event, reverting on failure
        // defines spender's transferrable tokens from from msg.sender
    function approve(
        address spender,
        uint256 amount
    ) public virtual override returns (
        bool succcess
    ) {
        address owner = _msgSender();
        _approve(
            owner,
            spender,
            amount);
        return true; }

//***********************************/

         // emitting Approvl event, reverting on failure
        // is internal implementation of approve() above 
    function _approve(
        address owner,
        address spender,
        uint256 amount
    ) internal virtual
        notPaused
        noZero(owner)
        noZero(spender)
    {
        _allowances[owner][spender] = amount;
        emit Approval(
            owner,
            spender,
            amount); }

//***********************************/

           // emitting Approval event, reverting on failure 
          // will do nothing if infinite allowance
         // used strictly internally
        // deducts from spender's allowance with owner
    function _spendAllowance(
        address owner,
        address spender,
        uint256 amount
    ) internal virtual
        isEnough(allowance(owner, spender), amount)
    {
        uint256 currentAllowance = allowance(owner, spender);
        if (currentAllowance != type(uint256).max) {
            unchecked {
                _approve(
                    owner,
                    spender,
                    currentAllowance - amount);} } }

//***********************************/

        // allows client safe approval facing double spend attack
    function increaseAllowance(
        address spender,
        uint256 addedValue
    ) public virtual returns (
        bool success
    ) {    
        address owner = _msgSender();
        _approve(
            owner,
            spender,
            allowance(owner, spender) + addedValue);
        return true; }

//***********************************/

        // allows client safe approval facing double spend attack
    function decreaseAllowance(
        address spender,
        uint256 subtractedValue
    ) public virtual returns (
        bool success
    ) {
        address owner = _msgSender();
        uint256 currentAllowance = allowance(owner, spender);
        require(
            currentAllowance >= subtractedValue,
            "ERC20: decreased allowance below zero");
        unchecked {
            _approve(
                owner,
                spender,
                currentAllowance - subtractedValue); }
        return true; }

//***********************************/

        // hook that inserts behavior prior to transfer
    function _beforeTokenTransfer(
        address _from,
        address _to,
        uint256 _amount
    ) internal virtual notPaused {}

//***********************************/

        // hook that inserts behavior after to transfer
    function _afterTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal virtual {}

//***********************************/


    function testingIncrementMonth(
    ) public returns (uint256) {


        return 1; }

    uint256[100] public storageGap;
}

//*************************************************************/
//*************************************************************/
//*************************************************************/



