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
// This contract is taken from the Open Zeppelin 4 contract suite.
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
contract ILOCKV2 is Initializable,
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
        address account
    );
    /** @dev Emitted when the contract is unpaused. */
    event Unpaused(
        address account
    );
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

    /** @dev ERC20 mapping for tracking balances. */
    mapping(
        address => uint256
    ) private _balances;
    /** @dev ERC20 mapping for tracking allowances. */
    mapping(
        address => mapping(
            address => uint256
        )
    ) private _allowances;

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
            "contract already initialized"
        );

        //
        //
        // ??? TokenOps: How do we manage supply incrementation
        // Answer: implement issue #242
        _totalSupply = 0;

        initialized = true;
        tgeTriggered = false;
    }

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
            "only owner can call"
        );
        _;
    }

//***********************************/

    /** @dev Ensures that the function is called only by the multisig safe. */
    modifier onlyMultisigSafe(
    ) {
        require(
            _msgSender() == multisigSafe,
            "only multisig safe can call"
        );
        _;
    }

//***********************************/

    /** @dev Ensures that a non-zero address is provided. */
    /** @param _address - Address of interest to check. */
    modifier noZero(
        address _address
    ) {
        require(
            _address != address(0),
            "zero address where it shouldn't be"
        );
        _;
    }

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
            "not enough tokens available"
        );
        _;
    }

//***********************************/

    /** @dev Ensures that the contract is not paused. */
    modifier notPaused(
    ) {
        require(
            !_paused,
            "contract is paused"
        );
        _;
    }

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * TGE
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/

    /** @dev Triggers the Token Generation Event, setting up the initial token distribution. */
    /** @param multisigSafe_ - Address of Gnosis Safe multisig account. */
    /** @notice Only contract owner may call. */
    /** @notice Input address parameter may not be zero address. */
    /** @notice Emits one approval event per vesting pool. */
    /** @notice Emits one approval event for contract owner to issue rewards. */
    function triggerTGE(
        address multisigSafe_
    ) public 
        onlyOwner
        noZero(multisigSafe_)
    {
        require(
            initialized,
            "contract not initialized"
        );
        require(
            !tgeTriggered,
            "TGE already happened"
        );

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
            _REWARDS_POOL - _AZERO_REWARDS_POOL
        );

        // this must never happen again...
        tgeTriggered = true;
    }

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * ownership and pausability
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/                    

    /** @dev Changes the contract owner to a new owner. */
    /** @param newOwner - Address of Gnosis Safe multisig account. */
    /** @notice Only multisig Safe address may call. */
    /** @notice Input address parameter may not be zero address. */
    function changeOwner(
        address newOwner
    ) public 
        onlyMultisigSafe
        noZero(newOwner)
    {
        contractOwner = newOwner;
    }

//***********************************/

    /** @dev Returns the pause status of the contract. */
    /** @return isPaused - Paused status. */
    function paused(
    ) public view returns (
        bool isPaused
    ) {
        return _paused;
    }

//***********************************/

    /** @dev Pauses the contract, preventing certain functions from being executed. */
    /** @notice Only multisig Safe address may call. */
    /** @notice Emits Paused event. */
    function pause(
    ) public
        onlyMultisigSafe
    {    
        require(
            !paused(),
            "already paused"
        );
        _paused = true;
        
        emit Paused(_msgSender());
    }

//***********************************/

    /** @dev Unpauses the contract, allowing certain functions to be executed again. */
    /** @notice Only multisig Safe address may call. */
    /** @notice Emits Unpaused event. */
    function unpause(
    ) public
        onlyMultisigSafe
    {    
        require(
            paused(),
            "already unpaused"
        );
        _paused = false;
        
        emit Unpaused(_msgSender());
    }

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * ERC20 (and other) getter methods 
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/

    /** @dev Returns the name of the token. */
    /** @return _name -  Token name, string, Interlock Network. */
    function name(
    ) public pure override returns (
        string memory _name
    ) {
        return _NAME;
    }

//***********************************/

    /** @dev Returns the symbol of the token. */
    /** @return _symbol - Token symbol, string, ILOCK. */
    function symbol(
    ) public pure override returns (
        string memory _symbol
    ) {
        return _SYMBOL;
    }

//***********************************/

    /** @dev Returns the number of decimals the token uses. */
    /** @return _decimals - Token decimals, uint8, 18. */
    function decimals(
    ) public pure override returns (
        uint8 _decimals
    ) {
        return _DECIMALS;
    }

//***********************************/

    /** @dev Returns the total supply of tokens. */
    /** @return _supply - Circulating supply (CAP - ALEPH - address(this) balance), uint256. */
    function totalSupply(
    ) public view override returns (
        uint256 _supply
    ) {
        return _totalSupply;
    }

//***********************************/

    /** @dev Returns the balance of the specified account. */
    /** @notice Input address parameter may not be zero address. */
    /** @return _balance - account token balance, uint256. */
    function balanceOf(
        address account
    ) public view override returns (
        uint256 _balance
    ) {
        return _balances[account];
    }

