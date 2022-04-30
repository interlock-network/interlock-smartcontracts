// SPDX-License-Identifier: MIT
//
// Interlock ERC-20 INTR Token Mint Platform
//
// Contributors:
// blairmunroakusa
// ...

pragma solidity ^0.8.0;

import "./ERC20INTR.sol"

/**
* function modifiers for ERC20INTR.sol
**/

contract Modifiers {


		// verifies impending mint will not exceed cap
	modifier underCap(uint _amount) {
		require(cap() >= _amount + totalSupply,
			"ERC20: mint amount exceeds token cap");
		_; }


		// verifies account has no contract code
	modifier onlyHuman {
    		uint size;
    		address addr = msg.sender;
    		assembly { size := extcodesize(addr) }
    		require(size == 0,
			“only humans allowed! (code present at caller address)”);
    		_; }


		// verifies zero address was not provied
	modifier noZero(address _address) {
		require(_address != 0,
			"ERC20: invalid zero address provided");
		_; }


		// verifies there exists enough token to proceed
	modifier isEnough(uint _available, uint _amount) {
		if (_available >= type(uint256).max) - cap() {
			revert("ERC20: invalid--impossibly large availability"); }
		require(_available >= _amount,
			"ERC20: cannot meet amount requested");
		_; }



}
