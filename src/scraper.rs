use crate::types::{SPLIT_MAIN_BASE, PULL_SPLIT_FACTORY_BASE, PUSH_SPLIT_FACTORY_BASE, Split};
use alloy::primitives::Address;
use anyhow::Result;
use scraper::{Html, Selector};
use std::time::Duration;

const BASESCAN_URL: &str = "https://basescan.org";
const REQUEST_DELAY_MS: u64 = 1500; // 1.5 seconds between requests

/// Scrape Basescan for split contract addresses from factory transactions
pub fn scrape_split_addresses() -> Result<Vec<Address>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .timeout(Duration::from_secs(30))
        .build()?;

    let mut all_addresses = Vec::new();

    // Scrape SplitMain (V1)
    println!("🔍 Scraping SplitMain transactions from Basescan...");
    match scrape_contract_transactions(&client, SPLIT_MAIN_BASE) {
        Ok(addresses) => {
            println!("  ✓ Found {} split addresses from SplitMain", addresses.len());
            all_addresses.extend(addresses);
        }
        Err(e) => {
            println!("  ⚠️ Failed to scrape SplitMain: {}", e);
        }
    }

    std::thread::sleep(Duration::from_millis(REQUEST_DELAY_MS));

    // Scrape PullSplitFactory (V2)
    println!("🔍 Scraping PullSplitFactory transactions from Basescan...");
    match scrape_contract_transactions(&client, PULL_SPLIT_FACTORY_BASE) {
        Ok(addresses) => {
            println!("  ✓ Found {} split addresses from PullSplitFactory", addresses.len());
            all_addresses.extend(addresses);
        }
        Err(e) => {
            println!("  ⚠️ Failed to scrape PullSplitFactory: {}", e);
        }
    }

    std::thread::sleep(Duration::from_millis(REQUEST_DELAY_MS));

    // Scrape PushSplitFactory (V2)
    println!("🔍 Scraping PushSplitFactory transactions from Basescan...");
    match scrape_contract_transactions(&client, PUSH_SPLIT_FACTORY_BASE) {
        Ok(addresses) => {
            println!("  ✓ Found {} split addresses from PushSplitFactory", addresses.len());
            all_addresses.extend(addresses);
        }
        Err(e) => {
            println!("  ⚠️ Failed to scrape PushSplitFactory: {}", e);
        }
    }

    // Deduplicate addresses
    all_addresses.sort();
    all_addresses.dedup();

    println!("✓ Total unique split addresses found: {}", all_addresses.len());
    Ok(all_addresses)
}

/// Scrape transaction hashes from a contract's Basescan page
fn scrape_contract_transactions(client: &reqwest::blocking::Client, contract_address: &str) -> Result<Vec<Address>> {
    let url = format!("{}/address/{}", BASESCAN_URL, contract_address);

    let response = client.get(&url).send()?;
    let html = response.text()?;
    let document = Html::parse_document(&html);

    // Extract transaction hashes from the transactions table
    let tx_selector = match Selector::parse("a.hash-link") {
        Ok(sel) => sel,
        Err(e) => return Err(anyhow::anyhow!("Failed to parse tx selector: {}", e)),
    };
    let mut tx_hashes = Vec::new();

    for element in document.select(&tx_selector) {
        if let Some(tx_hash) = element.value().attr("href") {
            // Extract hash from URL like "/tx/0x123..."
            if let Some(hash) = tx_hash.strip_prefix("/tx/") {
                tx_hashes.push(hash.to_string());
            }
        }
    }

    println!("  Found {} transactions, extracting split addresses...", tx_hashes.len());

    // Limit to first 20 transactions to avoid rate limiting
    let tx_hashes: Vec<_> = tx_hashes.into_iter().take(20).collect();

    let mut split_addresses = Vec::new();
    for (i, tx_hash) in tx_hashes.iter().enumerate() {
        if i > 0 && i % 5 == 0 {
            std::thread::sleep(Duration::from_millis(REQUEST_DELAY_MS));
        }

        match scrape_transaction_for_split(client, tx_hash) {
            Ok(Some(address)) => {
                split_addresses.push(address);
            }
            Ok(None) => {
                // Transaction doesn't create a split, skip
            }
            Err(e) => {
                println!("  ⚠️ Failed to scrape transaction {}: {}", tx_hash, e);
            }
        }
    }

    Ok(split_addresses)
}

