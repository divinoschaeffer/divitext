use std::ffi::c_long;
use crate::buffer::{Buffer, MarkerMovement};
use crate::display::Display;
use crossterm::event::Event::Key;
use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, ExecutableCommand};
use std::io::{empty, stdout, Error, Stdout};
use crossterm::cursor::{MoveTo, MoveToNextLine, RestorePosition, SavePosition};
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
        self.handle_key_events()?;
        disable_raw_mode()?;
        self.display.stdout.execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn handle_key_events(&mut self) -> Result<(), Error> {
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
        Ok(())
    }

    pub fn handle_cursor_movement(&mut self, movement: CursorMovement) -> Result<(), Error> {
        let (col , line) = cursor::position()?;
        match movement {
            CursorMovement::Up => {
                if line >= 1 && self.is_cursor_position_valid((line - 1) as usize, col as usize) {
                    self.current_buffer.move_point_to(-1, 0);
                    self.display.stdout.execute(cursor::MoveUp(1))?;
                }
            }
            CursorMovement::Down => {
                if self.is_cursor_position_valid((line + 1) as usize, col as usize) {
                    self.current_buffer.move_point_to(1, 0);
                    self.display.stdout.execute(cursor::MoveDown(1))?;
                }
            }
            CursorMovement::Left => {
                if col >= 1 && self.is_cursor_position_valid(line as usize, (col - 1) as usize) {
                    self.current_buffer.move_point_to(0, -1);
                    self.display.stdout.execute(cursor::MoveLeft(1))?;
                }
            }
            CursorMovement::Right => {
                if self.is_cursor_position_valid(line as usize, (col + 1) as usize) {
                    self.current_buffer.move_point_to(0, 1);
                    self.display.stdout.execute(cursor::MoveRight(1))?;
                }
            }
        }
        Ok(())
    }

    pub fn is_cursor_position_valid(&self, x: usize, y: usize) -> bool {
        let occupied_positions: Vec<Option<usize>> = self.current_buffer.get_last_visible_char_position();

        if occupied_positions.is_empty() {
            return true;
        }

        if x >= occupied_positions.len() {
            return false;
        }

        match occupied_positions.get(x) {
            Some(Some(occupied)) => y <= occupied + 1,
            Some(None) => y == 0,
            None => false,
        }
    }


    pub fn handle_char_input(&mut self, c: char) -> Result<(), Error> {
        self.current_buffer.write_char(c)?;
        if self.current_buffer.move_point_to(0, 1) {
            let position = cursor::position()?;
            self.display.clear_all_display();
            let updated_content = String::from_utf8_lossy(&self.current_buffer.content);
            self.display.print_string(&updated_content);
            self.display.stdout.execute(MoveTo(position.0 + 1, position.1))?;
        }

        Ok(())
    }

    pub fn handle_enter_input(&mut self) -> Result<(), Error> {
        self.current_buffer.write_char('\n')?;
        self.current_buffer.move_point_to(1, 0);
        self.display.clear_all_display();
        let updated_content = String::from_utf8_lossy(&self.current_buffer.content);
        self.display.print_string(&updated_content);
        Ok(())
    }
}