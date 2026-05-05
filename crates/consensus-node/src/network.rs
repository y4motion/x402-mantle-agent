use std::time::Duration;
use core_ipc::IpcBridge;
use crate::{config, engine};

pub async fn run_consensus_loop() {
    println!("[Consensus Node] Listening for agent votes via L0 IPC...");

    let ipc = IpcBridge::new();
    let mut last_timestamp = 0;

    loop {
        if let Some(state) = ipc.read_state()
            && state.timestamp > last_timestamp
        {
            last_timestamp = state.timestamp;
            
            let mut active_votes = 0;
            if state.sniper_vote.unwrap_or(false) { active_votes += 1; }
            if state.risk_vote.unwrap_or(false) { active_votes += 1; }
            
            if engine::check_consensus_reached(active_votes, config::VOTE_THRESHOLD) {
                println!("[Consensus Node] Consensus Reached! Proceeding to execution phase...");
            }
        }

        tokio::time::sleep(Duration::from_millis(config::CONSENSUS_TIMEOUT_MS)).await;
    }
}
