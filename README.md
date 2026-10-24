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

---

## Part of Orkid Labs

**splits-distributor** — Interact with 0xSplits contracts from Rust — discover splits, check balances, execute distributions.

- Install: `cargo add splits-distributor`
- Package: https://crates.io/crates/splits-distributor
- API docs: https://docs.rs/splits-distributor
- Docs: https://docs.orkidlabs.xyz/packages/splits-distributor

Related Orkid projects: [defi-core](https://github.com/orkid-labs/defi-core)

Orkid Labs builds gasless, MEV-protected swap routing and agent-native DeFi infrastructure — [orkidlabs.xyz](https://orkidlabs.xyz) · [@orkid-labs on npm](https://www.npmjs.com/org/orkid-labs) · [github.com/orkid-labs](https://github.com/orkid-labs)
