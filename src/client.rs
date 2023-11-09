use crate::Orderbook;
use crate::websocket;
use tungstenite::WebSocket;
use std::sync::{Arc, Mutex};

pub struct Client {
    exchange: String,
    symbol: String,
    pub orderbook: Arc<Mutex<crate::orderbook::Orderbook>>,
}

impl Client {
    pub fn new(exchange_name: String, symbol: String) -> Self {
        Self {
            exchange: exchange_name,
            symbol: symbol,
            orderbook: Arc::new(Mutex::new(Orderbook::new())),
        }
    }

    pub fn get_websocket(&self) -> Result<WebSocket<impl std::io::Read + std::io::Write>, Box<dyn std::error::Error>> {
        println!("Connecting to websocket...");
        let socket = websocket::connect_to_websocket(&self.symbol)?;
        println!("Connected to websocket!!!");
        Ok(socket)
    }
}
