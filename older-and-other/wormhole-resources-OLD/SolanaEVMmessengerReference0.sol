
// A CLIENTSIDE LOOK AT TOKEN TRANSFER FROM ETHEREUM TO SOLANA

/*************************************************/

// get Solana associated token account

import {
	Token,
	ASSOCIATED_TOKEN_PROGRAM_ID,
	TOKEN_PROGRAM_ID
} from '@solana/spl-token';
import {
	getForeignAssetSolana,
	hexToUint8Array,
	nativeToHexString,
	CHAIN_ID_ETH,
} from '@certusone/wormhole-sdk';

const SOLANA_TOKEN_BRIDGE_ADDRESS = "wormDTUJ6AWPNvk59vGQbDvGJmqbDTdgWgAqcLBCgUb";

// determine destination address - an associated token account
const solanaMintKey = new PublicKey(
 	(await getForeignAssetSolana(
  	connection,
    	SOLANA_TOKEN_BRIDGE_ADDRESS,
    	CHAIN_ID_ETH,
    	hexToUint8Array(nativeToHexString(tokenAddress, CHAIN_ID_ETH) || "")
  	)) || ""
);

const recipientAddress = await Token.getAssociatedTokenAddress(
  	ASSOCIATED_TOKEN_PROGRAM_ID,
  	TOKEN_PROGRAM_ID,
  	solanaMintKey,
  	recipientWalletAddress
);

/*************************************************/

// submit transfer message on ethereum

import {
	trasnferFromEth,
	parseSequenceFromLogEth,
	getEmitterAddressEth,
	CHAIN_ID_SOLANA,
} from '@certusone/wormhole-sdk';

const ETH_TOKEN_BRIDGE_ADDRESS = "0x3ee18B2214AFF97000D974cf647E7C347E8fa585";

// Submit transaction - results in a Wormhole message being published
const receipt = await transferFromEth(
  	ETH_TOKEN_BRIDGE_ADDRESS,
  	signer,
  	tokenAddress,
  	amount,
  	CHAIN_ID_SOLANA,
  	recipientAddress
);

// Get the sequence number and emitter address required to fetch the signedVAA of our message
const sequence = parseSequenceFromLogEth(receipt, ETH_BRIDGE_ADDRESS);

const emitterAddress = getEmitterAddressEth(ETH_TOKEN_BRIDGE_ADDRESS);

/*************************************************/

// fetch signed message from guardians

import {
	getSignedVAA,
} from '@certusone/wormhole-sdk';

// Fetch the signedVAA from the Wormhole Network (this may require retries while you wait for confirmation)
const { signedVAA } = await getSignedVAA(
  	WORMHOLE_RPC_HOST,
  	CHAIN_ID_ETH,
  	emitterAddress,
  	sequence
);

/*************************************************/

// post to solana to mint 

const SOL_BRIDGE_ADDRESS = "worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth";

// On Solana, we have to post the signedVAA ourselves
await postVaaSolana(
  	connection, // Solana Mainnet Connection
  	wallet,      //Solana Wallet Signer
  	SOL_BRIDGE_ADDRESS,
  	payerAddress,
  	signedVAA
);

/*************************************************/

// Finally, redeem on Solana
const transaction = await redeemOnSolana(
  	connection,
  	SOL_BRIDGE_ADDRESS,
  	SOL_TOKEN_BRIDGE_ADDRESS,
  	payerAddress,
  	signedVAA,
  	isSolanaNative,
  	mintAddress
);

const signed = await wallet.signTransaction(transaction);

const txid = await connection.sendRawTransaction(signed.serialize());

await connection.confirmTransaction(txid);
