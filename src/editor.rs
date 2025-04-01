use crate::buffer::Buffer;
use crate::state::State;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Widget;
use ratatui::style::Stylize;
use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io;
use std::io::{Write};
use std::ops::Deref;
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

    pub fn init(&mut self, file_path: Option<&String>) ->Result<(), io::Error> {
        let state = self.state.borrow_mut();
        let mut buffer_list = state.buffer_list.borrow_mut();
        let mut current_buffer = state.current_buffer.borrow_mut();

        if let Some(filename) = file_path {
            let mut buffer = Buffer::default();
            buffer.init(filename)?;

            buffer_list.push(buffer);
            *current_buffer = buffer_list.len() - 1;
        } else {
            let buffer = Buffer::new(TextArea::default(), None);
            buffer_list.push(buffer);
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
        let state = self.state.borrow_mut();
        let mut buffer_list = state.buffer_list.borrow_mut();
        let current_buffer = state.current_buffer.borrow_mut();
        buffer_list[*current_buffer].input.input(key);
    }

    pub fn get_current_buffer(&self) -> Buffer {
        let state = self.state.borrow();
        let buffer_list = state.buffer_list.borrow();
        let current_buffer = state.current_buffer.borrow();
        buffer_list[*current_buffer].clone()
    }

    pub fn get_buffer_list(&self) -> Vec<Buffer> {
        let state = self.state.borrow();
        let list = state.buffer_list.borrow().deref().clone();
        list
    }

    pub fn save_current_buffer(&self) -> Result<(), io::Error> {
        let state = self.state.borrow();
        let buffer_list = state.buffer_list.borrow();
        let current_buffer = state.current_buffer.borrow();

        let content  = buffer_list[*current_buffer].input.lines().join("\n");
        let filename = buffer_list[*current_buffer].clone().filename.unwrap().clone();

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(filename)?;

        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

impl Widget for &Editor<'_>{
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer)
    {
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
    use crate::state::State;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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

        let key_event = KeyEvent {
            code: KeyCode::Char(' '),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::empty(),
        };

        editor.handle_input(key_event).unwrap();
        assert!(editor.show_success_save);

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
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::empty(),
        };

        editor.handle_input(key_event).unwrap();
        let buffer = editor.get_current_buffer();
        assert_eq!(buffer.input.lines(), vec!["a"]);
    }
}
