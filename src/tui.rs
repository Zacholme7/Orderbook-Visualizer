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

        let bid_quantities: Vec<u64> = orderbook.bids.values()
            .rev()
            .cloned()
            .take(100)
            .map(|qty| qty.floor() as u64) // Round down and cast to u64
            .collect();

        let ask_quantities: Vec<u64> = orderbook.asks.values()
                .cloned()
                .take(100)
                .rev()
                .map(|qty| qty.floor() as u64 )
                .collect();

        // Now use bid_quantities for the barchart
        let bid_chart = BarChart::default()
            .block(Block::default().title("Bids").title_alignment(Alignment::Center).borders(Borders::ALL))
            .data(BarGroup::default().bars(
                &bid_quantities.iter().map(|&qty| Bar::default().value(qty).label("ehllo".into())).collect::<Vec<_>>()
            ))
            .direction(Direction::Vertical)
            .fg(Color::Green);

        let ask_chart = BarChart::default()
            .block(Block::default().title("Asks").title_alignment(Alignment::Center).borders(Borders::ALL))
            .data(BarGroup::default().bars(
                &ask_quantities.iter().map(|&qty| Bar::default().value(qty)).collect::<Vec<_>>()
            ))
            .direction(Direction::Vertical)
            .fg(Color::Red);

        frame.render_widget(bid_chart, layout[0]);
        frame.render_widget(ask_chart, layout[1]);
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
