use crate::buffer::Buffer;
use crate::display::Display;
use crossterm::cursor::MoveTo;
use crossterm::event::Event::Key;
use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, ExecutableCommand};
use std::cmp::PartialEq;
use std::io::Error;

const TAB_SIZE: u16 = 4;
pub struct Editor {
    pub display: Display,
    pub exit: bool,
    pub current_buffer: Buffer,
}

#[derive(PartialEq)]
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
                        self.handle_backspace_input()?;
                    },
                    KeyCode::Enter => {
                        self.handle_enter_input()?;
                    },
                    KeyCode::Tab => {
                        self.handle_tab_input()?;
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
        let (col, row) = cursor::position()?;
        match movement {
            CursorMovement::Up => {
                if row >= 1 {
                    if let Some((new_row, new_col)) = self.get_cursor_valid_position(row - 1, col, CursorMovement::Up) {
                        self.current_buffer.move_point_to(new_row, new_col);
                        self.display.stdout.execute(MoveTo(new_col, new_row))?;
                    }
                }
            }
            CursorMovement::Down => {
                if let Some((new_row, new_col)) = self.get_cursor_valid_position(row + 1, col, CursorMovement::Down) {
                    self.current_buffer.move_point_to(new_row, new_col);
                    self.display.stdout.execute(MoveTo(new_col, new_row))?;
                }
            }
            CursorMovement::Left => {
                if col >= 1 {
                    if let Some((new_row, new_col)) = self.get_cursor_valid_position(row, col - 1, CursorMovement::Left) {
                        self.current_buffer.move_point_to(new_row, new_col);
                        self.display.stdout.execute(MoveTo(new_col, new_row))?;
                    }
                }
            }
            CursorMovement::Right => {
                if let Some((new_row, new_col)) = self.get_cursor_valid_position(row, col + 1, CursorMovement::Right) {
                    self.current_buffer.move_point_to(new_row, new_col);
                    self.display.stdout.execute(MoveTo(new_col, new_row))?;
                }
            }
        }
        Ok(())
    }

    pub fn get_cursor_valid_position(&self, row: u16, col: u16, movement: CursorMovement) -> Option<(u16, u16)> {
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
                    match movement {
                        CursorMovement::Up => {
                            Some((row, *occupied))
                        },
                        CursorMovement::Down => {
                            Some((row, *occupied))
                        },
                        CursorMovement::Left => {
                            if row > 0 {
                                let last_position = occupied_positions[(row - 1) as usize];
                                if let Some(last_position) = last_position {
                                    Some((row - 1, last_position))
                                } else {
                                    Some((row - 1, 0))
                                }
                            } else {
                                None
                            }
                        },
                        CursorMovement::Right => {
                            if (row + 1) < occupied_positions.len() as u16 {
                                Some((row + 1, 0))
                            } else {
                                None
                            }
                        }
                    }
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
        let (col, row) = cursor::position()?;
        self.display.clear_and_print(self.current_buffer.content.clone())?;
        self.current_buffer.move_point_to(row, col + 1);
        self.display.stdout.execute(MoveTo(col + 1, row))?;
        Ok(())
    }

    pub fn handle_enter_input(&mut self) -> Result<(), Error> {
        let (col, row) = cursor::position()?;
        self.current_buffer.write_char('\n')?;
        self.current_buffer.move_point_to(row + 1, col);
        self.display.clear_and_print(self.current_buffer.content.clone())?;
        Ok(())
    }

    pub fn handle_backspace_input(&mut self) -> Result<(), Error> {
        let (col, row) = cursor::position()?;
        if row > 0 && col == 0 { // remove last character from previous line
            let new_row = row - 1;
            let new_col = self.current_buffer.get_last_column(new_row);
            self.current_buffer.move_point_to(new_row, new_col);
            self.current_buffer.remove_char()?;
            self.display.clear_and_print(self.current_buffer.content.clone())?;
            self.display.stdout.execute(MoveTo(new_col, new_row))?;
        } else if col > 0 {
            self.current_buffer.move_point_to(row, col - 1);
            self.current_buffer.remove_char()?;
            self.display.clear_and_print(self.current_buffer.content.clone())?;
            self.display.stdout.execute(MoveTo(col -1, row))?;
        }
        Ok(())
    }

    pub fn handle_tab_input(&mut self) -> Result<(), Error> {
        let (col, row) = cursor::position()?;
        for _i in 0..TAB_SIZE {
            self.current_buffer.write_char(' ')?
        }
        self.display.clear_and_print(self.current_buffer.content.clone())?;
        self.current_buffer.move_point_to(row, col + TAB_SIZE);
        self.display.stdout.execute(MoveTo(col + TAB_SIZE, row))?;
        Ok(())
    }
}