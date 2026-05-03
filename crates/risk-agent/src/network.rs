use std::time::Duration;
use crate::{config, engine};

pub async fn run_risk_loop() {
    println!("[Risk Agent] Monitoring Swarm exposure against Mantle Mainnet...");

    let mock_exposure = 50_000.0;

    loop {
        // MOCK: Checking exposure limits
        if engine::is_exposure_safe(mock_exposure, config::MAX_EXPOSURE_USD) {
            println!("[Risk Agent] Exposure safe (${mock_exposure}). Operations nominal.");
        } else {
            println!("[Risk Agent] EXPOSURE CRITICAL. HALTING NEW TRADES.");
        }

        tokio::time::sleep(Duration::from_millis(config::RISK_POLL_INTERVAL_MS)).await;
    }
}
