use core_ipc::IpcBridge;
use serde_json::Value;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[Polymarket Oracle] GIGANTOMANIA Vector 4 Online.");
    println!("[Polymarket Oracle] Establishing connection to Gamma API (gamma-api.polymarket.com)...");

    let mut ipc = IpcBridge::new();
    let client = reqwest::Client::new();

    loop {
        // Fetch top active markets by volume
        let url = "https://gamma-api.polymarket.com/markets?limit=5&order=volume24hr&ascending=false&active=true";
        
        match client.get(url).send().await {
            Ok(response) => {
                if let Ok(markets) = response.json::<Vec<Value>>().await {
                    let mut aggregated_prob = 0.0;
                    let mut count = 0.0;

                    for market in markets {
                        if let Some(prices) = market.get("outcomePrices") {
                            if let Some(prices_array) = prices.as_array() {
                                if let Some(yes_price_str) = prices_array.get(0).and_then(|v| v.as_str()) {
                                    if let Ok(yes_price) = yes_price_str.parse::<f64>() {
                                        aggregated_prob += yes_price;
                                        count += 1.0;
                                    }
                                }
                            }
                        }
                    }

                    if count > 0.0 {
                        let avg_sentiment = aggregated_prob / count;
                        println!("[Polymarket Oracle] Global Macro Sentiment Extracted: {:.4}", avg_sentiment);

                        // Read current state to avoid overwriting other agents' data
                        let mut state = ipc.read_state().unwrap_or_default();
                        
                        // Inject Global Sentiment into L0 IPC
                        state.global_sentiment_modifier = avg_sentiment;
                        state.timestamp = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs();

                        ipc.write_state(&state);
                        println!("[Polymarket Oracle] L0 IPC Memmap Updated: Sniper Agent leverage modified.");
                    }
                }
            }
            Err(e) => {
                println!("[Polymarket Oracle] API Error: {}", e);
            }
        }

        // Poll every 10 seconds (Gamma API rate limit friendly)
        thread::sleep(Duration::from_secs(10));
    }
}
