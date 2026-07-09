use alloy::primitives::Address;
use alloy::sol;

// 0xSplits contract addresses on Base Alloy sol! macro

sol! {
    /// @notice Split configuration struct
    #[derive(Debug)]
    struct Split {
        address[] recipients;
        uint256[] allocations;
        uint256 totalAllocation;
        uint16 distributionIncentive;
    }

    #[sol(rpc)]
    contract SplitWalletV2 {
        /// @notice Distributes the tokens in the split to recipients through the warehouse.
        function distribute(
            Split calldata _split,
            address _token,
            address _distributor
        ) external;

        /// @notice Gets the total token balance of the split wallet and the warehouse.
        function getSplitBalance(address _token) external view returns (uint256 splitBalance, uint256 warehouseBalance);

        /// @notice the split hash - Keccak256 hash of the split struct
        function splitHash() external view returns (bytes32);
    }

    #[sol(rpc)]
    contract SplitFactoryV2 {
        /// @notice Emitted when a new Split is created
        event SplitCreated(
            address indexed split,
            Split splitParams,
            address owner,
            address creator
        );

        /// @notice Create a new split with params and owner.
        function createSplit(
            Split calldata _splitParams,
            address _owner,
            address _creator
        ) external returns (address split);
    }

    #[sol(rpc)]
    contract SplitMain {
        /// @notice Emitted when a new Split is created (V1)
        event CreateSplit(address indexed split);

        /// @notice Creates a new split with recipients `accounts` with ownerships `percentAllocations`, a keeper fee for splitting of `distributorFee` and the controlling address `controller`
        function createSplit(
            address[] calldata accounts,
            uint32[] calldata percentAllocations,
            uint32 distributorFee,
            address controller
        ) external returns (address split);

        /// @notice Distributes the ETH balance for split `split`
        function distributeETH(
            address split,
            address[] calldata accounts,
            uint32[] calldata percentAllocations,
            uint32 distributorFee,
            address distributorAddress
        ) external;

        /// @notice Distributes the ERC20 token balance for split `split`
        function distributeERC20(
            address split,
            address token,
            address[] calldata accounts,
            uint32[] calldata percentAllocations,
            uint32 distributorFee,
            address distributorAddress
        ) external;
    }

    #[sol(rpc)]
    contract ERC20 {
        function balanceOf(address owner) external view returns (uint256);
    }
}

// The sol! macro generates these types in the current module's namespace
// They can be used directly as types::Split, types::SplitFactoryV2, etc.
// Source: https://github.com/0xSplits/splits-sdk/blob/main/packages/splits-sdk/src/constants/index.ts
pub const SPLIT_MAIN_BASE: &str = "0x2ed6c4B5dA6378c7897AC67Ba9e43102Feb694EE";
pub const PULL_SPLIT_FACTORY_BASE: &str = "0x80f1B766817D04870f115fEBbcCADF8DBF75E017";
pub const PUSH_SPLIT_FACTORY_BASE: &str = "0xaDC87646f736d6A82e9a6539cddC488b2aA07f38";
pub const WAREHOUSE_BASE: &str = "0x8fb66F38cF86A3d5e8768f8F1754A24A6c661Fb8";

// Token addresses on Base
pub const WETH_BASE: &str = "0x4200000000000000000000000000000000000006";
pub const USDC_BASE: &str = "0x833589fCD6eDb6E08f4c7C32D4f71b54bA0293EB";
pub const USDBC_BASE: &str = "0xd9aAEc86B65D86f6A7B5B1b0c42FFA531710b6CA";
pub const ETH_SENTINEL: &str = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"; // Native ETH sentinel in 0xSplits warehouse

/// Split configuration
#[derive(Clone)]
pub struct SplitConfig {
    pub address: Address,
    pub split: Split,
    pub owner: Address,
    pub creator: Address,
}

/// Monitored token for balance checking
#[derive(Clone, Debug)]
pub struct MonitoredToken {
    pub address: &'static str,
    pub symbol: &'static str,
    pub decimals: u8,
    pub price_usd: f64,
}
