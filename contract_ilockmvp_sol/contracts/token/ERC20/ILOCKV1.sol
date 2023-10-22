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
// This contract is comprised of Open Zeppelin components and
// bespoke components.
//
// This is a token contract that implements a vesting schedule
// for ILOCK stakeholders to claim their share of token (their 
// 'stake') over the course of the vesting period. NOTE: a stake
// in this context is not the same as 'staking tokens' in the
// typical web3 sense. 'Stake' in this context in in the sense
// that a stakeholder has a stake or investment in the project.
//
// Stakeholders are grouped into various token pools, with each
// pool being defined by the Interlock Network tokenomics token
// distribution schedule. Each pool is devoted to a specific
// type of stakeholder with its own vesting schedule (cliff and
// vesting period).
//
//*************************************************************/
//*************************************************************/
//*************************************************************/

pragma solidity ^0.8.0;

import "./IERC20Upgradeable.sol";
import "./ILOCKpool.sol";
import "./extensions/IERC20MetadataUpgradeable.sol";
import "../../utils/ContextUpgradeable.sol";
import "../../proxy/utils/Initializable.sol";

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

    /** @dev **/
    event Paused(
        address account);
    event Unpaused(
        address account);
    event StakeRegistered(
        Stake stake);
    event StakeClaimed(
        address stakeholder,
        bytes32 stakeIdentifier,
        uint256 amount);

    bool private _paused;

    string constant private _NAME = "Interlock Network";
    string constant private _SYMBOL = "TESTILOCK";
    uint8 constant private _DECIMALS = 18;
    uint256 constant private _DECIMAL_MAGNITUDE = 10 ** _DECIMALS;
    uint256 constant private _REWARDS_POOL = 300_000_000;
    uint256 constant private _CAP = 1_000_000_000;
    uint8 constant private _POOLCOUNT = 10;
    uint256 constant private _MONTH = 30 days;
    uint256 constant private _DAY = 24 hours;
    uint256 constant private _HOUR = 60 minutes;
    uint256 constant private _MINUTE = 60 seconds;
    
    uint256 private _totalSupply;
    uint256 public _nextPayout;

    address public _owner;
    address public _multisigSafe;

    mapping(
        address => uint256) private _balances;
    mapping(
        address => mapping(
            address => uint256)) private _allowances;

    mapping(
        bytes32 => Stake) private _stakes;
    mapping(
        address => bytes32[]) private _stakeIdentifiers;

    bool public TGEtriggered;
    bool public initialized;
    uint256 public monthsPassed;

    struct Stake {
        address stakeholder;
        uint256 share;
        uint256 paid;
        uint8 pool; }

    struct PoolData {
        string name;
        address addr;
        uint256 tokens;
        uint256 vests;
        uint256 cliff; }

    PoolData[_POOLCOUNT] public _pool;

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * init
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/

         // owned by msg.sender
        // initializes contract
    function initialize(
    ) public initializer onlyUninitialized {

        _owner = _msgSender();

        _initializePools();

        uint256 sumTokens;
        // iterate through pools to create struct array
        for (uint8 i = 0; i < _POOLCOUNT; i++) {

            // here we are adding up tokens to make sure
            // sum is correct
            sumTokens += _pool[i].tokens;

            // in the same breath we convert token amounts
            // to ERC20 format
            _pool[i].tokens *= _DECIMAL_MAGNITUDE;
        }
        require(
            sumTokens == _CAP - _REWARDS_POOL,
            "pool token amounts must add up to cap less rewards");

        _totalSupply = 0;
        initialized = true;
        TGEtriggered = false; }

