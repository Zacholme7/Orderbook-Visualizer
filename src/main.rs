use api::get_depth_snapshot;

mod websocket;
mod api;
mod orderbook;
mod models;

/// The main function to run the application. It will get the depth snapshot, create the orderbook,
/// then keep updating the orderbook
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let symbol = "BNBBTC";

    //get the curren orderbook snapshot
    let snapshot = get_depth_snapshot(symbol)?; // get the depth snapshot

    // create the orderbook and start the update loop
    let mut test = orderbook::Orderbook::new(); // create a new orderbook
    test.update_book(snapshot, symbol)?;
    Ok(())
}



