use crate::app::CurrentScreen;
use crate::error_type::ErrorType;
use crate::state::State;
use crate::text_area_popup_widget::text_area_popup;
use ratatui::buffer::Buffer;
use ratatui::layout::{Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};
use std::cell::RefCell;
use std::io;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;
use crossterm::event::KeyEvent;
use log::error;
use tui_textarea::{CursorMove, TextArea};
use crate::action_bar::ActionWidget;
use crate::popup::{popup_area};

const POPUP_TITLE: &str = "Filename";

#[derive(Debug)]
pub struct OpenFileWidget<'a> {
    pub state: Rc<RefCell<State<'a>>>,
    pub input: TextArea<'a>,
    pub error: ErrorType,
}

impl<'a> OpenFileWidget<'a> {
    pub fn new(state: Rc<RefCell<State<'a>>>) -> OpenFileWidget<'a> {
        OpenFileWidget { state,  input: text_area_popup(POPUP_TITLE), error: ErrorType::NONE }
    }

    pub fn open_file(&mut self) -> io::Result<()> {
        let mut state = self.state.borrow_mut();
        let mut buffer = crate::buffer::Buffer::default();
        let path = self.input.lines().first().unwrap();

        if !PathBuf::from(path).is_file() {
            self.error = ErrorType::FileNotFound;
            error!("File not found: {}", path);
            return Ok(());
        }

        buffer.init(path.deref())?;
        state.push_buffer(buffer);
        state.current_screen = CurrentScreen::Editor;
        self.input.move_cursor(CursorMove::Head);
        self.input.delete_line_by_end();
        Ok(())
    }

    fn render_content(&self, area: Rect, buf: &mut Buffer) {
        let pop_up_area = popup_area(area, 50, 3);
        Clear.render(pop_up_area, buf);
        if self.error == ErrorType::NONE {
            self.input.render(pop_up_area, buf);
        } else {
            let block = Block::default().borders(Borders::ALL);
            let text = Paragraph::new(self.error.to_string())
                .block(block)
                .centered()
                .bold();
            text.render(pop_up_area, buf);
        }
    }
}

impl ActionWidget for OpenFileWidget<'_> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.render_content(area, buf);
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<(), io::Error> {
        if self.error != ErrorType::NONE {
            self.error = ErrorType::NONE;
        } else {
            self.input.input(key);
        }
        Ok(())
    }

    fn has_error(&self) -> bool {
        self.error != ErrorType::NONE
    }

    fn process_action(&mut self) -> Result<bool, io::Error> {
        self.open_file()?;
        Ok(self.error == ErrorType::NONE)
    }

    fn init_action(&mut self) {}

    fn reset(&mut self) {
        self.error = ErrorType::NONE;
        self.input.move_cursor(CursorMove::Head);
        self.input.delete_line_by_end();
    }
}

impl Widget for &OpenFileWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        self.render_content(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;
    use std::cell::RefCell;
    use std::fs;
    use std::path::Path;
    use std::rc::Rc;

    fn create_widget_with_input(input: &str) -> OpenFileWidget<'static> {
        let state = Rc::new(RefCell::new(State::default()));
        let mut widget = OpenFileWidget::new(Rc::clone(&state));
        widget.input.insert_str(input);
        widget
    }

    #[test]
    fn test_open_file_success() {
        let file_path = "test_open_success.txt";
        fs::write(file_path, "dummy content").unwrap();

        let mut widget = create_widget_with_input(file_path);
        let result = widget.open_file();

        assert!(result.is_ok());
        assert_eq!(widget.error, ErrorType::NONE);
        assert!(Path::new(file_path).exists());
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_open_file_not_found() {
        let file_path = "non_existing_file.txt";
        if Path::new(file_path).exists() {
            fs::remove_file(file_path).unwrap();
        }

        let mut widget = create_widget_with_input(file_path);
        let result = widget.open_file();

        assert!(result.is_ok());
        assert_eq!(widget.error, ErrorType::FileNotFound);
    }

    #[test]
    fn test_popup_area_is_centered() {
        let area = Rect::new(0, 0, 100, 30);
        let popup = popup_area(area, 50, 3);
        assert_eq!(popup.width, 50);
        assert_eq!(popup.height, 3);
        assert!(popup.x > 0);
        assert!(popup.y > 0);
    }
}
