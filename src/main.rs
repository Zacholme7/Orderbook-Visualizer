use websocket::connect_to_websocket;
use api::get_depth_snapshot;
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use orderbook::Orderbook;

mod websocket;
mod api;
mod orderbook;
mod models;

// this is a random test blah blah

fn main() {
    let symbol = "BNBBTC";
    //connect_to_websocket(&symbol); // connect to the websocket

    // get the curren orderbook snapshot
    let snapshot = get_depth_snapshot(&symbol).unwrap(); // get the depth snapshot

    // create the orderbook and start the update loop
    // run this in a new thread??
    let mut test = orderbook::Orderbook::new(); // create a new orderbook
    test.update_book(snapshot, symbol);

}



