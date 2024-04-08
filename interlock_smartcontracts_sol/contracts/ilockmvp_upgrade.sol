// INTERLOCK NETWORK ILOCK SOLIDITY CONTRACT

// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Contracts ^5.0.0

pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20CappedUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts/interfaces/draft-IERC6093.sol";

contract InterlockNetworkUpgrade is
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
    }

    /// @dev only the multisig owner can approve/disapprove spending from contract token treasury
    modifier contractOwnerApprovalCheck(address owner) {
        if (owner == address(this)) {
            if (_msgSender() != super.owner()) {
                revert ERC20InvalidApprover(_msgSender());
            }
        }
        _;
    }

    /// @dev only the multisig owner can issue transfer from contract token treasury
    modifier contractOwnerTransferCheck(address owner) {
        if (owner == address(this)) {
            if (_msgSender() != super.owner()) {
                revert ERC20InvalidSpender(_msgSender());
            }
        }
        _;
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }

    function approve(
        address owner,
        address spender,
        uint256 value
    ) public contractOwnerApprovalCheck(owner) {
        super._approve(owner, spender, value, true);
    }

    function transferFrom(
        address owner,
        address spender,
        uint256 value
    )
        public
        override(ERC20Upgradeable)
        contractOwnerTransferCheck(owner)
        returns (bool)
    {
        return super.transferFrom(owner, spender, value);
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

    function newFeature() public pure returns (string memory) {
        return "new feature";
    }

    uint256 public newstorage;
    uint256[99] public storageGap;
}
