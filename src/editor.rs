use crate::buffer::Buffer;
use crate::state::State;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Widget;
use ratatui::style::Stylize;
use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::rc::Rc;
use ratatui::text::Text;
use tui_textarea::TextArea;

const FILE_SUCCESSFULLY_SAVED:&str = "File saved successfully !";

#[derive(Debug)]
pub struct Editor<'a> {
    pub state: Rc<RefCell<State<'a>>>,
    pub show_success_save: bool,
}

impl<'a> Editor<'a> {
    pub fn new(state: Rc<RefCell<State<'a>>>) -> Editor<'a> {
        Self {
            state,
            show_success_save: false,
        }
    }

    pub fn init(&mut self, file_path: Option<&String>) -> Result<(), io::Error> {
        let mut state = self.state.borrow_mut();

        if let Some(filename) = file_path {
            let mut buffer = Buffer::default();
            buffer.init(filename)?;

            state.buffer_list.push(buffer);
            state.current_buffer = state.buffer_list.len() - 1;
        } else {
            let buffer = Buffer::new(TextArea::default(), None);
            state.buffer_list.push(buffer);
        }
        Ok(())
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> Result<(), io::Error> {
        match key {
            KeyEvent { code: KeyCode::Char('s'), modifiers: KeyModifiers::CONTROL, .. } => {
                self.save_current_buffer()?;
                self.show_success_save = true;
            }
            _ => {
                if self.show_success_save {
                    self.show_success_save = false;
                }
                self.handle_input_current_buffer(key)
            }
        }

        Ok(())
    }

    pub fn handle_input_current_buffer(&self, key: KeyEvent) {
        let mut state = self.state.borrow_mut();
        let index = state.current_buffer;
        state.buffer_list[index].input.input(key);
    }

    pub fn get_current_buffer(&self) -> Buffer {
        let state = self.state.borrow();
        state.buffer_list[state.current_buffer].clone()
    }

    pub fn get_buffer_list(&self) -> Vec<Buffer> {
        let state = self.state.borrow();
        state.buffer_list.clone()
    }

    pub fn save_current_buffer(&self) -> Result<(), io::Error> {
        let state = self.state.borrow();

        let content = state.buffer_list[state.current_buffer].input.lines().join("\n");
        let filename = state.buffer_list[state.current_buffer]
            .clone()
            .filename
            .unwrap()
            .clone();

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(filename)?;

        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

impl Widget for &Editor<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Max(1),
            ])
            .split(area);

        if !self.get_buffer_list().is_empty() {
            self.get_current_buffer().input.render(layout[0], buf);
        }
        if self.show_success_save {
            let message = Text::raw(FILE_SUCCESSFULLY_SAVED)
                .black()
                .on_white()
                .bold()
                .centered();
            message.render(layout[1], buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, KeyEventKind, KeyEventState};
    use std::cell::RefCell;
    use std::fs::{self, File};
    use std::rc::Rc;

    #[test]
    fn test_editor_init_with_file() {
        let state = Rc::new(RefCell::new(State::default()));
        let mut editor = Editor::new(Rc::clone(&state));

        let file_path = "test_file.txt";
        File::create(file_path).unwrap();
        editor.init(Some(&file_path.to_string())).unwrap();

        let buffer_list = editor.get_buffer_list();
        assert_eq!(buffer_list.len(), 1);
        assert_eq!(buffer_list[0].filename, Some(file_path.to_string()));

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_editor_init_without_file() {
        let state = Rc::new(RefCell::new(State::default()));
        let mut editor = Editor::new(Rc::clone(&state));
        editor.init(None).unwrap();

        let buffer_list = editor.get_buffer_list();
        assert_eq!(buffer_list.len(), 1);
        assert!(buffer_list[0].filename.is_none());
    }

    #[test]
    fn test_handle_input_save_file() {
        let state = Rc::new(RefCell::new(State::default()));
        let mut editor = Editor::new(Rc::clone(&state));
        let file_path = "test_save.txt";

        File::create(file_path).unwrap();
        editor.init(Some(&file_path.to_string())).unwrap();

        let key_event_write = KeyEvent {
            code: KeyCode::Char('H'),
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        };
        editor.handle_input(key_event_write).unwrap();

        let key_event_save = KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        };
        editor.handle_input(key_event_save).unwrap();

        assert!(editor.show_success_save);

        let saved_content = fs::read_to_string(file_path).unwrap();
        assert_eq!(saved_content, "H");

        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_handle_input_current_buffer() {
        let state = Rc::new(RefCell::new(State::default()));
        let mut editor = Editor::new(Rc::clone(&state));
        editor.init(None).unwrap();

        let key_event = KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        };

        editor.handle_input(key_event).unwrap();
        let buffer = editor.get_current_buffer();
        assert_eq!(buffer.input.lines(), vec!["a"]);
    }

    #[test]
    fn test_success_message_resets_on_other_input() {
        let state = Rc::new(RefCell::new(State::default()));
        let mut editor = Editor::new(Rc::clone(&state));
        editor.init(None).unwrap();

        editor.show_success_save = true;

        let key_event = KeyEvent {
            code: KeyCode::Char('x'),
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        };

        editor.handle_input(key_event).unwrap();
        assert!(!editor.show_success_save);
    }

    #[test]
    fn test_get_current_buffer_returns_expected_buffer() {
        let state = Rc::new(RefCell::new(State::default()));
        let mut editor = Editor::new(Rc::clone(&state));
        editor.init(None).unwrap();

        let buffer = editor.get_current_buffer();
        assert!(buffer.filename.is_none());
    }
}