//***********************************/

    function _initializePools(
    ) internal {
        
        _pool[0] = PoolData({
            name: "Community Sale",
            addr: address(0),
            tokens: 3_703_703,
            vests: 3,
            cliff: 1
        });
        _pool[1] = PoolData({
            name: "Presale 1",
            addr: address(0),
            tokens: 48_626_667,
            vests: 18,
            cliff: 1
        });
        _pool[2] = PoolData({
            name: "Presale 2",
            addr: address(0),
            tokens: 33_333_333,
            vests: 15,
            cliff: 1
        });
        _pool[3] = PoolData({
            name: "Presale 3",
            addr: address(0),
            tokens: 25_714_286,
            vests: 12,
            cliff: 2
        });
        _pool[4] = PoolData({
            name: "Public Sale",
            addr: address(0),
            tokens: 28_500_000,
            vests: 3,
            cliff: 0
        });
        _pool[5] = PoolData({
            name: "Founders and Team",
            addr: address(0),
            tokens: 200_000_000,
            vests: 36,
            cliff: 1
        });
        _pool[6] = PoolData({
            name: "Outlier Ventures",
            addr: address(0),
            tokens: 40_000_000,
            vests: 24,
            cliff: 1
        });
        _pool[7] = PoolData({
            name: "Advisors",
            addr: address(0),
            tokens: 25_000_000,
            vests: 24,
            cliff: 1
        });
        _pool[8] = PoolData({
            name: "Interlock Foundation",
            addr: address(0),
            tokens: 258_122_011,
            vests: 84,
            cliff: 0
        });
        _pool[9] = PoolData({
            name: "Strategic Partners and KOL",
            addr: address(0),
            tokens: 37_000_000,
            vests: 12,
            cliff: 1
        }); }

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * modifiers
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/

        // only callable once, on initialization
    modifier onlyUninitialized(
    ) {
        require(
            !initialized,
            "can only initialize once");
        _; }

//***********************************/

        // only allows owner to call
    modifier onlyOwner(
    ) {
        require(
            _msgSender() == _owner,
            "only owner can call");
        _; }

//***********************************/

        // only allows the Safe wallet multisig safe to call
    modifier onlyMultisigSafe(
    ) {
        require(
            _msgSender() == _multisigSafe,
            "only multisig safe can call");
        _; }

//***********************************/

        // verifies zero address was not provied
    modifier noZero(
        address _address
    ) {
        require(
            _address != address(0),
            "zero address where it shouldn't be");
        _; }

//***********************************/

        // verifies there exists enough token to proceed
    modifier isEnough(
        uint256 _available,
        uint256 _amount
    ) {
        require(
            _available >= _amount,
            "not enough tokens available");
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
    ) public onlyOwner {

        require(
            initialized,
            "contract not initialized");
        require(
            !TGEtriggered,
            "TGE already happened");

        _multisigSafe = multisigSafe_;

        // create pool accounts and initiate
        for (uint8 i = 0; i < _POOLCOUNT; i++) {
            
            // generate pools and mint to
            address Pool = address(new ILOCKpool());
            _pool[i].addr = Pool;

            // mint to pools
            uint256 poolBalance = _pool[i].tokens;
            _balances[Pool] = poolBalance;
            emit Transfer(
                address(0),
                Pool,
                poolBalance); }

        // start the clock for time vault pools
        _nextPayout = block.timestamp + _MONTH;
        monthsPassed = 0;

        // approve owner to spend any tokens sent to this contract in future
        _approve(
            address(this),
            _msgSender(),
            _CAP * _DECIMAL_MAGNITUDE);

        // this must never happen again...
        TGEtriggered = true; }

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
    ) public onlyMultisigSafe {

        _owner = newOwner; }

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
    ) public onlyMultisigSafe {
        
        require(
            paused(),
            "already paused");
        _paused = true;
        
        emit Paused(_msgSender()); }

