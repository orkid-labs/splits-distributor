# Splits Distributor

Rust crate for interacting with 0xSplits contracts - split discovery, balance checking, and distribution execution.

## Features

- 0xSplits contract interaction (V1 and V2)
- Split discovery via Basescan scraping
- Balance checking (contract + warehouse)
- Distribution execution
- Configuration scraping from contract read sections

## Usage

```rust
use splits_distributor::{SplitsClient, scrape_split_addresses, get_split_balance};

// Discover split addresses from Basescan
let addresses = scrape_split_addresses()?;

// Check split balance
let (split_balance, warehouse_balance) = get_split_balance(&provider, split_address, token_address).await?;

// Execute distribution
let result = execute_distribution(&provider, contract_address, token_address, &split, distributor_address).await?;
```

## License

MIT
