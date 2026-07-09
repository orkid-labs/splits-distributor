//! 0xSplits Distributor Crate
//!
//! A Rust crate for interacting with 0xSplits contracts as a distributor.
//! Provides split discovery, balance checking, and distribution execution functionality.

pub mod client;
pub mod discovery;
pub mod distribution;
pub mod scraper;
pub mod types;

pub use client::SplitsClient;
pub use discovery::{discover_splits_from_factory, discover_splits_from_splitmain};
pub use distribution::{execute_distribution, get_split_balance, get_split_config};
pub use scraper::{scrape_split_addresses, scrape_split_config};
pub use types::{MonitoredToken, SplitConfig, WETH_BASE, USDC_BASE, USDBC_BASE, ETH_SENTINEL, PULL_SPLIT_FACTORY_BASE, PUSH_SPLIT_FACTORY_BASE, SPLIT_MAIN_BASE, Split};
// Re-export Address to ensure type consistency across crates
pub use alloy::primitives::Address;
