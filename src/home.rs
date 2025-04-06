use crate::state::State;
use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer as RatBuffer;
use ratatui::prelude::*;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

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

const NEW_FILE: &str = r##"+ New File    CRL ESP n"##;
const OPEN_FILE: &str = r##"- Open File   CRL ESP o"##;

#[derive(Debug)]
pub struct Home<'a> {
    pub state: Rc<RefCell<State<'a>>>,
}

impl<'a> Home<'a> {

    pub fn new(state: Rc<RefCell<State<'a>>>) -> Home<'a> {
        Self {
            state: state.clone(),
        }
    }

    pub fn handle_input(&mut self, _key: KeyEvent) -> Result<(), io::Error> {
        Ok(())
    }
}

impl Widget for &Home<'_> {
    fn render(self, area: Rect, buf: &mut RatBuffer)
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
                Constraint::Percentage(20),
            ])
            .split(main_area);

        let title: Text = Text::raw(NAME_TITLE.replace('.', " "))
            .centered();

        let new_file_ui: Text = Text::raw(NEW_FILE)
            .centered()
            .bold();

        let open_file_ui: Text = Text::raw(OPEN_FILE)
            .centered()
            .bold();

        new_file_ui.render(actions_area[0], buf);
        open_file_ui.render(actions_area[1], buf);
        title.render(title_area, buf);
    }
}
