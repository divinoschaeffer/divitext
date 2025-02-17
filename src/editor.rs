use crate::buffer::Buffer;
use crate::display::Display;
use crossterm::cursor::MoveTo;
use crossterm::event::Event::Key;
use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, ExecutableCommand};
use std::io::Error;

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
        let (col , row) = cursor::position()?;
        match movement {
            CursorMovement::Up => {
                if row >= 1 {
                    if let Some((new_row, new_col)) = self.get_cursor_valid_position(row - 1, col) {
                        self.current_buffer.move_point_to(-1, 0);
                        self.display.stdout.execute(MoveTo(new_col, new_row))?;
                    }
                }
            }
            CursorMovement::Down => {
                if let Some((new_row, new_col)) = self.get_cursor_valid_position(row + 1, col) {
                    self.current_buffer.move_point_to(1, 0);
                    self.display.stdout.execute(MoveTo(new_col, new_row))?;
                }
            }
            CursorMovement::Left => {
                if col >= 1 {
                    if let Some((new_row, new_col)) = self.get_cursor_valid_position(row, col - 1) {
                        self.current_buffer.move_point_to(0, -1);
                        self.display.stdout.execute(MoveTo(new_col, new_row))?;
                    }
                }
            }
            CursorMovement::Right => {
                if let Some((new_row, new_col)) = self.get_cursor_valid_position(row, col + 1) {
                    self.current_buffer.move_point_to(0, 1);
                    self.display.stdout.execute(MoveTo(new_col, new_row))?;
                }
            }
        }
        Ok(())
    }

    pub fn get_cursor_valid_position(&self, row: u16, col: u16) -> Option<(u16, u16)> {
        let occupied_positions: Vec<Option<u16>> = self.current_buffer.get_last_visible_char_position();

        if occupied_positions.is_empty() {
            return Some((row, col))
        }

        if row >= occupied_positions.len() as u16 {
            return None;
        }

        match occupied_positions.get(row as usize) {
            Some(Some(occupied)) => {
                if col <= occupied + 1 {
                    Some((row, col))
                } else {
                    Some((row, *occupied))
                }
            },
            Some(None) => {
                Some((row, 0))
            },
            None => None,
        }
    }

    pub fn is_cursor_position_valid(&self, row: u16, col: u16) -> bool {
        let occupied_positions: Vec<Option<u16>> = self.current_buffer.get_last_visible_char_position();

        if occupied_positions.is_empty() {
            return true;
        }

        if row >= occupied_positions.len() as u16 {
            return false;
        }

        match occupied_positions.get(row as usize) {
            Some(Some(occupied)) => col <= occupied + 1,
            Some(None) => col == 0,
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