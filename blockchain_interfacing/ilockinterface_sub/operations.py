"""
Interlock Network blockchain integration operations.

Docs in module README.
Type and class definitions in types.py.
"""

from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException
from substrateinterface.contracts import ContractInstance
from ilockinterface_sub.types import BlockchainResult, Receipt, Errors, validate_address, validate_type
import logging
import os

from dotenv import load_dotenv
load_dotenv("./.env.blockchain.sub")

OWNER_ADDRESS = os.getenv("OWNER_ADDRESS")
MNEMONIC_PHRASE = os.getenv("MNEMONIC_PHRASE")
WEBSOCKET_URL = os.getenv("WEBSOCKET_URL")
CONTRACT_ADDRESS = os.getenv("CONTRACT_ADDRESS")
CONTRACT_METADATA = os.getenv("CONTRACT_METADATA")

# gas limit object components
REF_TIME_LIMIT = int(os.getenv("REF_TIME_LIMIT"))
PROOF_SIZE_LIMIT = int(os.getenv("PROOF_SIZE_LIMIT"))
STORAGE_DEPOSIT_LIMIT = int(os.getenv("STORAGE_DEPOSIT_LIMIT"))

# establish connection with Aleph Zero blockchain
substrate = SubstrateInterface(url=WEBSOCKET_URL)
# establish credential keypair for contract OWNER, for signing
keypair = Keypair.create_from_mnemonic(MNEMONIC_PHRASE)

# establish ILOCK contract object
contract = ContractInstance.create_from_address(
    contract_address=CONTRACT_ADDRESS,
    metadata_file=CONTRACT_METADATA,
    substrate=substrate
)

def reward_interlocker(reward: int, interlocker: str) -> BlockchainResult:
    """
    Airdrops reward of specified amount to specified Interlocker.

    return values = (Balance(new_rewarded_total))
           kwvalues = {Receipt(extrinsic_hash, block_hash)}
    """
    # first validate input types and address format
    reward_check = validate_type(reward, "int")
    if reward_check[0] == False:
        return reward_check[1]
    interlocker_check = validate_type(interlocker, "str")
    if interlocker_check[0] == False:
        return interlocker_check[1]
    address_check = validate_address(interlocker, substrate)
    if address_check[0] == False:
        return address_check[1]

    # now do the things
    try:
        # perform dry run
        dryrun_result = contract.read(
            keypair=keypair,
            method="reward_interlocker",
            args={
                "reward": reward,
                "interlocker": interlocker,
            })

        # isolate dry run results
        success = True if dryrun_result.contract_result_data[1][0] == "Ok" else False
        gas_required = dryrun_result.gas_required 
        storage_required = dryrun_result["storage_deposit"][1]

        # make sure this transaction won't break the bank due to degraded netowrk conditions, etc
        if gas_required["ref_time"] > REF_TIME_LIMIT or \
           gas_required["proof_size"] > PROOF_SIZE_LIMIT:
            return BlockchainResult(None, {
                "name": Errors["GasLimitExceeded"].name,
                "code": Errors["GasLimitExceeded"].value[0],
                "desc": Errors["GasLimitExceeded"].value[1]
                })
        if storage_required > STORAGE_DEPOSIT_LIMIT:
            return BlockchainResult(None, {
                "name": Errors["StorageLimitExceeded"].name,
                "code": Errors["StorageLimitExceeded"].value[0],
                "desc": Errors["StorageLimitExceeded"].value[1]
                })

        # return smartcontract error if dryrun fail
        if not success:
            return BlockchainResult(success, dryrun_result)

        # establish return value for successful extrinsic execution
        new_rewarded_total = int(format(dryrun_result.contract_result_data[1][1]))
        
        # now execute actual call is dryrun success
        try:
            exec_receipt = contract.exec(
                keypair=keypair,
                method="reward_interlocker",
                args={
                    "reward": reward,
                    "interlocker": interlocker,
                },
                gas_limit={
                    "ref_time": REF_TIME_LIMIT,
                    "proof_size":PROOF_SIZE_LIMIT
                },
                storage_deposit_limit=1, #STORAGE_DEPOSIT_LIMIT,
                wait_for_finalization=True)

            # executed extrinsic succeeds happy
            if exec_receipt.is_success:
                return_receipt = Receipt(exec_receipt.extrinsic_hash, exec_receipt.block_hash)
                return BlockchainResult(success, None, new_rewarded_total, return_receipt)

            # executed extrinsic succeeds sad, pass whole exec receipt to BlockchainResult for parsing
            else:
                return BlockchainResult(not success, exec_receipt)

        # executed extrinsic fails
        except SubstrateRequestException as error:
            logging.exception(error)
            return BlockchainResult(None, f"{format(error)}")

    # catch all other exceptions
    except Exception as e:
        logging.exception(e)
        return e



