use crate::models::DepthSnapshot;
use reqwest::blocking::Client;
use std::error::Error;

/// This function will get an initial snapshot of the orderbook. This is to provide us
/// with the starting orders to update from the websocket
pub fn get_depth_snapshot(symbol: &str) -> Result<DepthSnapshot, Box<dyn Error>> {
    let snapshot_url = "https://api.binance.com/api/v3/depth";
    let params = [("symbol", symbol)];

    let client = Client::new();
    let res = client.get(snapshot_url)
        .query(&params)
        .send()?;

    if res.status().is_success() {
        let depth_snapshot = res.json()?;
        Ok(depth_snapshot)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to fetch depth snapshot: {}", res.status()),
        )))
    }
}
