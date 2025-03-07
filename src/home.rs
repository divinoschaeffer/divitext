use std::cell::RefCell;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::prelude::*;
use std::io;
use std::rc::Rc;
use crate::state::State;

const NAME_TITLE: &str = r##"
      ██    ██  ███████  ██       ██       ███████.
      ██    ██  ██       ██       ██      ██     ██
      ████████  ██████   ██       ██      ██     ██
      ██    ██  ██       ██       ██      ██     ██
      ██    ██  ███████  ███████  ███████  ███████.

██     ███    ██   ███████   ███████    ██       ████████    ██
██     ███    ██  ██     ██  ██    ██   ██       ██     ██   ██
██     ███    ██  ██     ██  ███████    ██       ██      ██  ██
 ██   ██ ██  ██   ██     ██  ██    ██   ██       ██     ██.....
   ████   ████     ███████   ██    ██   ███████  ████████    ██
"##;

const NEW_FILE: &str = r##"+ New File       CRL n"##;

#[derive(Debug)]
pub struct Home<'a> {
    pub state: Rc<RefCell<State<'a>>>
}

impl<'a> Home<'a> {

    pub fn new(state: Rc<RefCell<State<'a>>>) -> Home<'a> {
        Self {
            state
        }
    }
    pub fn handle_input(&mut self, key: KeyEvent) -> Result<(), io::Error> {
        match key {
            KeyEvent { code, modifiers, .. } => {
                if code == KeyCode::Char('n') && modifiers == KeyModifiers::CONTROL {
                    println!("Ctrl+N pressed!");
                }
            }
        }
        Ok(())
    }
}

impl Widget for &Home<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ])
            .split(area);

        let center_area_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(500),
                Constraint::Fill(1),
            ])
            .split(layout[1]);

        let inner_center_area= Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(300),
                Constraint::Min(400),
            ])
            .split(center_area_layout[1]);

        let title_area = inner_center_area[0];
        let main_area = inner_center_area[1];

        let actions_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
            ])
            .split(main_area);

        let title: Text = Text::raw(NAME_TITLE.replace('.', " "))
            .centered()
            .blue();

        let new_file_ui: Text = Text::raw(NEW_FILE)
            .centered()
            .bold()
            .blue();

        new_file_ui.render(actions_area[0], buf);
        title.render(title_area, buf);
    }
}