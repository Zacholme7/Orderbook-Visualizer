use tungstenite::stream::Stream;
use crate::Orderbook;
use crate::websocket;
use std::sync::{Arc, Mutex};



pub struct Client{
    exchange: String,
    symbol: String,
    pub orderbook: Arc<Mutex<crate::orderbook::Orderbook>>
}

impl Client {
    pub fn new(exchange_name: String, symbol: String) -> Self {
        let socket = websocket::connect_to_websocket(&symbol);

        Self {
            exchange: exchange_name,
            symbol: symbol,
            orderbook: Arc::new(Mutex::new(Orderbook::new()))
        }
    }
}