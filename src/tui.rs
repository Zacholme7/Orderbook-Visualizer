use std::sync::MutexGuard;

// tui to be refactored
use ratatui::prelude::*;
use ratatui::widgets::*;

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
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
        Ok(terminal)
    }

    // TUI shutdown logic
    pub fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn view(&self, orderbook: &MutexGuard<Orderbook>, frame: &mut Frame) {

        let layout = Layout::new()
            . direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(frame.size());

        // Render the outer block for bids
        let bids_block = Block::default().title("Bids").borders(Borders::ALL);
        let bids_area = layout[0];
        let bid_area = bids_block.inner(bids_area); // Get the inner area of the bids block
        frame.render_widget(bids_block, bids_area);

        // Calculate the constraints for 10 equally spaced blocks
        let bid_constraints = std::iter::repeat(Constraint::Percentage(1))
            .take(100)
            .collect::<Vec<_>>();

        // Create a layout for the 10 inner bid blocks
        let bids_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(bid_constraints)
            .split(bid_area);

        // Render each inner bid block
        for i in 0..100 {
            let bids_inner = Block::default().title(format!("Bid {}", i + 1)).borders(Borders::ALL);
            frame.render_widget(bids_inner, bids_layout[i]);
        }


        let asks_block = Block::default().title("Asks").borders(Borders::ALL);
        let asks_area = layout[1];
        let ask_area = asks_block.inner(asks_area); // Get the inner area of the bids block
        frame.render_widget(asks_block, asks_area);

        // Calculate the constraints for 10 equally spaced blocks
        let ask_constraints = std::iter::repeat(Constraint::Percentage(1))
            .take(100)
            .collect::<Vec<_>>();

        // Create a layout for the 10 inner bid blocks
        let asks_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(ask_constraints)
            .split(ask_area);

        // Render each inner bid block
        for i in 0..100 {
            let asks_inner = Block::default().title(format!("Asks {}", i + 1));
            frame.render_widget(asks_inner, asks_layout[i]);
        }
    }
    
    // Helper function to render a side of the order book


    pub fn handle_event(&self) -> Result<Option<Message>, Box<dyn std::error::Error>> {
        let message = if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    crossterm::event::KeyCode::Char('q') => {
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
