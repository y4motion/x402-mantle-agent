use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct MarketData {
    price: f64,
    rsi: f64,
}

#[derive(Serialize)]
struct AgentDecision {
    action: String,
    reason: String,
}

#[plugin_fn]
pub fn execute_strategy(Json(data): Json<MarketData>) -> FnResult<Json<AgentDecision>> {
    let mut action = "HOLD".to_string();
    let mut reason = "Market conditions stable.".to_string();

    // Devloot logic port (Simplified RSI / Price threshold strategy)
    if data.rsi < 30.0 {
        action = "BUY".to_string();
        reason = format!("RSI {} indicates oversold conditions.", data.rsi);
    } else if data.rsi > 70.0 {
        action = "SELL".to_string();
        reason = format!("RSI {} indicates overbought conditions.", data.rsi);
    }

    Ok(Json(AgentDecision { action, reason }))
}
