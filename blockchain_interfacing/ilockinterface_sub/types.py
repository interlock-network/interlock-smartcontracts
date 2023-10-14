""" This file contains types used by interface to ease the integration of blockchain into serverside application. """

from enum import Enum
from typing import NewType

Success = NewType("Success", bool | None)
Val = NewType("Val", int | float | str | list | tuple | range | bytes | bytearray | bool | set | dict | None)
Err = NewType("Err", object | None)

class Receipt(object):
    def __new__(cls, extrinsic_hash: str, block_hash: str):
        return {"extrinsic_hash": extrinsic_hash, "block_hash": block_hash}

class Status(Enum):
    Ok  = ":)"
    Err = ":("
    Unk = ":|"

class Errors(Enum):

    SmartcontractError = (
        100,
        "Smartcontract error.")
    AlreadyOrdered = (
        101,
        "There is already a multisig order in for that function, or by the caller.")
    AlreadyRegistered = (
        102,
        "The Interlocker stakeholder is already registered in the given pool.")
    AlreadySigned = (
        103,
        "The signatory caller has already signed the multisig order and cannot sign twice.")
    AlreadySignatory = (
        104,
        "The address given to add as a signatory already serves as a signatory.")
    CallerIsOwner = (
        105,
        "The contract owner is NOT permitted to use transfer functions to draw from supply pool.")
    CallerIsSignatory = (
        106,
        "Contract constructor may not input signatory that is constructor caller (owner).")
    CallerNotOperator = (
        107,
        "Application socket caller is not the port operator.")
    CallerNotOwner = (
        108,
        "Only the contract owner may call this function.")
    CallerNotSignatory = (
        109,
        "Only the owner or a contract signatory may call this function.")
    CannotReorder = (
        110,
        "The multisig transaction has already been ordered, or the orderer is attempting to order a transaction two times in a row, which is forbidden.")
    CannotRewardContract = (
        111,
        "Contracts are not eligible for receiving rewards.")
    CapTooLarge = (
        112,
        "Cap for port spending allowance exceeds the total rewards pool size.")
    CliffNotPassed = (
        113,
        "The vesting cliff has not yet passed, thus token distributions are not permitted at this time.")
    DivError = (
        114,
        "Checked division error.")
    DivideByZero = (
        115,
        "Divide by zero error.")
    InsufficientIlockBalance = (
        116,
        "The account in question does not have enough ILOCK to complete the transaction.")
    InvalidFunction = (
        117,
        "The input function is not a valid multisig function.")
    InvalidPool = (
        118,
        "The pool specified does not exist.")
    InvalidPort = (
        119,
        "The port number specified does not lie in the port number range.")
    IsZeroAddress = (
        120,
        "The zero address is forbidden as an argument.")
    NoPool = (
        121,
        "The specified pool does not exist.")
    NoPort = (
        122,
        "The specified port does not exist.")
    NoSignatory = (
        123,
        "The specified account address is NOT a signatory.")
    NoSocket = (
        124,
        "The specified socket does not exist.")
    NoStake = (
        125,
        "The specified account and pool does not exist as a stake.")
    NoTransaction = (
        126,
        "Not transaction has been ordered for specified multisig call function.")
    NotContract = (
        127,
        "Caller requesting socket creation is NOT an application contract.")
    NotEnoughSignatures = (
        128,
        "Multisig transaction signers have not passed signature count threshold.")
    Overflow = (
        129,
        "Checked add error.")
    PayoutTooEarly = (
        130,
        "The time elapsed since passing cliff has not passed vest threshold")
    PaymentTooLarge = (
        131,
        "Payment exceeds pool size.")
    PoolOutOfBounds = (
        132,
        "Specified pool index is out of bounds.")
    PortCapSurpassed = (
        133,
        "The allotted allowance for port ILOCK distribution has been exceeded.")
    PortExists = (
        134,
        "Port already exists.")
    PortLocked = (
        135,
        "Port is locked and may only be accessed by the Interlock Foundation.")
    ShareTooSmall = (
        136,
        "The Stake share is too small to allow safe division.")
    SignatoriesAreTheSame = (
        137,
        "Constructor signatory inputs are the same address.")
    StakeholderNotFound = (
        138,
        "The specified stakeholder does not exist in contract.")
    StakeholderSharePaid = (
        139,
        "The stakeholder is completely paid out.")
    TransactionAlreadyCalled = (
        140,
        "The multisig transaction has already been successfully called.")
    TransactionAlreadyOrdered = (
        141,
        "The transaction is ordered and waiting for signatures and call.")
    TransactionStale = (
        142,
        "Ordered multisig transaction has exceeded the timelimit imposed for signing and call.")
    Underflow = (
        143,
        "Checked subtraction error.")
    UnderThresholdMin = (
        144,
        "Cannot remove signatory or lower signature threshold any lower due to contract limits imposed.")
    UnderTimeMin = (
        145,
        "Cannot lower multisig sign and call time below contract imposed time limit minimum.")
    UnsafeContract = (
        146,
        "Contract attempting to create a socket with ILOCK contract has unrecognized codehash thus deemed unsafe.")
    WrongFunction = (
        147,
        "Multisig call function specified does not match the function specified in the transaction order.")
    #####################################################
    ContractsModuleError = (
        200,
        "Contracts Module error.")
    OutOfGas = (
        201,
        "The executed contract exhausted its gas limit.")
    TransferFailed = (
        202,
        "Performing the requested transfer failed. Probably because there isn't enough free balance in the sender's account.")
    MaxCallDepthReached = (
        203,
        "Performing a call was denied because the calling depth reached the limit of what is specified in the schedule.")
    ContractNotFound = (
        204,
        "No contract was found at the specified address.")
    CodeNotFound = (
        205,
        "No code could be found at the supplied code hash.")
    DecodingFailed = (
        206,
        "Input passed to a contract API function failed to decode as expected type.")
    ValueTooLarge = (
        207,
        "The size defined in `T::MaxValueSize` was exceeded.")
    DuplicateContract = (
        208,
        "A contract with the same AccountId already exists.")
    ReentranceDenied = (
        209,
        "A call tried to invoke a contract that is flagged as non-reentrant.")
    StorageDepositNotEnoughFunds = (
        210,
        "Origin doesn't have enough balance to pay the required storage deposits.")
    StorageDepositLimitExhausted = (
        211,
        "More storage was created than allowed by the storage deposit limit.")
    ContractReverted = (
        212,
        "The contract ran to completion but decided to revert its storage changes. Please note that this error is only returned from extrinsics. When called directly or via RPC an `Ok` will be returned. In this case the caller needs to inspect the flags to determine whether a reversion has taken place.")
    #####################################################
    SchedulerModuleError = (
        300,
        "Scheduler Module error.")
    FailedToSchedule = (
        301,
        "Failed to schedule a call.")
    #####################################################
    BalancesModuleError = (
        400,
        "Balances Module error.")
    InsufficientBalance = (
        401,
        "Balance too low to send value.")
    ExistentialDeposit = (
        402,
        "Value too low to create account due to existential deposit.")
    KeepAlive = (
        403,
        "Transfer/payment would kill account.")
    #####################################################
    UtilityModuleError = (
        500,
        "Utility Module error.")
    TooManyCalls = (
        501,
        "Too many calls batched.")
    #####################################################
    IdentityModuleError = (
        600,
        "Identity Module error.")
    NotFound = (
        601,
        "Account isn't found.")
    #####################################################
    BadOrigin = (
        700,
        "Bad origin.")
    CannotLookup = (
        800,
        "Cannot lookup.")
    #####################################################
    UnknownSmartcontractError = (
        27183,
        "Unknown Smartcontract error")
    UnknownModuleError = (
        141421,
        "Unknown Module error")
    OtherError = (
        314159,
        "Other error or failure or catastrophic meltdown.")
    # other errors VV
    #####################################################
    GasLimitExceeded = (
        314160,
        "The gas required determined by contract dry run exceeds specified limit.")
    InvalidAccountId = (
        314161,
        "The given AccountId is an invalid ss58 blockchain account address.")
    InvalidInputType = (
        314162,
        "The input value does not match the specified typehint for this function trait.")
