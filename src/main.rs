use api::get_depth_snapshot;
use orderbook::Orderbook;
use ratatui::prelude::{Direction, Layout};
use ratatui::style::Color;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use ratatui::prelude::Constraint;
use ratatui::widgets::Gauge;
use client::Client;
use ratatui::prelude::Style;

mod client;
mod websocket;
mod api;
mod orderbook;
mod models;
mod tui;

/// The main function to run the application. It will get the depth snapshot, create the orderbook,
/// then keep updating the orderbook
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let symbol = "BNBBTC";



    // make the client, this will block untill we are able to connect, if we cannot connect, handle this and exit the program
    let mut client = Client::new("Binance".to_string(), "BNBBTC".to_string());
    // if we have connected, take the snapshot and update the orderboo
    let snapshot = get_depth_snapshot(symbol)?; // get the depth snapshot
    let last_update_id = client.orderbook.lock().unwrap().update_book(snapshot)?;
    let should_continue = Arc::new(AtomicBool::new(true));

    let updater_thread = thread::spawn(move || {
        Orderbook::update_stream(client.orderbook.clone(), &symbol, last_update_id, should_continue.clone()).expect("failed in the update stream");
    });

    // create the model
    let mut model = tui::Model {
        should_quit: false,
    };

    // start up the tui and get the terminal
    let mut terminal = model.startup()?;


    while !model.should_quit {
        {
            let orderbook = client.orderbook.lock().unwrap();
            /* 
            terminal.draw(|frame| {

            })?;
            */


            let mut current_msg = model.handle_event()?;

            if current_msg != None && current_msg.unwrap() == tui::Message::Quit {
                model.should_quit = true;
                should_continue.store(false, Ordering::Relaxed);
            }

        }

        thread::sleep(Duration::from_millis(100));
    }
    model.shutdown()?;

    updater_thread.join().expect("unable to join thread");

    Ok(())
}



