use alloy::providers::Provider;
use alloy::transports::Transport;
use alloy::network::Ethereum;
use alloy::primitives::{Address, U256};
use alloy::rpc::types::eth::TransactionRequest;
use alloy::providers::network::TransactionBuilder;
use std::str::FromStr;

pub fn calculate_leverage_multiplier(sentiment: f64) -> f64 {
    1.0 + sentiment
}

pub async fn execute_flash_loan_tx<P, T>(
    provider: &P,
    target: &str,
    _leverage: f64,
) -> Result<String, Box<dyn std::error::Error>> 
where
    T: Transport + Clone,
    P: Provider<T, Ethereum>,
{
    // For MVP, we construct a real transaction to a dummy Liquidator contract.
    let liquidator_contract = Address::from_str("0x0000000000000000000000000000000000000003")?;
    let _target_addr = Address::from_str(target).unwrap_or(Address::ZERO);

    // Build the transaction
    let tx = TransactionRequest::default()
        .with_to(liquidator_contract)
        .with_value(U256::from(0)); // We are just demonstrating the capability to broadcast a TX

    println!("[Sniper Agent] Constructing transaction to Liquidator Contract...");
    
    // Send the transaction (this requires the provider to be configured with a Wallet)
    let pending_tx = provider.send_transaction(tx).await?;
    
    let tx_hash = *pending_tx.tx_hash();
    println!("[Sniper Agent] Transaction broadcasted! Hash: {}", tx_hash);

    // Wait for receipt
    let receipt = pending_tx.get_receipt().await?;
    println!("[Sniper Agent] Transaction confirmed in block: {:?}", receipt.block_number);

    Ok(tx_hash.to_string())
}
