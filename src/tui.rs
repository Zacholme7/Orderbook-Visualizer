// tui to be refactored
use ratatui::prelude::{CrosstermBackend, Terminal};

use crate::orderbook::Orderbook;

pub struct Model {
    pub should_quit: bool, 
}

// message states for communication
#[derive(PartialEq)]
pub enum Message {
    UpdateOrderbook,
    Quit,
}

impl Model {
    // TUI startup logic
    pub fn startup(&self) -> Result<Terminal<CrosstermBackend<std::io::Stderr>>, Box<dyn std::error::Error>> {
        println!("starting up the terminal");
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
        Ok(terminal)
    }

    // TUI shutdown logic
    pub fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("shutting down the terminal");
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn view(&self, orderbook: Orderbook ) {
        // should just map model to visual representation
        println!("updating the view");
    }


    pub fn handle_event(&self) -> Result<Option<Message>, Box<dyn std::error::Error>> {
        let message = if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    crossterm::event::KeyCode::Char('q') => {
                        println!("this is the quitting");
                        Message::Quit
                    }
                    _ => return Ok(None)
                }
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        };
        Ok(Some(message))
    }


}
