use std::time::Duration;
use core_ipc::IpcBridge;
use crate::{config, engine};

pub async fn run_router_loop() {
    println!("[IPFS Router] Connecting to IPFS node at {}...", config::IPFS_GATEWAY);

    let ipc = IpcBridge::new();
    let mut last_timestamp = 0;

    loop {
        // Read live swarm state from L0 IPC bridge
        let payload_data = if let Some(state) = ipc.read_state()
            && state.timestamp > last_timestamp
        {
            last_timestamp = state.timestamp;
            let target = state.liquidation_target.as_deref().unwrap_or("idle");
            format!("ts={},target={},sentiment={:.4}",
                    state.timestamp, target, state.global_sentiment_modifier)
        } else {
            "idle".to_string()
        };

        let payload = engine::prepare_ipfs_payload(&payload_data);
        println!("[IPFS Router] Pinning state snapshot to IPFS: {payload}");

        tokio::time::sleep(Duration::from_millis(config::ROUTING_INTERVAL_MS)).await;
    }
}