#########################################################

class BlockchainResult(object):
    def __new__(cls, success: Success, error: Err, *values: Val, **kwvalues: Val):

        if success == True:

            if values == () and kwvalues == {}:
                raise Exception("Status ':)' MUST return value.")
            return {"status": Status.Ok.value,
                    "return": {
                        "values": values if values != () else None,
                        "kwvalues": kwvalues if kwvalues != {} else None
                    }}

        elif success == False:

            if "ContractExecResult" in type(error).__name__:
                error = str(error.contract_result_data[1][1])
                if error not in tuple(error.name for error in Errors):

                    return {"status": Status.Unk.value,
                            "return": {
                                "name": Errors.UnknownSmartcontractError.name,
                                "code": Errors.UnknownSmartcontractError.value[0],
                                "desc": Errors.UnknownSmartcontractError.value[1],
                                "data": error
                            }}
                return {"status": Status.Err.value,
                        "return": {
                            "name": Errors[error].name,
                            "code": Errors[error].value[0],
                            "desc": Errors[error].value[1]
                        }}

            elif "ContractExecutionReceipt" in type(error).__name__:
                error = error.error_message["name"]
                if error not in tuple(error.name for error in Errors):
                    return {"status": Status.Unk.value,
                            "return": {
                                "name": Errors.UnknownModuleError.name,
                                "code": Errors.UnknownModuleError.value[0],
                                "desc": Errors.UnknownModuleError.value[1],
                                "data": error
                            }}
                return {"status": Status.Err.value,
                        "return": {
                            "name": Errors[error].name,
                            "code": Errors[error].value[0],
                            "desc": Errors[error].value[1]
                        }}

            else:
                return {"status": Status.Unk.value,
                        "return": {
                            "name": Errors.OtherError.name,
                            "code": Errors.OtherError.value[0],
                            "desc": Errors.OtherError.value[1],
                            "data": dict(error)
                        }}

        elif success == None:

            return {"status": Status.Unk.value,
                    "return": {
                        "name": Errors.OtherError.name,
                        "code": Errors.OtherError.value[0],
                        "desc": Errors.OtherError.value[1],
                        "data": error
                    }}

def validate_address(address: str, substrate) -> bool | BlockchainResult:
    """
    Checks to see that input account address is valid ss58 blockchain address.

    returns tuple(bool, BlockchainResult)
    """
    if not substrate.is_valid_ss58_address(address):
        return (False, BlockchainResult(None, {
            "name": Errors["InvalidAccountId"].name,
            "code": Errors["InvalidAccountId"].value[0],
            "desc": Errors["InvalidAccountId"].value[1]
            }))
    return (True, None)

def validate_type(input_value, type_hint: str) -> bool | BlockchainResult:
    """
    Checks to see that input value matches specified typehint.

    returns tuple(bool, BlockchainResult)
    """
    if type(input_value).__name__ != type_hint:
        return (False, BlockchainResult(None, {
            "name": Errors["InvalidInputType"].name,
            "code": Errors["InvalidInputType"].value[0],
            "desc": Errors["InvalidInputType"].value[1]
            }))
    return (True, None)

