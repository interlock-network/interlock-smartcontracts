// SPDX-License-Identifier: MIT
//
// Interlock ERC-20 INTR Token Mint Platform
//
// Contributors:
// blairmunroakusa
// ...

pragma solidity ^0.8.0;

import "./ERC20INTR.sol";

contract INTRpublicsale {

    	address _mintAddress;
    	ERC20INTR public mint;

    	constructor (address mintAddress_, uint8 paymentsTotal_) {
        	_mintAddress = mintAddress_;
        	mint = ERC20INTR(_mintAddress);
		_patmentsTotal = paymentsTotal_;
		_paymentsMade = 0; }


	function splitPool(uint256 share, address member) public {
        require(mint.balanceOf(member) == 0, "already added");
        mint.approve(member, share); }

}
	

