use std::ffi::c_long;
use crate::buffer::{Buffer, MarkerMovement};
use crate::display::Display;
use crossterm::event::Event::Key;
use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, ExecutableCommand};
use std::io::{stdout, Stdout};
use log::info;

pub struct Editor {
    pub display: Display,
    pub stdout: Stdout,
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
            stdout: stdout(),
            exit: false,
            current_buffer: Buffer::default(),
        }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        self.stdout.execute(EnterAlternateScreen)?;
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
                        self.stdout.execute(Print(c))?;
                        self.current_buffer.content.push(u8::try_from(c).unwrap());
                        self.current_buffer.move_point_to(0, 1);
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
                        self.stdout.execute(cursor::MoveLeft(1))?;
                        self.stdout.execute(Print(' '))?;
                        self.stdout.execute(cursor::MoveLeft(1))?;
                    },
                    KeyCode::Enter => {
                        self.current_buffer.content.push(u8::try_from('\n').unwrap());
                        self.stdout.execute(Print('\n'))?;
                    }
                    _ => ()
                };
            };
            if self.exit {
                break;
            };
        }
        disable_raw_mode()?;
        self.stdout.execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn handle_cursor_movement(&mut self, movement: CursorMovement) -> Result<(), std::io::Error> {
        match movement {
            CursorMovement::Up => {
                if self.current_buffer.move_point_to(-1, 0) {
                    self.stdout.execute(cursor::MoveUp(1))?;
                }
            }
            CursorMovement::Down => {
                if self.current_buffer.move_point_to(1, 0) {
                    self.stdout.execute(cursor::MoveDown(1))?;
                }
            }
            CursorMovement::Left => {
                if self.current_buffer.move_point_to(0, -1) {
                    self.stdout.execute(cursor::MoveLeft(1))?;
                }
            }
            CursorMovement::Right => {
                if self.current_buffer.move_point_to(0, 1) {
                    self.stdout.execute(cursor::MoveRight(1))?;
                }
            }
        }
        info!("buffer: {:?}, len : {:?}", String::from_utf8(self.current_buffer.content.clone()), self.current_buffer.content.len());
        info!("buffer: {:?}", self.current_buffer.point);
        Ok(())
    }
}