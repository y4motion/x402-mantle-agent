use std::time::Duration;
use crate::{config, engine};

pub async fn run_router_loop() {
    println!("[IPFS Router] Connecting to IPFS node at {}...", config::IPFS_GATEWAY);

    loop {
        // MOCK: Polling for new agent states to pin
        let payload = engine::prepare_ipfs_payload("mock_state_data");
        println!("[IPFS Router] Pinning payload to IPFS: {payload}");

        tokio::time::sleep(Duration::from_millis(config::ROUTING_INTERVAL_MS)).await;
    }
}
