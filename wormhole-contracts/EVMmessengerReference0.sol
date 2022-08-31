//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

import "./Wormhole/IWormhole.sol";

contract Messenger {
    
	string private current_msg;
    
	address private wormhole_core_bridge_address = address(0xC89Ce4735882C9F0f0FE26686c53074E09B0D550);
    
	IWormhole core_bridge = IWormhole(wormhole_core_bridge_address);

    	// Used to calculate the Sequence for each message sent from this contract    
    	uint32 nonce = 0;
    
	// Chain ID => Application Contract mapping to ensure we only process messages from contracts we want to.
    	mapping(uint16 => bytes32) _applicationContracts;
    	address owner;
    
	// Track which messages we've already processed so we don't double process messages.
    	mapping(bytes32 => bool) _completedMessages;

/*******************************************/

	constructor(){
		owner = msg.sender;
	}


/*******************************************/

	function sendMsg(bytes memory str) public returns (uint64 sequence) {
    		
		sequence = core_bridge.publishMessage(nonce, str, 1);
    		nonce = nonce+1;
	}

/*******************************************/

	function receiveEncodedMsg(bytes memory encodedMsg) public {
    	(IWormhole.VM memory vm, bool valid, string memory reason) = core_bridge.parseAndVerifyVM(encodedMsg);
    
    		//1. Check Wormhole Guardian Signatures
    		//  If the VM is NOT valid, will return the reason it's not valid
    		//  If the VM IS valid, reason will be blank
    		require(valid, reason);

    		//2. Check if the Emitter Chain contract is registered
    		require(_applicationContracts[vm.emitterChainId] == vm.emitterAddress, "Invalid Emitter Address!");

    		//3. Check that the message hasn't already been processed
    		require(!_completedMessages[vm.hash], "Message already processed");
    		_completedMessages[vm.hash] = true;

    		//Do the thing
    		current_msg = string(vm.payload);
	}

/*******************************************/

	function getCurrentMsg() public view returns (string memory){
    
		return current_msg;
	}

/*******************************************/

	function registerApplicationContracts(uint16 chainId, bytes32 applicationAddr) public {
    		
		require(msg.sender == owner, "Only owner can register new chains!");
    		_applicationContracts[chainId] = applicationAddr;
	}

/*******************************************/

}
