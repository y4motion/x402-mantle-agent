use core_ipc::{IpcBridge, AgentState};
use notify::{Watcher, RecursiveMode, Event, EventKind};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
struct HyperEdge {
    source: String,
    target: String,
    relation: String,
    sentiment: f64,
    timestamp: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[Memory Node] GIGANTOMANIA Vector 6 Online.");
    
    let db_path = "/home/minimalmod/curator-workspace/x402-mantle-agent/x402_memory.db";
    let db = sled::open(db_path)?;
    println!("[Memory Node] Ultra-Fast Sled HyperGraph Initialized at {}", db_path);

    let mut ipc = IpcBridge::new();
    let memory_dir = "/home/minimalmod/curator-workspace/x402-mantle-agent/swarm_memory";
    
    // Ensure directory exists
    std::fs::create_dir_all(memory_dir)?;

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new(memory_dir), RecursiveMode::Recursive)?;

    println!("[Memory Node] Obsidian-Vault Watcher listening on {}...", memory_dir);

    let mut last_timestamp = 0;

    loop {
        // 1. Process L0 IPC Experience (Liquidations)
        if let Some(state) = ipc.read_state() {
            if state.timestamp > last_timestamp && state.liquidation_target.is_some() {
                last_timestamp = state.timestamp;
                let target = state.liquidation_target.clone().unwrap();
                
                let edge = HyperEdge {
                    source: "SwarmX402".to_string(),
                    target: target.clone(),
                    relation: "LIQUIDATED".to_string(),
                    sentiment: state.global_sentiment_modifier,
                    timestamp: state.timestamp,
                };
                
                let edge_bytes = bincode::serialize(&edge)?;
                let key = format!("edge:{}:{}", edge.timestamp, target);
                db.insert(key, edge_bytes)?;
                db.flush()?;
                
                println!("[Memory Node] 🧠 Experience Crystallized in Sled HyperGraph: SwarmX402 -> LIQUIDATED -> {}", target);
            }
        }

        // 2. Process Artificial Injection (File Drops)
        if let Ok(Ok(event)) = rx.recv_timeout(Duration::from_millis(100)) {
            match event.kind {
                EventKind::Create(_) | EventKind::Modify(_) => {
                    for path in event.paths {
                        if path.extension().and_then(|s| s.to_str()) == Some("md") {
                            println!("[Memory Node] 📥 New Knowledge Tome Detected: {:?}", path.file_name().unwrap());
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                // Inject into HyperGraph
                                let edge = HyperEdge {
                                    source: "Creator".to_string(),
                                    target: "SwarmStrategy".to_string(),
                                    relation: "OVERRIDE_INJECTED".to_string(),
                                    sentiment: 1.0,
                                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                                };
                                let edge_bytes = bincode::serialize(&edge)?;
                                let key = format!("tome:{}", edge.timestamp);
                                db.insert(key, edge_bytes)?;
                                db.flush()?;
                                println!("[Memory Node] 🧬 Strategy Override Synthesized from Markdown into HyperGraph!");
                                
                                // Push to L0 IPC
                                let mut state = ipc.read_state().unwrap_or_default();
                                // Artificially boost sentiment/leverage based on Creator's input
                                state.global_sentiment_modifier += 0.5; 
                                state.timestamp = edge.timestamp;
                                ipc.write_state(&state);
                                println!("[Memory Node] ⚡ L0 IPC State Mutated via Memory Injection.");
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
