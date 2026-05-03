# X402 MANTLE PANOPTICON SWARM
`VERSION: 0.2.0-beta` | `ENVIRONMENT: Rust Native / L0 IPC` | `TARGET: Mantle Turing Test Phase 2`

## ARCHITECTURE OVERVIEW

The X402 Panopticon is a hyper-fast, 7-node sovereign Rust swarm designed for the Mantle ecosystem. It abandons traditional HTTP polling and REST APIs in favor of an L0 Memory-Mapped Inter-Process Communication (IPC) bridge, WebSocket streaming, and WASM sandboxing.

Unlike traditional trading bots, X402 operates as an autonomous multi-agent intelligence, integrating real-world geopolitical sentiment via Polymarket and maintaining its own localized, bi-temporal hypergraph memory.

## CORE DIRECTORY STRUCTURE

```text
/x402-mantle-agent
├── crates/                    # The Sovereign Agent Nodes (Rust)
│   ├── core-ipc/              # L0 IPC bridge using memmap2 and bincode
│   ├── auditor-node/          # Transaction auditing and risk verification
│   ├── ipfs-router/           # Decentralized storage and logging
│   ├── liquidator-daemon/     # High-frequency target scanning
│   ├── memory-node/           # Akashic Sled HyperGraph for experience retention
│   ├── polymarket-oracle/     # Real-world geopolitical sentiment ingestion
│   └── sniper-agent/          # WebSocket execution and flash loan leverage scaling
├── wasm-core/                 # Extism-compatible WASM plugin engine
├── contracts/                 # Hardhat/Foundry solidity contracts for Mantle testing
├── swarm_memory/              # The Obsidian-style Vault for Markdown strategy injection
├── legacy-python/             # Archived Python scripts (Gigantomania Phase 1)
└── x402_memory.db/            # The Sled key-value localized HyperGraph data store
```

## THE 7 VECTORS OF GIGANTOMANIA

1. **L0 IPC Engine**: Zero-copy state synchronization across all 7 agents using `/tmp/x402_ipc.mmap`.
2. **WebSocket Sniper**: Continuous, sub-millisecond execution via Mantle native RPC WebSocket feeds.
3. **WASM Sandboxing**: Untrusted strategy scripts run safely in an isolated WebAssembly environment.
4. **Polymarket Omniscience**: `global_sentiment_modifier` continuously feeds real-world probabilities into leverage equations.
5. **Akashic Sled Memory**: Ultra-fast embedded graph database (`sled`) that records liquidations as hyper-edges.
6. **Strategy Override**: The `swarm_memory` folder allows human operators to inject Markdown-based strategy overrides directly into the swarm's consciousness.
7. **Absolute OPSEC**: A quarantined `.gitignore` and isolated knowledge base prevents identity or logic leakage.

## DEPLOYMENT

```bash
# 1. Compile the Swarm
cargo build --release --workspace

# 2. Boot the L0 Bridge and Agents
./target/release/polymarket-oracle &
./target/release/liquidator-daemon &
./target/release/memory-node &
./target/release/sniper-agent &
```

## ORIGIN
Forged in the Triarchy Crucible.
Designed for the DoraHacks Mantle Turing Test (2026).
