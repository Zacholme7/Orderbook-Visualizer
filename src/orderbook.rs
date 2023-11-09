use crate::{models, websocket};
use clearscreen::clear;
use ordered_float::OrderedFloat;
use tungstenite::WebSocket;
use std::{collections::BTreeMap, error::Error};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
/// Structure representing a market orderboo
pub struct Orderbook {
    /// This is all of the ask orders
    pub asks: BTreeMap<OrderedFloat<f64>, f64>,
    /// This is all of the bid orders
    pub bids: BTreeMap<OrderedFloat<f64>, f64>,
}

impl Orderbook {
    /// Constructor for a new orderbook, just default initialization
    pub fn new() -> Self {
        Self {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
        }
    }
    // Function to get the maximum bid quantity
    pub fn get_max_bid_quantity(&self) -> Option<f64> {
        self.bids.values().cloned().max_by(|x, y| x.partial_cmp(y).unwrap())
    }

    // Function to get the maximum ask quantity
    pub fn get_max_ask_quantity(&self) -> Option<f64> {
        self.asks.values().cloned().max_by(|x, y| x.partial_cmp(y).unwrap())
    }


    /// Main function to update the orderbook
    /// This will get a connection to the websocket, update the book with the initla snapshot
    /// then continuously update the book while printing it out
    pub fn update_book(&mut self, snapshot: models::DepthSnapshot) -> Result<i64, Box<dyn Error>> {
        let last_update_id = snapshot.lastUpdateId;
        self.asks = snapshot.asks.into_iter().map(|entry| (OrderedFloat(entry.price), entry.qty)).collect();
        self.bids = snapshot.bids.into_iter().map(|entry| (OrderedFloat(entry.price), entry.qty)).collect();
        Ok(last_update_id)
    }

    pub fn update_stream(
        orderbook: Arc<Mutex<Orderbook>>, 
        last_update_id: i64, 
        should_continue: Arc<AtomicBool>,
        mut socket: WebSocket<impl std::io::Read + std::io::Write>,
        ) -> Result<(), Box<dyn Error>> {
        while should_continue.load(Ordering::Relaxed) {
            // The let binding is incorrect here; you should match on the result of the expression directly.
            match socket.read_message() {
                Ok(msg) => { // Corrected let binding
                    match msg {
                        tungstenite::Message::Text(text) => {
                            if let Ok(update) = serde_json::from_str::<models::DepthUpdateEvent>(&text) {
                                let mut orderbook = orderbook.lock().unwrap();
                                orderbook.process_message(update, last_update_id)?;
                            }
                        }
                        _ => eprintln!("Received a non-text message"),
                    }
                }
                Err(_) => {
                    // Handle the error, for example, by breaking out of the loop
                    break;
                }
            }
        }
        Ok(())
    }


    /// This function will process each message recieved from the websocket
    /// It will update the orderbook with the new bids and asks and remove the orders that have been filled 
    fn process_message(&mut self, update: models::DepthUpdateEvent, last_update_id: i64) -> Result<(), Box<dyn Error>> {
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

        // want to update the tui from here now
        //self.print_orderbook()?;
        Ok(())
    }

    /// Print out the orderbook for visualization
    fn print_orderbook(&self) -> Result<(), Box<dyn Error>> {
        clear()?;
        println!("\nTop 10 Asks:");
        self.asks.iter().take(10).rev().for_each(|(price, qty)| println!("Price: {}, Quantity: {}", price.into_inner(), qty));
        println!("\nTop 10 Bids:");
        self.bids.iter().rev().take(10).for_each(|(price, qty)| println!("Price: {}, Quantity: {}", price.into_inner(), qty));
        Ok(())
    }
}