pub struct LiquidationTarget {
    pub address: String,
    pub health_factor: f64,
}

pub fn scan_for_targets() -> Option<LiquidationTarget> {
    // MOCK: Scanning logic using alloy to read from RPC
    let mock_health_factor = 0.95; // Underwater!
    let target_address = "0xHumanTrader00000000000000000000000000001";

    if mock_health_factor < 1.0 {
        Some(LiquidationTarget {
            address: target_address.to_string(),
            health_factor: mock_health_factor,
        })
    } else {
        None
    }
}
