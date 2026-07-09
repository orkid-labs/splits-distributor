use crate::types;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use anyhow::Result;

/// Get the total token balance of a split (contract + warehouse)
pub async fn get_split_balance<P>(
    provider: &P,
    split_address: Address,
    token_address: Address,
) -> Result<(U256, U256)>
where
    P: Provider,
{
    // Try V2 SplitWalletV2 interface first
    let contract = types::SplitWalletV2::new(split_address, provider);
    match contract.getSplitBalance(token_address).call().await {
        Ok(balances) => Ok((balances.splitBalance, balances.warehouseBalance)),
        Err(_) => {
            // For V1 splits or non-V2 contracts, fall back to direct ERC20 balance check
            let erc20 = types::ERC20::new(token_address, provider);
            let balance = erc20.balanceOf(split_address).call().await?;
            Ok((balance, U256::ZERO))
        }
    }
}

/// Query split configuration from the contract via RPC
/// Note: V1 SplitMain contracts don't have a getSplit() function that returns split params.
/// V2 SplitWalletV2 contracts only store the split hash, not the full params.
/// This function returns None since split params are not retrievable from deployed contracts.
pub async fn get_split_config<P>(
    _provider: &P,
    _split_address: Address,
) -> Result<Option<crate::types::Split>>
where
    P: Provider,
{
    // Split parameters are not stored on-chain in a retrievable way
    // They must be obtained from the creation event or from external storage
    Ok(None)
}

/// Execute a distribution on a split contract (V2 only)
pub async fn execute_distribution<P>(
    provider: &P,
    contract_address: Address,
    token_address: Address,
    split: &types::Split,
    distributor_address: Address,
) -> Result<String>
where
    P: Provider,
{
    let contract = types::SplitWalletV2::new(contract_address, provider);
    let call = contract.distribute(split.clone(), token_address, distributor_address);

    let _result = call.call().await?;
    Ok("Distribution executed successfully".to_string())
}
