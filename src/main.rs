use websocket::connect_to_websocket;
use api::get_depth_snapshot;
use std::collections::BTreeMap;

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

    // get the last updated id
    let last_update_id = snapshot.lastUpdateId;

    // init our local orderbook or asks and bids
    let mut asks: BTreeMap<f64, f64> = BTreeMap::new();
    let mut bids: BTreeMap<f64, f64> = BTreeMap::new();

    for ask in snapshot.asks {
        asks.insert(ask.price, ask.qty);
    }

    for bid in snapshot.bids {
        bids.insert(bid.price, bid.qty);
    }
}

