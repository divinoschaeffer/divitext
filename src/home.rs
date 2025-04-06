use crate::new_file_widget::NewFileWidget;
use crate::state::State;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer as RatBuffer;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};
use std::cell::RefCell;
use std::io;
use std::ops::Deref;
use std::rc::Rc;
use tui_textarea::TextArea;

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
const OPEN_FILE: &str = r##"- Open File      CRL o"##;
#[derive(Debug, PartialEq)]
pub enum CurrentPopup {
    None,
    NewFile,
    OpenFile,
}

#[derive(Debug)]
pub struct Home<'a> {
    pub state: Rc<RefCell<State<'a>>>,
    pub show_popup: bool,
    pub current_popup: CurrentPopup,
    pub new_file_widget: NewFileWidget<'a>,
}

impl<'a> Home<'a> {

    pub fn new(state: Rc<RefCell<State<'a>>>) -> Home<'a> {
        Self {
            state: state.clone(),
            show_popup: false,
            new_file_widget: NewFileWidget::new(state),
            current_popup: CurrentPopup::None,
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> Result<(), io::Error> {
        if self.current_popup != CurrentPopup::None {
            match key {
                KeyEvent { code: KeyCode::Enter, .. }
                | KeyEvent { code: KeyCode::Char('m'), modifiers: KeyModifiers::CONTROL, .. } => {
                    match self.current_popup {
                        CurrentPopup::None => (),
                        CurrentPopup::OpenFile => {
                            self.show_popup = false;
                        },
                        CurrentPopup::NewFile => {
                            self.new_file_widget.create_new_file()?;
                            self.show_popup = false;
                        },
                    }
                },
                KeyEvent { code: KeyCode::Esc, .. } => {
                    self.show_popup = false;
                    self.current_popup = CurrentPopup::None;
                }
                _ => {
                    match self.current_popup {
                        CurrentPopup::None => (),
                        CurrentPopup::OpenFile => {

                        },
                        CurrentPopup::NewFile => {
                            self.new_file_widget.input.input(key);
                        }
                    }
                }
            }
        } else {
            match key {
                KeyEvent { code: KeyCode::Char('n'), modifiers: KeyModifiers::CONTROL, .. } => {
                    self.show_popup = true;
                    self.current_popup = CurrentPopup::NewFile;
                },
                KeyEvent { code: KeyCode::Char('o'), modifiers: KeyModifiers::CONTROL, .. } => {
                    self.show_popup = true;
                    self.current_popup = CurrentPopup::OpenFile;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /*pub fn handle_open_file(&mut self) -> Result<(), io::Error> {
        let mut state = self.state.borrow_mut();
        let mut buffer = Buffer::default();

        let filename = self.input.lines().first().unwrap();

        if !PathBuf::from(filename).is_file() {
            self.error_message = ErrorMessage::FileNotFound;
            self.show_popup = true;
            self.current_popup = CurrentPopup::Error;
            return Ok(());
        }

        buffer.init(filename.deref())?;
        state.push_buffer(buffer);
        state.current_screen = CurrentScreen::Editor;
        Ok(())
    }*/

    pub fn text_area_popup(title: &'a str) -> TextArea<'a> {
        let mut text_area = TextArea::default();
        text_area.set_cursor_line_style(Style::default());
        text_area.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
        );
        text_area
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

        if self.show_popup {
            match self.current_popup {
                CurrentPopup::None => (),
                CurrentPopup::OpenFile => {}
                CurrentPopup::NewFile => {
                    self.new_file_widget.render(area, buf);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;
    use crossterm::event::{KeyEventKind, KeyEventState};

    #[test]
    fn test_home_initialization() {
        let state = Rc::new(RefCell::new(State::default()));
        let home = Home::new(state.clone());

        assert_eq!(home.show_popup, false);
        assert_eq!(home.current_popup, CurrentPopup::None);
    }

    #[test]
    fn test_handle_input_new_file_shortcut() {
        let state = Rc::new(RefCell::new(State::default()));
        let mut home = Home::new(state.clone());

        home.handle_input(KeyEvent {
            code: KeyCode::Char('n'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }).unwrap();

        assert_eq!(home.show_popup, true);
        assert_eq!(home.current_popup, CurrentPopup::NewFile);
    }

    #[test]
    fn test_handle_input_open_file_shortcut() {
        let state = Rc::new(RefCell::new(State::default()));
        let mut home = Home::new(state.clone());

        home.handle_input(KeyEvent {
            code: KeyCode::Char('o'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }).unwrap();

        assert_eq!(home.show_popup, true);
        assert_eq!(home.current_popup, CurrentPopup::OpenFile);
    }

    #[test]
    fn test_handle_input_escape_closes_popup() {
        let state = Rc::new(RefCell::new(State::default()));
        let mut home = Home::new(state.clone());

        home.show_popup = true;
        home.current_popup = CurrentPopup::NewFile;

        home.handle_input(KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }).unwrap();

        assert_eq!(home.show_popup, false);
        assert_eq!(home.current_popup, CurrentPopup::None);
    }
}
