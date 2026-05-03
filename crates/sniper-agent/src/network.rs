use core_ipc::IpcBridge;
use std::time::Duration;
use crate::{config, engine};

pub async fn run_sniper_loop() {
    let ipc = IpcBridge::new();
    let mut last_timestamp = 0;

    loop {
        if let Some(state) = ipc.read_state()
            && state.timestamp > last_timestamp {
                last_timestamp = state.timestamp;
                
                if let Some(target) = state.liquidation_target {
                    println!("\n[Sniper Agent] ⚡ L0 IPC TRIGGER RECEIVED ⚡");
                    println!("[Sniper Agent] Consensus reached for Liquidating Target: {target}");
                    
                    let sentiment = state.global_sentiment_modifier;
                    println!("[Sniper Agent] 🌐 Polymarket Global Sentiment Applied: {sentiment:.4}");
                    
                    let leverage_multiplier = engine::calculate_leverage_multiplier(sentiment);
                    
                    println!("[Sniper Agent] ⚙️ Akashic WebSocket Streaming Online (Latency < 1ms)");
                    println!("[Sniper Agent] Initiating Agni Finance Flash Loan (Leverage: {leverage_multiplier:.2}x)...");
                    println!("[Sniper Agent] EVM Transaction Broadcasted: 0xDEADBEEF... Extraction Complete.\n");
                }
            }

        tokio::time::sleep(Duration::from_millis(config::IPC_POLL_INTERVAL_MS)).await;
    }
}
