use alloy::providers::Provider;
use alloy::transports::Transport;
use alloy::network::Network;
use alloy::sol;
use std::str::FromStr;
use alloy::primitives::Address;

pub struct LiquidationTarget {
    pub address: String,
    pub health_factor: f64,
}

// ABI for a generic Lending Pool to check health factor
sol! {
    #[sol(rpc)]
    contract ILendingPool {
        function getUserAccountData(address user) external view returns (
            uint256 totalCollateralETH,
            uint256 totalDebtETH,
            uint256 availableBorrowsETH,
            uint256 currentLiquidationThreshold,
            uint256 ltv,
            uint256 healthFactor
        );
    }
}

pub async fn scan_for_targets<P, T, N>(provider: &P) -> Option<LiquidationTarget> 
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
{
    // We are now connected to the actual RPC.
    // For MVP, we query a specific target on Mantle Testnet
    let target_address = Address::from_str("0x0000000000000000000000000000000000000001").unwrap();
    let pool_address = Address::from_str("0x0000000000000000000000000000000000000002").unwrap();
    
    let pool = ILendingPool::new(pool_address, provider);
    
    // Perform the actual on-chain RPC call
    let result = pool.getUserAccountData(target_address).call().await;
    
    match result {
        Ok(data) => {
            // healthFactor is typically returned with 18 decimals, we convert it to f64
            // Here we assume it returns a value where 1e18 = 1.0
            let hf_f64 = data.healthFactor.to_string().parse::<f64>().unwrap_or(1.0) / 1e18;
            println!("[Liquidator Daemon] On-Chain scan completed. HF: {}", hf_f64);
            
            if hf_f64 < 1.0 {
                Some(LiquidationTarget {
                    address: target_address.to_string(),
                    health_factor: hf_f64,
                })
            } else {
                None
            }
        },
        Err(e) => {
            println!("[Liquidator Daemon] RPC Error scanning target: {:?}", e);
            None
        }
    }
}
