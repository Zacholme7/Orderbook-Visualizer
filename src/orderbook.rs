use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use crate::{websocket, models};

// Structure Representing the orderboo
pub struct Orderbook {
    pub asks: BTreeMap<OrderedFloat<f64>, f64>,
    pub bids: BTreeMap<OrderedFloat<f64>, f64>,
}


impl Orderbook {
    // constructor for the orderbook
    pub fn new() -> Orderbook {
        Orderbook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
        }
    }

    // recieve a mesasge from the websocket and update the book
    pub fn update_book(&mut self, snapshot: models::DepthSnapshot, symbol: &str) {
        // get the last update id, for managing local order boo
        let last_update_id = snapshot.lastUpdateId;

        // insert the current asks and bids from the snapshot
        for ask in snapshot.asks {
            self.asks.insert(OrderedFloat(ask.price), ask.qty);
        }
        for bid in snapshot.bids {
            self.bids.insert(OrderedFloat(bid.price), bid.qty);
        }

        let mut socket = match websocket::connect_to_websocket(symbol) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to connect to the WebSocket: {}", e);
                return;
            }
        };
    
        loop {
            match socket.read_message() {
                Ok(msg) => match msg {
                    tungstenite::Message::Text(text) => {
                        match serde_json::from_str::<models::DepthUpdateEvent>(&text) {
                            Ok(update) => {
                                // If successful, process the update
                                self.process_message(update)
                            },
                            Err(e) => {
                                eprintln!("Failed to parse depth update: {}", e);
                            }
                        }
                    }
                    _ => eprintln!("Received a non-text message"),
                },
                Err(e) => {
                    eprintln!("Error reading message: {}", e);
                    break;
                }
            }
        }
    }

    pub fn process_message(&mut self, msg: models::DepthUpdateEvent ) {
        println!("this is just a test")
    }
}