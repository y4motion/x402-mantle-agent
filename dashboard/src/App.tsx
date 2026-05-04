import React, { useState } from 'react';
import { Activity, Shield, Zap, Terminal, Globe, Network } from 'lucide-react';
import './index.css';

// Mocked wagmi/viem imports for the UI demo.
// In a real build, we wrap App with <WagmiConfig> and use `useContractRead`.

type ViewMode = 'IPC' | 'ERC8004';

export default function App() {
  const [viewMode, setViewMode] = useState<ViewMode>('ERC8004');

  // Simulated On-Chain Events from X402FlashLiquidator
  const events = [
    { id: '1', agentId: 'ERC8004: 1', target: '0x3a9f...8b21', sentiment: 1.05, success: true, time: '2 mins ago' },
    { id: '2', agentId: 'ERC8004: 1', target: '0x71bc...11a0', sentiment: 1.12, success: true, time: '15 mins ago' },
    { id: '3', agentId: 'ERC8004: 2', target: '0x99aa...ff10', sentiment: 0.95, success: true, time: '1 hour ago' },
  ];

  return (
    <div className="layout">
      <header className="header">
        <div className="title-group">
          <h1>X402 Swarm Intelligence</h1>
          <p>Mantle Network | AI Agent Execution Layer</p>
        </div>
        
        <div className="glass-panel" style={{ padding: '8px', display: 'flex', gap: '8px' }}>
          <button 
            className={`btn ${viewMode === 'IPC' ? 'active' : ''}`}
            onClick={() => setViewMode('IPC')}
          >
            <Terminal size={16} />
            L0 IPC Metrics
          </button>
          <button 
            className={`btn ${viewMode === 'ERC8004' ? 'active' : ''}`}
            onClick={() => setViewMode('ERC8004')}
          >
            <Shield size={16} />
            ERC-8004 Identity
          </button>
        </div>
      </header>

      <div className="metrics-grid">
        <div className="glass-panel metric-card">
          <h3><Activity size={14} style={{ display: 'inline', marginRight: '6px' }}/> Total Liquidations</h3>
          <div className="value">1,402</div>
        </div>
        <div className="glass-panel metric-card">
          <h3><Zap size={14} style={{ display: 'inline', marginRight: '6px' }}/> Swarm Reputation</h3>
          <div className="value">8,950</div>
        </div>
        <div className="glass-panel metric-card">
          <h3><Globe size={14} style={{ display: 'inline', marginRight: '6px' }}/> Mantle Network</h3>
          <div className="value" style={{ color: '#00d2ff' }}>Connected</div>
        </div>
        <div className="glass-panel metric-card">
          <h3><Network size={14} style={{ display: 'inline', marginRight: '6px' }}/> Active Agents</h3>
          <div className="value">2 (ERC-8004)</div>
        </div>
      </div>

      <div className="glass-panel">
        <h2 style={{ marginBottom: '24px', fontSize: '1.2rem', fontWeight: 600 }}>
          {viewMode === 'ERC8004' ? 'Recent On-Chain AI Inferences (X402FlashLiquidator)' : 'Internal Memory-Mapped IPC Logs'}
        </h2>
        
        <div className="events-list">
          {events.map((ev) => (
            <div className="event-row" key={ev.id}>
              <div className="event-col">
                <span className="event-label">Agent ID</span>
                <span className="event-value" style={{ color: '#00d2ff' }}>{ev.agentId}</span>
              </div>
              <div className="event-col">
                <span className="event-label">Target Address</span>
                <span className="event-value">{ev.target}</span>
              </div>
              <div className="event-col">
                <span className="event-label">AI Sentiment Multiplier</span>
                <span className="event-value">{ev.sentiment}x</span>
              </div>
              <div className="event-col">
                <span className="event-label">Status</span>
                <span className={`badge ${ev.success ? 'success' : ''}`}>
                  {ev.success ? 'EXECUTED' : 'FAILED'}
                </span>
              </div>
              <div className="event-col">
                <span className="event-label">Time</span>
                <span className="event-value" style={{ color: 'var(--text-secondary)'}}>{ev.time}</span>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
