""" This is a barebones module for barebones testing ilockinterface. """

from ilockinterface_sub.operations import reward_interlocker
from ilockinterface_sub.types import BlockchainResult

# Successful reward:
print(reward_interlocker(99900000000000000, "5HpLwg1MSy1ZbZESK1WmVn4H5Gt5fU7AAMUDDua3HLvBBopP"))

# this next one is a fresh account for storage limit test
# print(reward_interlocker(99900000000000000, "5GgurowEEXP5GgshyzEH9iHUwPNbjit3KBiuqRuL4AKH6eNM"))
    # To test Module Error, manually set ref_time in gas_limit of contract.exec, then recall above
    # To test Module Error Unknown, edit second BlockchainResult if-statement to miss "ContractExecutionReceipt"
    # and recall above

# verify that invalid account address is caught
#print(reward_interlocker(99900000000000000, "5HpLwg1MSy1ZbZESK1WmVn4H5Gt5fU7AAMUDDua3HLvBBop1"))

    #.
    #.
    #.

# Fail reward, smartcontract error
#print(reward_interlocker(99900000000000000, "5FjaH3F67MXdAFo2HK3Vr6sgUYeVpXPJPGPNG1zFtdC8751K"))
    # To check unknown smartcontract error, misspell 'CannotRewardContract' Errors enum label
    # and recall above













# Yes, I know there should be formal tests written. This is backburner unfortunately. Until then, I know my shit works. We debug in integration testing and production.