/// Scrape a transaction page to extract the created split contract address
fn scrape_transaction_for_split(client: &reqwest::blocking::Client, tx_hash: &str) -> Result<Option<Address>> {
    let url = format!("{}/tx/{}", BASESCAN_URL, tx_hash);

    let response = client.get(&url).send()?;
    let html = response.text()?;
    let document = Html::parse_document(&html);

    // Look for contract address in the page - simpler approach
    // Look for addresses that might be created contracts
    let address_selector = match Selector::parse("a[href^='/address/']") {
        Ok(sel) => sel,
        Err(e) => return Err(anyhow::anyhow!("Failed to parse address selector: {}", e)),
    };
    for element in document.select(&address_selector) {
        if let Some(href) = element.value().attr("href") {
            if let Some(address_str) = href.strip_prefix("/address/") {
                // Filter out common addresses (sender, factory, etc.)
                if address_str.len() == 42 && address_str.starts_with("0x") {
                    // This could be a created contract address
                    if let Ok(address) = address_str.parse::<Address>() {
                        // Additional heuristic: check if this appears in a "Contract Created" context
                        let text: String = element.text().collect();
                        if text.to_lowercase().contains("contract") || text.to_lowercase().contains("created") {
                            return Ok(Some(address));
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}

/// Scrape split configuration from Basescan contract tab
pub fn scrape_split_config(client: &reqwest::blocking::Client, split_address: Address) -> Result<Option<Split>> {
    let url = format!("{}/address/{}#code", BASESCAN_URL, split_address);

    let response = client.get(&url).send()?;
    let html = response.text()?;
    let document = Html::parse_document(&html);

    // Look for the "Read Contract" section which shows public variables
    // The split parameters should be visible as readable state variables

    // Try to find recipients array
    let recipients = extract_contract_array(&document, "recipients")?;

    // Try to find allocations array
    let allocations = extract_contract_array(&document, "allocations")?;

    // Try to find distribution incentive
    let distribution_incentive = extract_contract_uint(&document, "distributionIncentive")?;

    // Try to find total allocation
    let total_allocation = extract_contract_uint(&document, "totalAllocation")?;

    if recipients.is_empty() || allocations.is_empty() {
        return Ok(None);
    }

    // Parse addresses and convert to alloy::primitives::fixed_bytes::Address
    let recipient_addresses: Vec<Address> = recipients
        .into_iter()
        .filter_map(|s| s.parse::<Address>().ok())
        .collect();

    // Parse allocations as u256
    let allocation_values: Vec<alloy::primitives::U256> = allocations
        .into_iter()
        .filter_map(|s| {
            // Handle hex strings like "0x..." or decimal strings
            if s.starts_with("0x") {
                u256_from_hex(&s)
            } else {
                u256_from_dec(&s)
            }
        })
        .collect();

    if recipient_addresses.len() != allocation_values.len() {
        return Ok(None);
    }

    // Convert distribution incentive from U256 to u16
    let incentive_u16: u16 = distribution_incentive
        .to::<u128>()
        .try_into()
        .unwrap_or(0);

    Ok(Some(Split {
        recipients: recipient_addresses,
        allocations: allocation_values,
        totalAllocation: total_allocation,
        distributionIncentive: incentive_u16,
    }))
}

/// Extract an array variable from contract read section
fn extract_contract_array(document: &Html, var_name: &str) -> Result<Vec<String>> {
    let mut values = Vec::new();

    // Try to find by row in the contract read table
    let table_selector = match Selector::parse("table.table tbody tr") {
        Ok(sel) => sel,
        Err(e) => return Err(anyhow::anyhow!("Failed to parse table selector: {}", e)),
    };

    for row in document.select(&table_selector) {
        let row_text: String = row.text().collect();
        if row_text.contains(var_name) {
            // Extract array from this row
            if let Some(start) = row_text.find('[') {
                if let Some(end) = row_text.find(']') {
                    let array_str = &row_text[start + 1..end];
                    for item in array_str.split(',') {
                        let item = item.trim();
                        if !item.is_empty() {
                            values.push(item.to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(values)
}

/// Extract a uint variable from contract read section
fn extract_contract_uint(document: &Html, var_name: &str) -> Result<alloy::primitives::U256> {
    let table_selector = match Selector::parse("table.table tbody tr") {
        Ok(sel) => sel,
        Err(e) => return Err(anyhow::anyhow!("Failed to parse table selector: {}", e)),
    };

    for row in document.select(&table_selector) {
        let row_text: String = row.text().collect();
        if row_text.contains(var_name) {
            // Try to extract the value
            let parts: Vec<&str> = row_text.split_whitespace().collect();
            for part in parts {
                if part.starts_with("0x") {
                    return Ok(u256_from_hex(part).unwrap_or(alloy::primitives::U256::ZERO));
                } else if part.chars().all(|c| c.is_digit(10)) {
                    return Ok(u256_from_dec(part).unwrap_or(alloy::primitives::U256::ZERO));
                }
            }
        }
    }

    Ok(alloy::primitives::U256::ZERO)
}

fn u256_from_hex(s: &str) -> Option<alloy::primitives::U256> {
    let s = s.trim_start_matches("0x");
    alloy::primitives::U256::from_str_radix(s, 16).ok()
}

fn u256_from_dec(s: &str) -> Option<alloy::primitives::U256> {
    alloy::primitives::U256::from_str_radix(s, 10).ok()
}
