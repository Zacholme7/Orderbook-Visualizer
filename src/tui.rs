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

    pub fn view(&self, orderbook: &MutexGuard<Orderbook>, frame: &mut Frame ) {
        // should just map model to visual representation

        // make the rect centered
        let percent_y = 99;
        let percent_x = 99;
        let popup_layout = Layout::default()
          .direction(Direction::Vertical)
          .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
          ])
          .split(frame.size());
      
        let centered_rect = Layout::default()
          .direction(Direction::Horizontal)
          .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
          ])
          .split(popup_layout[1])[1];

        // split into the two boxes for bids and asks
        let bid_ask_layout = Layout::default()
          .direction(Direction::Horizontal)
          .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
          ])
          .split(centered_rect);

         let constraints = (0..10) // Create an iterator for 10 items
            .map(|i| Constraint::Percentage(10 * (10 - i))) // Map each item to a decreasing percentage
            .collect::<Vec<_>>(); // Collect into a Vec<Constraint>

        let bids = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints) // Use the generated constraints
            .split(bid_ask_layout[0]);

            /* 
        frame.render_widget(
            Block::default().borders(Borders::all()).title("Orderbook").title_alignment(Alignment::Center), centered_rect
        );

        frame.render_widget(
            Block::default().borders(Borders::all()).title("Bids").title_alignment(Alignment::Center), bid_ask_layout[0]
        );
        */
        frame.render_widget(
            Block::default().borders(Borders::all()).title("Asks").title_alignment(Alignment::Center), bid_ask_layout[1]
        );

        for i in 0..10 {
            frame.render_widget(
                Block::default().borders(Borders::all()), bids[i]
            );
        }



    }



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
