use std::cell::Cell;
use std::io;
use std::rc::Rc;
use crossterm::event::{KeyCode, KeyEvent};
use log::{warn};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Widget;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

const ACTION: &str = "n: Create File | o: Open File\n\nEsc: Close";

#[derive(Debug, Default)]
pub struct ActionBar {
    pub show: Rc<Cell<bool>>,
}

impl ActionBar {
    pub fn new(show: Rc<Cell<bool>>) -> ActionBar {
        ActionBar { show }
    }
}

impl ActionBar {
    pub fn handle_input(&mut self, key: KeyEvent) -> Result<(), io::Error> {
        match key {
            KeyEvent { code: KeyCode::Esc, modifiers: _, .. } => {
                self.show.set(false);
            }
            KeyEvent { code: KeyCode::Char('n'), .. } => {
                warn!("create file");
            },
            KeyEvent { code: KeyCode::Char('o'), .. } => {
                warn!("open file");
            },
            _ => ()
        }
        Ok(())
    }
}

impl Widget for &ActionBar {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Max(6),
            ])
            .split(area);

        let block = Block::default().borders(Borders::ALL);

        let action_message = Paragraph::new(ACTION)
            .block(block)
            .centered()
            .bold();

        Clear.render(layout[1], buf);
        action_message.render(layout[1], buf);
    }
}