//***********************************/

    /** @dev Returns the allowance one address has to spend on behalf of another. */
    /** @notice Input address parameter may not be zero address. */
    /** @return _allowance - account token allowance for spender, uint256. */
    function allowance(
        address owner,
        address spender
    ) public view virtual override returns (
        uint256 _allowance
    ) {
        return _allowances[owner][spender];
    }

//***********************************/

    /** @dev Returns the token cap. */
    /** @return _cap - ILOCK token cap accross all blockchains, uint256, 1_000_000_000. */
    function cap(
    ) public pure returns (
        uint256 _cap
    ) {
        return _CAP;
    }

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * ERC20 doer methods
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/

    /** @dev Transfer tokens from msg.sender to another address, per ERC20 standard. */
    /** @notice Input address parameter may not be zero address. */
    /** @notice Emits Transfer event. */
    /** @return success - true. Revert on failure. */
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
            amount
        );
        return true;
    }

//***********************************/

    /** @dev Transfer tokens from one address to another, per ERC20 standard. */
    /** @notice Input address parameter may not be zero address. */
    /** @notice Emits Transfer event. */
    /** @notice Emits Approval event. */
    /** @return success - true. Revert on failure. */
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
            amount
        );
        _transfer(
            from,
            to,
            amount
        );
        return true;
    }

//***********************************/

    /** @dev Internal transfer function, per ERC20 standard. */
    /** @notice Input address parameter may not be zero address. */
    /** @notice Contract must not be paused. */
    /** @notice Origin address must have enough tokens. */
    /** @notice Emits Transfer event. */
    /** @return success - true. Revert on failure. */
    function _transfer(
        address from,
        address to,
        uint256 amount
    ) internal virtual
        noZero(from)
        noZero(to)
        notPaused
        isEnough(_balances[from], amount)
    returns (
        bool success
    ) {
        _beforeTokenTransfer(
            from,
            to,
            amount
        );
        uint256 fromBalance = _balances[from];
        unchecked {
            _balances[from] = fromBalance - amount;
            _balances[to] += amount;
        }
        emit Transfer(
            from,
            to,
            amount
        );
        _afterTokenTransfer(
            from, 
            to,
            amount
        );
        return true;
    }

//***********************************/

    /** @dev Approves spender to transfer tokens from from msg.sender, per ERC20 standard. */
    /** @notice Input address parameter may not be zero address. */
    /** @notice Emits Approval event. */
    /** @return success - true. Revert on failure. */
    function approve(
        address spender,
        uint256 amount
    ) public virtual override returns (
        bool success
    ) {
        address owner = _msgSender();
        _approve(
            owner,
            spender,
            amount
        );
        return true;
    }

//***********************************/

    /** @dev Sets `amount` as the allowance of `spender` over the `owner`s tokens, per ERC20 standard. */
    /** @notice Contract must not be paused. */
    /** @notice Input address parameter may not be zero address. */
    /** @notice Emits Approval event. */
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
            amount
        );
    }

//***********************************/

    /** @dev Updates `owner` s allowance for `spender` based on spent `amount`.
    /** @notice Allowance must be enough tokens. */
    /** @notice Emits Approval event. */
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
                    currentAllowance - amount
                );
            }
        }
    }

//***********************************/

    /** @dev Allows client safe approval facing double spend attack. */
    /** @notice Input address parameter may not be zero address. */
    /** @notice Emits Approval event. */
    /** @return success - true. Reverts on failure. */
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
            allowance(owner, spender) + addedValue
        );
        return true;
    }

//***********************************/

    /** @dev Allows client safe approval facing double spend attack. */
    /** @notice Input address parameter may not be zero address. */
    /** @notice Emits Approval event. */
    /** @return success - true. Reverts on failure. */
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
            "ERC20: decreased allowance below zero"
        );
        unchecked {
            _approve(
                owner,
                spender,
                currentAllowance - subtractedValue
            );
        }
        return true;
    }

//***********************************/

    /** @dev Hook that is called before any transfer of tokens. */
    function _beforeTokenTransfer(
        address _from,
        address _to,
        uint256 _amount
    ) internal virtual notPaused {}

//***********************************/

    /** @dev Hook that is called after any transfer of tokens. */
    /** @notice This takes care of the supply increment decrement functionality. */
    function _afterTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal virtual {
    	
		// token transfer from this contract means tokens entering circulation
		if (from == address(this)) {
			_totalSupply += amount;	
		}
		// token transfer to this contract means tokens leaving circulation
		if (to == address(this)) {
			_totalSupply -= amount;	
		}
    }

//*************************************************************/
//*************************************************************/
//*************************************************************/

    function newFeature(
    ) public pure returns (string memory) {

    return "new feature"; }


    uint256 public newstorage;
    uint256[99] public storageGap;
}

//*************************************************************/
//*************************************************************/
//*************************************************************/



