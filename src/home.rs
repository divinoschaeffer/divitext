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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_home_new() {
        let state = Rc::new(RefCell::new(State::default()));
        let _home = Home::new(Rc::clone(&state));
        assert_eq!(Rc::strong_count(&state), 2);
    }

    #[test]
    fn test_home_handle_input_returns_ok() {
        let state = Rc::new(RefCell::new(State::default()));
        let mut home = Home::new(Rc::clone(&state));
        let dummy_key = KeyEvent::from(crossterm::event::KeyCode::Char('n'));
        let result = home.handle_input(dummy_key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_home_render_does_not_panic() {
        let state = Rc::new(RefCell::new(State::default()));
        let home = Home::new(Rc::clone(&state));
        let mut buffer = RatBuffer::empty(Rect::new(0, 0, 120, 40));
        let area = Rect::new(0, 0, 120, 40);

        (&home).render(area, &mut buffer);
    }
}
