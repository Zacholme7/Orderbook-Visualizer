use crate::{models, websocket};
use clearscreen::clear;
use ordered_float::OrderedFloat;
use std::{collections::BTreeMap, error::Error};

pub struct Orderbook {
    pub asks: BTreeMap<OrderedFloat<f64>, f64>,
    pub bids: BTreeMap<OrderedFloat<f64>, f64>,
}

impl Orderbook {
    pub fn new() -> Self {
        Self {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
        }
    }

    pub fn update_book(&mut self, snapshot: models::DepthSnapshot, symbol: &str) -> Result<(), Box<dyn Error>> {
        let last_update_id = snapshot.lastUpdateId;
        self.asks = snapshot.asks.into_iter().map(|entry| (OrderedFloat(entry.price), entry.qty)).collect();
        self.bids = snapshot.bids.into_iter().map(|entry| (OrderedFloat(entry.price), entry.qty)).collect();

        let mut socket = websocket::connect_to_websocket(symbol)?;
        while let Ok(msg) = socket.read_message() {
            match msg {
                tungstenite::Message::Text(text) => {
                    if let Ok(update) = serde_json::from_str::<models::DepthUpdateEvent>(&text) {
                        self.process_message(update, last_update_id)?;
                    }
                }
                _ => eprintln!("Received a non-text message"),
            }
        }
        Ok(())
    }

    pub fn process_message(&mut self, update: models::DepthUpdateEvent, last_update_id: i64) -> Result<(), Box<dyn Error>> {
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
        self.print_orderbook()?;
        Ok(())
    }

    pub fn print_orderbook(&self) -> Result<(), Box<dyn Error>> {
        clear()?;
        println!("\nTop 10 Asks:");
        self.asks.iter().take(10).rev().for_each(|(price, qty)| println!("Price: {}, Quantity: {}", price.into_inner(), qty));
        println!("\nTop 10 Bids:");
        self.bids.iter().rev().take(10).for_each(|(price, qty)| println!("Price: {}, Quantity: {}", price.into_inner(), qty));
        Ok(())
    }
}