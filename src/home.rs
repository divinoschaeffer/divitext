use crate::app::CurrentScreen;
use crate::buffer::Buffer;
use crate::state::State;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer as RatBuffer;
use ratatui::layout::Flex;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use std::cell::RefCell;
use std::io;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;
use tui_textarea::{CursorMove, TextArea};

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

#[derive(Debug)]
pub enum ErrorMessage {
    FileNotFound,
    FileAlreadyExists,
}

#[derive(Debug, PartialEq)]
pub enum CurrentPopup {
    None,
    NewFile,
    OpenFile,
    Error,
}

impl ErrorMessage {
    pub fn message(&self) -> &'static str {
        match self {
            ErrorMessage::FileNotFound => "File not found",
            ErrorMessage::FileAlreadyExists => "File already exists",
        }
    }
}

#[derive(Debug)]
pub struct Home<'a> {
    pub state: Rc<RefCell<State<'a>>>,
    pub show_popup: bool,
    pub current_popup: CurrentPopup,
    pub valid_input: bool,
    pub input: TextArea<'a>,
    pub error_message: ErrorMessage,
}

impl<'a> Home<'a> {

    pub fn new(state: Rc<RefCell<State<'a>>>) -> Home<'a> {
        Self {
            state,
            valid_input: false,
            show_popup: false,
            current_popup: CurrentPopup::None,
            input: Self::text_area_popup("Filename"),
            error_message: ErrorMessage::FileNotFound,
        }
    }
    pub fn handle_input(&mut self, key: KeyEvent) -> Result<(), io::Error> {
        if self.current_popup != CurrentPopup::None {
            match key {
                KeyEvent { code: KeyCode::Enter, .. }
                | KeyEvent { code: KeyCode::Char('m'), modifiers: KeyModifiers::CONTROL, .. } => {
                    self.valid_input = true;
                    match self.current_popup {
                        CurrentPopup::None => (),
                        CurrentPopup::OpenFile => {
                            self.handle_open_file()?;
                            self.show_popup = false;
                        },
                        CurrentPopup::NewFile => {
                            self.handle_create_file()?;
                            self.show_popup = false;
                        },
                        CurrentPopup::Error => ()
                    }
                    self.reset_input();
                    self.valid_input = false;
                },
                KeyEvent { code: KeyCode::Esc, .. } => {
                    self.show_popup = false;
                    self.current_popup = CurrentPopup::None;
                }
                _ => {
                    self.input.input(key);
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

    pub fn handle_open_file(&mut self) -> Result<(), io::Error> {
        let state = self.state.borrow_mut();
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
        state.current_screen.replace(CurrentScreen::Editor);
        Ok(())
    }

    pub fn handle_create_file(&mut self) -> Result<(), io::Error> {
        let state = self.state.borrow_mut();
        let mut buffer = Buffer::default();
        let filename = self.input.lines().first().unwrap();

        if PathBuf::from(filename).is_file() {
            self.error_message = ErrorMessage::FileAlreadyExists;
            self.show_popup = true;
            self.current_popup = CurrentPopup::Error;
            return Ok(());
        }

        buffer.init(filename.deref())?;
        state.push_buffer(buffer);
        state.current_screen.replace(CurrentScreen::Editor);
        Ok(())
    }

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

    pub fn reset_input(&mut self) {
        self.input.move_cursor(CursorMove::Head);
        self.input.delete_line_by_end();
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
            .centered()
            .blue();

        let new_file_ui: Text = Text::raw(NEW_FILE)
            .centered()
            .bold()
            .blue();

        let open_file_ui: Text = Text::raw(OPEN_FILE)
            .centered()
            .bold()
            .blue();

        new_file_ui.render(actions_area[0], buf);
        open_file_ui.render(actions_area[1], buf);
        title.render(title_area, buf);

        let area = popup_area(area, 50, 3);

        match self.current_popup {
            CurrentPopup::NewFile | CurrentPopup::OpenFile => {
                Clear.render(area, buf);
                self.input.render(area, buf);
            },
            CurrentPopup::Error => {
                Clear.render(area, buf);
                let block = Block::default().borders(Borders::ALL);
                let text = Paragraph::new(self.error_message.message())
                    .block(block)
                    .centered()
                    .bold();
                text.render(area, buf);
            },
            _ => ()
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
