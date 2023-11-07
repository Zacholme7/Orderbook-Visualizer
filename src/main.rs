use websocket::connect_to_websocket;
use api::get_depth_snapshot;
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use orderbook::Orderbook;

mod websocket;
mod api;
mod orderbook;
mod models;

fn main() {
    let symbol = "BNBBTC";

    // get the curren orderbook snapshot
    let snapshot = get_depth_snapshot(&symbol).unwrap(); // get the depth snapshot

    // create the orderbook and start the update loop
    let mut test = orderbook::Orderbook::new(); // create a new orderbook
    test.update_book(snapshot, symbol);
}



