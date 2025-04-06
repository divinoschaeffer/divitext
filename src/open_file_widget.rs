use crate::app::CurrentScreen;
use crate::error_type::ErrorType;
use crate::state::State;
use crate::text_area_popup_widget::text_area_popup;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};
use std::cell::RefCell;
use std::io;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;
use tui_textarea::{CursorMove, TextArea};

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
            return Ok(());
        }

        buffer.init(path.deref())?;
        state.push_buffer(buffer);
        state.current_screen = CurrentScreen::Editor;
        self.input.move_cursor(CursorMove::Head);
        self.input.delete_line_by_end();
        Ok(())
    }
}

impl Widget for &OpenFileWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
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

fn popup_area(area: Rect, max_x: u16, max_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Max(max_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Max(max_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