//***********************************/

        // resumes operation of functions requiring unpause
    function unpause(
    ) public onlyMultisigSafe {
        
        require(
            !paused(),
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

        // gets total tokens remaining in pools
    function reserve(
    ) public view returns (
        uint256 _reserve
    ) {
        return _CAP * _DECIMAL_MAGNITUDE - _totalSupply; }

//***********************************/

        // gets token cap
    function cap(
    ) public pure returns (
        uint256 _cap
    ) {
        return _CAP * _DECIMAL_MAGNITUDE; }

//***********************************/

        // gets relevant pool data
    function poolData(
        uint8 poolNumber
    ) public view returns (
        string memory poolName,
        address poolAddress,
        uint256 poolTokenSize,
        uint256 poolTokenBalance,
        uint256 poolTokensRemaining,
        uint256 vestingMonths,
        uint256 vestingCliff
    ) {
        PoolData memory thisPool = _pool[poolNumber];
        uint256 poolBalance = balanceOf(thisPool.addr);

        return (
            thisPool.name,
            thisPool.addr,
            thisPool.tokens,
            poolBalance,
            thisPool.tokens - poolBalance,
            thisPool.vests,
            thisPool.cliff); }

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
    ) internal virtual noZero(from) noZero(to) isEnough(_balances[from], amount) returns (
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
    ) internal virtual noZero(owner) noZero(spender) {

        require(
            !paused(),
            "contract is paused");
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
    ) internal virtual isEnough(allowance(owner, spender), amount) {
    
        uint256 currentAllowance = allowance(owner, spender);
        if (currentAllowance != type(uint256).max) {
            unchecked {
                _approve(
                    owner,
                    spender,
                    currentAllowance - amount);} } }

//***********************************/

          // emitting Approval event, reverting on failure
          // only callable by multisig safe
        // defines tokens directly available to spender from contract pool
    function approvePool(
        address spender,
        uint256 amount,
        uint8 poolNumber
    ) public onlyMultisigSafe returns (
        bool success
    ) {
        _approve(
            _pool[poolNumber].addr,
            spender,
            amount);
        return true; }

//***********************************/

        // allows client to safely execute approval facing double spend attack
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

        // allows client to safely execute approval facing double spend attack
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

          // where `from` && `to` != zero account => to be regular xfer
         // where `from` && `to` = zero account => impossible
        // hook that inserts behavior prior to transfer/mint/burn
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal virtual {

        from;
        to;
        amount;    
        require(
            !paused(),
            "contract is paused"); }

//***********************************/

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

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * stakeholder entry and distribution
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/

        // makes sure that distributions do not happen too early
    function _checkTime(
    ) public returns (
        bool isTime
    ) {
        // test time
        if (block.timestamp > _nextPayout) {

            // delta time between now and last payout
            uint256 deltaT = block.timestamp - _nextPayout;
            // calculate how many months to increment
            uint256 months = deltaT / _MONTH + 1;
            // increment next payout by months in seconds
            _nextPayout += _nextPayout + months * _MONTH;
            // increment months passed
            monthsPassed += months;

            // is time
            return true; }
        // is not time
        return false; }

//***********************************/

        // register stake
    function registerStake(
        Stake calldata data
    ) public onlyOwner returns (
        bool success
    ) {
        // generate stake identifier
        bytes32 stakeIdentifier = keccak256(
                                  bytes.concat(bytes20(data.stakeholder),
                                               bytes32(data.share),
                                               bytes1(data.pool) ) );
        // validate input
        require(
            !stakeExists(stakeIdentifier),
            "this stake already exists and cannot be edited");
        require(
            data.paid == 0,
            "amount paid must be zero");
        require(
            data.share >= _pool[data.pool].vests,
            "share is too small");
        require(
            data.pool < _POOLCOUNT,
            "invalid pool number");
        require(
            data.stakeholder != address(0),
            "stakeholder cannot be zero address");
        require(
            data.stakeholder != address(this),
            "stakeholder cannot be this contract address");

        // store stake
        _stakes[stakeIdentifier] = data;
        // store identifier for future iteration
        _stakeIdentifiers[data.stakeholder].push(stakeIdentifier);

        emit StakeRegistered(
			data);
        return true; }

//***********************************/

        // claim stake for vest periods accumulated
    function claimStake(
        bytes32 stakeIdentifier
    ) public returns (
        bool success
    ) {
        // see if we need to update time
        _checkTime();

        // if stake exists, then get it
        require(
            stakeExists(stakeIdentifier),
            "this stake does not exist");
        Stake storage stake = _stakes[stakeIdentifier];

		// define relevant stake values
        address stakeholder = stake.stakeholder;
        uint256 tokenShare = stake.share;
        uint256 tokensPaid = stake.paid;
        uint256 tokensRemaining = tokenShare - tokensPaid;
        uint256 cliff = _pool[stake.pool].cliff;
        uint256 vestingMonths = _pool[stake.pool].vests;

        // make sure cliff has been surpassed
        require(
            monthsPassed >= cliff,
            "too soon -- cliff not yet passed");
        // number of payouts must not surpass number of vests
        require(
            tokensPaid < tokenShare,
            "stakeholder already collected entire token share");
        
        // determine the traunch amount claimant has rights to for each vested month
        uint256 monthlyTokenAmount = tokenShare / vestingMonths;
        // and determine the number of payments claimant has received
        uint256 paymentsMade = tokensPaid / monthlyTokenAmount;

        // even if cliff is passed, is it too soon for next payment?
        require(
            paymentsMade <= monthsPassed - cliff,
            "payout too early");
        
        uint256 thesePayments;
        // when time has past vesting period, pay out remaining unclaimed payments
        if (cliff + vestingMonths <= monthsPassed) {
            
			// these are all remaining payments in this case
            thesePayments = vestingMonths - paymentsMade;

        // don't count months past payments made + cliff as payments
        } else {

			// these 
            thesePayments = 1 + monthsPassed - paymentsMade - cliff; }

        // use payments to calculate amount to pay out
        uint256 thisPayout = thesePayments * monthlyTokenAmount;

        // if at final payment, add remainder of share to final payment
        if (tokensRemaining - thisPayout < monthlyTokenAmount) {
            
            thisPayout += tokenShare % vestingMonths; }

        // transfer and make sure it succeeds
        require(
            _transfer(
                _pool[stake.pool].addr,
                stakeholder,
                thisPayout),
            "stake claim transfer failed");

        // update member state
        _stakes[stakeIdentifier].paid += thisPayout;
        // update total supply and reserve
        _totalSupply += thisPayout;

        emit StakeClaimed(
            stakeholder,
            stakeIdentifier,
            thisPayout);
        return true; }    

//*************************************************************/
//*************************************************************/
//*************************************************************/
    /**
    * stakeholder getters
    **/
//*************************************************************/
//*************************************************************/
//*************************************************************/

         // on a stake by stake basis
        // returns time remaining until next token traunch may be claimed
    function timeRemaining(
        bytes32 stakeIdentifier
    ) public view returns (
        uint256 monthsRemaining,
        uint256 daysRemaining,
        uint256 hoursRemaining,
        uint256 minutesRemaining,
        uint256 secondsRemaining
    ) {
        // if stake exists, then get it
        require(
            stakeExists(stakeIdentifier),
            "this stake does not exist");
        Stake memory stake = _stakes[stakeIdentifier];

		// define relevant stake values
        uint256 cliff = _pool[stake.pool].cliff;
        uint256 vests = _pool[stake.pool].vests;

        uint256 timeLeft;
        // compute the time left until the next payment is available
        // if months passed beyond last payment, stop counting
        if (monthsPassed >= vests + cliff ||
            _nextPayout < block.timestamp) {
            
            timeLeft = 0;

        // when cliff hasn't been surpassed, include that time into countdown
        } else if (monthsPassed < cliff) {
            
            timeLeft = (cliff - monthsPassed - 1) * _MONTH +
                        _nextPayout - block.timestamp;

        // during vesting period, timeleft is only time til next month's payment
        } else {

            timeLeft = _nextPayout - block.timestamp; }

        return parseTimeLeft(timeLeft); }

//***********************************/

        // breaks time left into human readable units for display on blockscanner
    function parseTimeLeft(
        uint256 timeLeft
    ) internal pure returns (
        uint256 monthsRemaining,
        uint256 daysRemaining,
        uint256 hoursRemaining,
        uint256 minutesRemaining,
        uint256 secondsRemaining
    ) {
        uint256 remainingSeconds;

        monthsRemaining = timeLeft / _MONTH;
        remainingSeconds = timeLeft % _MONTH;

        daysRemaining = remainingSeconds / _DAY;
        remainingSeconds = remainingSeconds % _DAY;

        hoursRemaining = remainingSeconds / _HOUR;
        remainingSeconds = remainingSeconds % _HOUR;

        minutesRemaining = remainingSeconds / _MINUTE;
        remainingSeconds = remainingSeconds % _MINUTE;

        secondsRemaining = remainingSeconds;

        return (
            monthsRemaining,
            daysRemaining,
            hoursRemaining,
            minutesRemaining,
            secondsRemaining); }

//***********************************/

            // get how much of amount left to pay is available to claim
           // get amount left to pay
          // get amount paid so far to member
         // get amount investor still needs to pay in before claiming tokens
        // get time remaining until next payout ready
    function stakeStatus(
        bytes32 stakeIdentifier
    ) public view returns (
        uint256 tokenShare,
        uint256 tokensPaidOut,
        uint256 tokensRemaining,
        uint256 tokensAvailable,
        uint256 monthlyTokenAmount,
        uint256 vestingMonths,
        uint256 cliff
    ) {
        // if stake exists, then get it
        require(
            stakeExists(stakeIdentifier),
            "this stake does not exist");
        Stake memory stake = _stakes[stakeIdentifier];
        cliff = _pool[stake.pool].cliff;
        vestingMonths = _pool[stake.pool].vests;
        tokenShare = stake.share;

        // how much has member already claimed
        tokensPaidOut = stake.paid;

        // determine the number of payments claimant has rights to
        monthlyTokenAmount = tokenShare / vestingMonths;

        // and determine the number of payments claimant has received
        uint256 payments = tokensPaidOut / monthlyTokenAmount;

        // how much does member have yet to collect, after vesting complete
        tokensRemaining = tokenShare - tokensPaidOut;

        // compute the pay available to claim at current moment
        // if months passed are inbetween cliff and end of vesting period
        if (monthsPassed >= cliff && monthsPassed < cliff + vestingMonths) {
            
            tokensAvailable = (1 + monthsPassed - cliff - payments) * monthlyTokenAmount;

        // until time reaches cliff, no pay is available
        } else if (monthsPassed < cliff ){

            tokensAvailable = 0;

        // if time has passed cliff and vesting period, the entire remaining share is available
        } else {

            tokensAvailable = tokenShare - tokensPaidOut; }

        // if at final payment, add remainder of share to final payment
        if (tokenShare - tokensPaidOut - tokensAvailable < monthlyTokenAmount && tokensAvailable > 0) {
            
            tokensAvailable += tokenShare % vestingMonths; }

        return (
            tokenShare,
            tokensPaidOut,
            tokensRemaining,
            tokensAvailable,
            monthlyTokenAmount,
            vestingMonths,
            cliff); }

//***********************************/

        // gets stake identifiers for stakes owned by message caller
    function getStakeIdentifiers(
        address stakeholder
    ) public view returns (
        bytes32[] memory stakeIdentifiers
    ) {
        return _stakeIdentifiers[stakeholder]; }

//***********************************/

        // gets stake designated by stake identifier
    function getStake(
        bytes32 stakeIdentifier
    ) public view returns (
        address stakeholder,
        uint256 share,
        uint256 paid,
        uint256 pool
    ) {
        Stake memory stake = _stakes[stakeIdentifier];
        return (
            stake.stakeholder,
            stake.share,
            stake.paid,
            stake.pool); }

//***********************************/

        // view predicate for validating getStake & claimStake input
    function stakeExists(
        bytes32 stakeIdentifier
    ) public view returns (
        bool exists
    ) {
        if (_stakes[stakeIdentifier].share > 0 &&
            _stakes[stakeIdentifier].stakeholder != address(0)) {
            
            // does exist
            return true; }
        // does not exist
        return false; }

//*************************************************************/
//*************************************************************/
//*************************************************************/

    function testingIncrementMonth(
    ) public returns (uint256) {

        monthsPassed += 1;
        _nextPayout += _MONTH;

        return monthsPassed; }

    uint256[100] public __gap;
}

//*************************************************************/
//*************************************************************/
//*************************************************************/



