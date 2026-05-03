# X402 PROJECT MAP

This document maps the data flow, architecture layers, and OPSEC boundaries of the X402 Mantle Agent Swarm.

## TOPOLOGY DIAGRAM

```text
                                [ EXTERNAL APIS ]
                                        |
      +---------------------+           | (HTTP/JSON)
      | POLYMARKET ORACLE   |<----------+
      | (crates/oracle)     |           
      +---------------------+           
                 | (Sentiment Injection)
                 V
=========================================================
            [ L0 IPC LAYER ] (/tmp/x402_ipc.mmap)
      [ AgentState: sentiment, target, timestamp ]
=========================================================
      ^          ^          ^           ^           |
      |          |          |           |           |
+----------+ +--------+ +-------+ +-----------+     |
|LIQUIDATOR| | SNIPER | |AUDITOR| |IPFS-ROUTER|     | (Bi-Temporal Logging)
| DAEMON   | | AGENT  | | NODE  | |   NODE    |     |
+----------+ +--------+ +-------+ +-----------+     |
      |          |                                  V
  (WSS)      (WSS)                          +-------------------+
      |          |                          |   MEMORY NODE     |<-- (notify) [ swarm_memory/ ]
      V          V                          | (crates/memory)   |
  [ MANTLE MAINNET / ANVIL ]                +-------------------+
  [ Agni / Merchant Moe    ]                          |
                                            [ x402_memory.db ]
                                            (Sled HyperGraph)
```

## LAYER DEFINITIONS

### 1. The Real-World Ingestion Layer
- **Polymarket Oracle**: Scans Gamma API. Outputs a float `global_sentiment_modifier` reflecting macroeconomic conditions. Prevents the swarm from acting blindly in volatile real-world markets.

### 2. The Execution Layer
- **Liquidator Daemon**: The hunter. Scans on-chain states for undercollateralized targets. Drops target addresses into the L0 IPC.
- **Sniper Agent**: The executioner. Pulls the target from L0 IPC. Scales flash loan leverage based on the Oracle's sentiment modifier. Executes via low-latency WebSocket.
- **Auditor Node**: Post-execution validation. Analyzes gas spent vs revenue.
- **IPFS Router**: Pushes execution logs to IPFS for immutable decentralized storage.

### 3. The Akashic Memory Layer (L2 Graph)
- **Memory Node**: The Hippocampus. Reads successful execution states from L0 IPC and serializes them into a pure-Rust `sled` embedded KV store as hyper-edges.
- **Swarm Memory Vault**: A local folder (`swarm_memory/`) acting as the brain interface. The creator can drop Markdown files here to manually override swarm sentiment or logic.

### 4. The WASM Layer
- **WASM Core**: `wasm-core/` allows sandboxing of experimental or untrusted prediction algorithms using Extism. 

## DEBT AND TO-DO LIST
- [ ] Connect `auditor-node` to the WASM core for dynamic policy updates.
- [ ] Fully wire `IPFS-router` to upload `swarm_memory` digests.
- [ ] Expand `sled` graph schemas to track target wallet history.
