use crate::models;
use clearscreen::clear;
use ordered_float::OrderedFloat;
use tungstenite::WebSocket;
use std::{collections::BTreeMap, error::Error};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Orderbook {
    pub asks: BTreeMap<OrderedFloat<f64>, f64>,
    pub bids: BTreeMap<OrderedFloat<f64>, f64>,
    pub last_update_id: i64,
}

impl Orderbook {
    pub fn new() -> Self {
        Self {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            last_update_id: 0,
        }
    }

    pub fn update_book(
        orderbook: Arc<Mutex<Orderbook>>, 
        mut socket: WebSocket<impl std::io::Read + std::io::Write>, 
        snapshot: models::DepthSnapshot, 
        should_continue: Arc<AtomicBool>
    ) -> Result<(), Box<dyn Error>> {
        // update the book with the initial snapshot
        {
            let mut orderbook = orderbook.lock().unwrap();
            orderbook.last_update_id = snapshot.lastUpdateId;
            orderbook.asks = snapshot.asks.into_iter().map(|entry| (OrderedFloat(entry.price), entry.qty)).collect();
            orderbook.bids = snapshot.bids.into_iter().map(|entry| (OrderedFloat(entry.price), entry.qty)).collect();
        }

        // Buffer to hold incoming WebSocket events
        let mut event_buffer = Vec::new();

        // continuoulsy read messages and update the book
        while should_continue.load(Ordering::Relaxed) {
            match socket.read_message() {
                Ok(msg) => match msg {
                    tungstenite::Message::Text(text) => {
                        if let Ok(update) = serde_json::from_str::<models::DepthUpdateEvent>(&text) {
                            let orderbook = orderbook.lock().unwrap();
                            if update.u > orderbook.last_update_id {
                                event_buffer.push(update);
                            }
                        }
                    }
                    _ => eprintln!("Received a non-text message"),
                },
                Err(_) => break, // Handle error as appropriate
            }
    
            // Process buffered events
            while let Some(event) = event_buffer.pop() {
                orderbook.lock().unwrap().process_message(event)?;
            }
        }
        Ok(())
    }

    pub fn process_message(&mut self, update: models::DepthUpdateEvent) -> Result<(), Box<dyn Error>> {
        if update.U <= self.last_update_id + 1 && update.u >= self.last_update_id + 1 {
            self.last_update_id = update.u;
            // Update bids and asks based on update.b and update.a
            for bid in update.b {
                let price = OrderedFloat(bid.price);
                if bid.qty == 0.0 {
                    self.bids.remove(&price);
                } else {
                    self.bids.insert(price, bid.qty);
                }
            }

            for ask in update.a {
                let price = OrderedFloat(ask.price);
                if ask.qty == 0.0 {
                    self.asks.remove(&price);
                } else {
                    self.asks.insert(price, ask.qty);
                }
            }
        }
        //self.print_orderbook();
        Ok(())
    }

    pub fn print_orderbook(&self) {
        clear();
        println!("\nTop 10 Asks:");
        for (price, qty) in self.asks.iter().take(10).rev() {
            println!("Price: {}, Quantity: {}", price.into_inner(), qty);
        }

        println!("\nTop 10 Bids:");
        for (price, qty) in self.bids.iter().rev().take(10) {
            println!("Price: {}, Quantity: {}", price.into_inner(), qty);
        }
    }
}
