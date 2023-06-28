'''
This script gets all active nominators and total stakes on Aleph Zero blockchain.

setup:
pip install substrate-interface
'''

from substrateinterface import SubstrateInterface

substrate = SubstrateInterface(url="wss://ws.azero.dev")

result = substrate.query_map("Staking", "Nominators")

i = 0
f = open("nominators.txt", "w")
for address in result:
    data = substrate.query("Staking", "Ledger", [address[0]])

    if data.value == None:

        data = substrate.query("Staking", "Ledger",
            [
                substrate.query("Staking", "Bonded", [address[0]])
            ]
        )

    print(str(i) + " of " + str(count) + " addresses")
    f.write(str(address[0]) + ":" + str(data.value["total"]) + "\n")
    i += 1

f.close()
