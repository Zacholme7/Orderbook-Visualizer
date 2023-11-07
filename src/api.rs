use crate::models::DepthSnapshot;
use reqwest::blocking::Client;

pub fn get_depth_snapshot(symbol: &str) -> Result<DepthSnapshot, Box<dyn std::error::Error>> {
    let snapshot_url = "https://api.binance.com/api/v3/depth";
    let params = [("symbol", symbol)];

    let client = Client::new();

    let res = client.get(snapshot_url)
        .query(&params)
        .send()?;

    if res.status().is_success() {
        let depth_snapshot: DepthSnapshot = res.json()?;
        Ok(depth_snapshot)
    } else {
        // If the HTTP request was not successful, handle the error
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to fetch depth snapshot: {}", res.status()),
        )))
    }
}