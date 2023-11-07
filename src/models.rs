use serde::{Serialize, Deserialize, de::{self, Deserializer}};
/// Represents all of the structures that we use for serialization and deserialization from the
/// websocket and the api

#[derive(Serialize, Deserialize, Debug)]
pub struct DepthUpdateEvent {
    pub e: String, // Event type
    pub E: i64,    // Event time
    pub s: String, // Symbol
    pub U: i64,    // First update ID in event
    pub u: i64,    // Final update ID in event
    pub b: Vec<OrderBookEntry>, // Bids to be updated
    pub a: Vec<OrderBookEntry>, // Asks to be updated
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepthSnapshot {
    pub lastUpdateId: i64,
    pub bids: Vec<OrderBookEntry>,
    pub asks: Vec<OrderBookEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookEntry {
    #[serde(deserialize_with = "deserialize_order")]
    pub price: f64, // PRICE

    #[serde(deserialize_with = "deserialize_order")]
    pub qty: f64, // QTY
}

// Custom deserialization function for price
fn deserialize_order<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(de::Error::custom)
}