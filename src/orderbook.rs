use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use crate::{websocket, models};
use clearscreen::clear;

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
            // read a message and parse it
            // if it is a successful message, pass it to be processed
            // otherwise, throw an error
            match socket.read_message() {
                Ok(msg) => match msg {
                    tungstenite::Message::Text(text) => {
                        match serde_json::from_str::<models::DepthUpdateEvent>(&text) {
                            Ok(update) => {
                                // If successful, process the update
                                //println!("{:?}", update);
                                self.process_message(update, last_update_id)
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

    pub fn process_message(&mut self, update: models::DepthUpdateEvent, last_update_id: i64) {
        // First, check if the update ID is relevant.
        if update.u > last_update_id {
            // Update bids
            for bid in update.b {
                let price = OrderedFloat(bid.price);
                if bid.qty == 0.0 {
                    // If the quantity is zero, remove the price level.
                    self.bids.remove(&price);
                } else {
                    // Otherwise, insert or update the price level with the new quantity.
                    self.bids.insert(price, bid.qty);
                }
            }

            // Update asks
            for ask in update.a {
                let price = OrderedFloat(ask.price);
                if ask.qty == 0.0 {
                    // If the quantity is zero, remove the price level.
                    self.asks.remove(&price);
                } else {
                    // Otherwise, insert or update the price level with the new quantity.
                    self.asks.insert(price, ask.qty);
                }
            }
        }
        self.print_orderbook();
    }

    pub fn print_orderbook(&self) {
        if let Err(e) = clear() {
            eprintln!("Failed to clear screen: {}", e);
            return;
        }
        let top_asks: Vec<_> = self.asks.iter().take(10).collect();
        println!("\nTop 10 Asks:");
        for (price, qty) in top_asks.iter().rev() {
            println!("Price: {}, Quantity: {}", price.into_inner(), qty);
        }

        // Print top 10 bids (highest price first)
        println!("\nTop 10 Bids:");
        for (price, qty) in self.bids.iter().rev().take(10) {
            println!("Price: {}, Quantity: {}", price.into_inner(), qty);
        }
    }

}