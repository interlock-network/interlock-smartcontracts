// SPDX-License-Identifier: MIT
//
// Interlock ERC-20 ILOCK Token Mint Platform
// 		(containing)
// excerpts from OpenZeppelin v4.6.0 contract (token/ERC20/IERC20.sol)
//
// Contributors:
// blairmunroakusa
// ...

pragma solidity ^0.8.0;

/**
* Interface of ERC20 standard + metadata as defined in the EIP.
**/

interface IERC20 {

   	/**
     	* events
     	**/
		// `value` may be zero.
    	event Transfer(
		address indexed from,
		address indexed to,
		uint256 value );

		// `value` is the new allowance.
    	event Approval(
		address indexed owner,
		address indexed spender,
		uint256 value );

    		// This event is triggered whenever a call to #claim succeeds.
    	event Claimed(
		uint256 index,
		address account,
		uint256 share,
		uint8 pool );	

	event Repaid(
		address indexed creditor,
		uint256 value );

    	/**
     	* getter methods
     	**/

		// gets token name (Interlock Network)
	function name() external view returns (string memory);


		// gets token symbol (ILOCK)
    	function symbol() external view returns (string memory);


		// gets token decimals (18)
    	function decimals() external view returns (uint8);


 		// gets tokens minted so far (total tokens)
    	function totalSupply() external view returns (uint256);


		// gets account balance (tokens payable)
    	function balanceOf(
		address account
	) external view returns (uint256);


		// gets tokens approved (tokens spendable)
    	function allowance(
		address owner,
		address spender
	) external view returns (uint256);

    	/**
     	* doer methods
     	**/
		// emits Transfer, returns true on success
   	function transfer(
		address to,
		uint256 amount
	) external returns (bool);


		// emits Approval, returns true on success 
	function approve(
		address spender,
		uint256 amount
	) external returns (bool);


		// emit Transfer, returns true on success
    	function transferFrom(
        	address from,
        	address to,
        	uint256 amount
    	) external returns (bool);
}

    /**
     * IMPORTANT: Beware that changing an allowance with approve() runs the risk
     * that someone may use both the old and the new allowance by unfortunate
     * transaction ordering. One possible solution to mitigate this race
     * condition is to first reduce the spender's allowance to 0 and set the
     * desired value afterwards:
     * https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
     */

