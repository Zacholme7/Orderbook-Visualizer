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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbol = "ADAUSDT";

    // Create a new orderbook instance
    let orderbook = Arc::new(Mutex::new(orderbook::Orderbook::new()));

    // Connect to WebSocket and start listening to events
    let socket = websocket::connect_to_websocket(symbol)?;
    
      // Fetch the depth snapshot and populate book wtih it initially
    let snapshot = get_depth_snapshot(symbol)?;

    // boolean to disconnect the thread
    let should_continue = Arc::new(AtomicBool::new(true));


    // clones to be passed into the updater thread
    let orderbook_clone = orderbook.clone();
    let should_continue_clone = should_continue.clone();

    let updater_thread = thread::spawn(move || {
        Orderbook::update_book(
            orderbook_clone,
            socket, 
            snapshot,
            should_continue_clone,
        ).expect("failed in the update stream");
    });

    // create the model for the tui
    let mut model = tui::Model {
        should_quit: false,
    };

    // start up the tui and get the terminal
    let mut terminal = model.startup()?;

    while !model.should_quit {
        {
            let orderbook = orderbook.lock().unwrap();
            terminal.draw(|f| {
                model.view(&orderbook, f);
            })?;
            let current_msg = model.handle_event()?;

            if current_msg != None && current_msg.unwrap() == tui::Message::Quit {
                model.should_quit = true;
                should_continue.store(false, Ordering::Relaxed);
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
    model.shutdown()?;
    updater_thread.join();

    Ok(())
}



