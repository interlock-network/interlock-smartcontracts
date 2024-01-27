"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
async function signWhitelist(chainId, contractAddress, whitelistKey, mintingAddress) {
    // Domain data should match whats specified in the DOMAIN_SEPARATOR constructed in the contract
    // https://github.com/msfeldstein/EIP712-whitelisting/blob/main/contracts/EIP712Whitelisting.sol#L33-L43
    const domain = {
        name: "Validator",
        version: "1",
        chainId: 1,
        verifyingContract: contractAddress,
    };
    // The types should match the TYPEHASH specified in the contract
    // https://github.com/msfeldstein/EIP712-whitelisting/blob/main/contracts/EIP712Whitelisting.sol#L27-L28
    const types = {
        Validation: [
            { name: "wallet", type: "address" },
            { name: "share", type: "uint256" },
            { name: "pool", type: "uint8" },
        ],
    };
    const sig = await whitelistKey._signTypedData(domain, types, {
        wallet: "0xeeBA65D9C7E5832918d1F4277DE0a78b78efEC43",
        share: 1000,
        pool: 5
    });
    console.log(sig);
}
exports.default = signWhitelist;
signWhitelist();
