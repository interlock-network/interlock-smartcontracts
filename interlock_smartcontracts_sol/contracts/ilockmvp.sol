// INTERLOCK NETWORK ILOCK SOLIDITY CONTRACT

// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Contracts ^5.0.0

pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20CappedUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

contract InterlockNetwork is
    Initializable,
    ERC20Upgradeable,
    ERC20PausableUpgradeable,
    ERC20CappedUpgradeable,
    OwnableUpgradeable
{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address initialOwner) public initializer {
        uint256 CAP = 1_000_000_000 * 10 ** decimals();
        uint256 ARBITRUM_MINT = 700_000_000 * 10 ** decimals();

        __ERC20_init("InterlockNetwork", "ILOCK");
        __ERC20Pausable_init();
        __ERC20Capped_init(CAP);
        __Ownable_init(initialOwner);

        _mint(address(this), ARBITRUM_MINT);
        _approve(address(this), initialOwner, CAP);
        _pause();
    }

    function treasuryApprove(address spender, uint256 value) public onlyOwner {
        _approve(address(this), spender, value);
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    function _update(
        address from,
        address to,
        uint256 value
    )
        internal
        override(
            ERC20Upgradeable,
            ERC20PausableUpgradeable,
            ERC20CappedUpgradeable
        )
    {
        super._update(from, to, value);
    }

    /// @dev Gap for upgradeable storage. */
    uint256[100] public storageGap;
}
