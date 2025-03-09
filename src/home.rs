use std::cell::RefCell;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer as RatBuffer;
use ratatui::prelude::*;
use std::io;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;
use ratatui::layout::Flex;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use tui_textarea::TextArea;
use crate::app::CurrentScreen;
use crate::state::State;
use crate::buffer::Buffer;

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
enum ErrorMessage {
    FileNotFound,
    FileAlreadyExists,
}

impl ErrorMessage {
    fn message(&self) -> &'static str {
        match self {
            ErrorMessage::FileNotFound => "File not found",
            ErrorMessage::FileAlreadyExists => "File already exists",
        }
    }
}

#[derive(Debug)]
pub struct Home<'a> {
    pub state: Rc<RefCell<State<'a>>>,
    pub show_new_file_popup: bool,
    pub show_error_popup: bool,
    pub valid_input: bool,
    pub input: TextArea<'a>,
    pub error_message: ErrorMessage,
}

impl<'a> Home<'a> {

    pub fn new(state: Rc<RefCell<State<'a>>>) -> Home<'a> {
        Self {
            state,
            show_new_file_popup: false,
            show_error_popup: false,
            valid_input: false,
            input: Self::text_area_popup("Filename"),
            error_message: ErrorMessage::FileNotFound,
        }
    }
    pub fn handle_input(&mut self, key: KeyEvent) -> Result<(), io::Error> {
        if self.show_new_file_popup || self.show_error_popup {
            match key {
                KeyEvent { code: KeyCode::Enter, .. }
                | KeyEvent { code: KeyCode::Char('m'), modifiers: KeyModifiers::CONTROL, .. } => {
                    self.valid_input = true;
                    self.show_new_file_popup = false;
                    self.handle_create_file()?;
                    self.valid_input = false;
                },
                KeyEvent { code: KeyCode::Esc, .. } => {
                    self.show_new_file_popup = false;
                    self.show_error_popup = false;
                }
                _ => {
                    self.input.input(key);
                }
            }
        } else {
            match key {
                KeyEvent { code: KeyCode::Char('n'), modifiers: KeyModifiers::CONTROL, .. } => {
                    self.show_new_file_popup = true;
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn handle_create_file(&mut self) -> Result<(), io::Error> {
        let state = self.state.borrow_mut();
        let mut buffer = Buffer::default();
        let filename = self.input.lines().first().unwrap();

        if PathBuf::from(filename).is_file() {
            self.error_message = ErrorMessage::FileAlreadyExists;
            self.show_error_popup = true;
            return Ok(());
        }

        buffer.init(filename.deref())?;
        state.push_buffer(buffer);
        state.current_screen.replace(CurrentScreen::Editor);
        Ok(())
    }

    pub fn text_area_popup(title: &'a str) -> TextArea<'a> {
        let mut text_area = TextArea::default();
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

        let area = popup_area(area, 50, 3);

        if self.show_new_file_popup {
            Clear.render(area, buf);
            self.input.render(area, buf);
        } else if self.show_error_popup {
            Clear.render(area, buf);
            let block = Block::default().borders(Borders::ALL).title("Error");
            let text = Paragraph::new(self.error_message.message())
                .block(block)
                .centered()
                .bold();
            text.render(area, buf);
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
