use std::ffi::c_long;
use crate::buffer::{Buffer, MarkerMovement};
use crate::display::Display;
use crossterm::event::Event::Key;
use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, ExecutableCommand};
use std::io::{stdout, Error, Stdout};
use crossterm::cursor::{MoveToNextLine, SavePosition};
use log::{error, info, warn};

pub struct Editor {
    pub display: Display,
    pub exit: bool,
    pub current_buffer: Buffer,
}

pub enum CursorMovement {
    Up,
    Down,
    Left,
    Right,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            display: Display::default(),
            exit: false,
            current_buffer: Buffer::default(),
        }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        self.display.stdout.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        loop {
            if let Key(KeyEvent {
                           code, modifiers, kind, state
                       }) = read()?
            {
                match code {
                    KeyCode::Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.exit = true;
                    },
                    KeyCode::Char(c) if KeyModifiers::is_empty(&modifiers) => {
                        self.handle_char_input(c)?;
                    },
                    KeyCode::Right => {
                        self.handle_cursor_movement(CursorMovement::Right)?;
                    },
                    KeyCode::Left => {
                        self.handle_cursor_movement(CursorMovement::Left)?;
                    },
                    KeyCode::Up => {
                        self.handle_cursor_movement(CursorMovement::Up)?;
                    },
                    KeyCode::Down => {
                        self.handle_cursor_movement(CursorMovement::Down)?;
                    },
                    KeyCode::Backspace => {
                        self.display.stdout.execute(cursor::MoveLeft(1))?;
                        self.current_buffer.write_char(' ')?;
                        self.display.stdout.execute(cursor::MoveLeft(1))?;
                    },
                    KeyCode::Enter => {
                        self.handle_enter_input()?;
                    }
                    _ => ()
                };
            };
            if self.exit {
                break;
            };
        }
        disable_raw_mode()?;
        self.display.stdout.execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn handle_cursor_movement(&mut self, movement: CursorMovement) -> Result<(), std::io::Error> {
        match movement {
            CursorMovement::Up => {
                if self.current_buffer.move_point_to(-1, 0) {
                    self.display.stdout.execute(cursor::MoveUp(1))?;
                }
            }
            CursorMovement::Down => {
                if self.current_buffer.move_point_to(1, 0) {
                    self.display.stdout.execute(cursor::MoveDown(1))?;
                }
            }
            CursorMovement::Left => {
                if self.current_buffer.move_point_to(0, -1) {
                    self.display.stdout.execute(cursor::MoveLeft(1))?;
                }
            }
            CursorMovement::Right => {
                if self.current_buffer.move_point_to(0, 1) {
                    self.display.stdout.execute(cursor::MoveRight(1))?;
                }
            }
        }
        info!("buffer: {:?}, len: {:?}, point: {:?}", String::from_utf8(self.current_buffer.content.clone()), self.current_buffer.content.len(), self.current_buffer.point);
        Ok(())
    }

    pub fn handle_char_input(&mut self, c: char) -> Result<(), Error> {
        self.display.print_char(c);
        self.current_buffer.write_char(c)?;
        self.current_buffer.move_point_to(0, 1);
        Ok(())
    }

    pub fn handle_enter_input(&mut self) -> Result<(), Error> {
        // TODO : handle enter when insert
        self.current_buffer.write_char('\n')?;
        self.current_buffer.move_point_to(1, 0);
        self.display.stdout.execute(MoveToNextLine(1))?;
        Ok(())
    }
}