use std::io::{stdout, Stdout};
use crossterm::event::Event::Key;
use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{cursor, ExecutableCommand};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, window_size, EnterAlternateScreen, LeaveAlternateScreen, WindowSize};

pub struct Editor {
    size: WindowSize,
    stdout: Stdout,
    pub exit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            size: window_size().unwrap(),
            stdout: stdout(),
            exit: false,
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
                    },
                    KeyCode::Right => {
                        self.stdout.execute(cursor::MoveRight(1))?;
                    },
                    KeyCode::Left => {
                        self.stdout.execute(cursor::MoveLeft(1))?;
                    },
                    KeyCode::Up => {
                        self.stdout.execute(cursor::MoveUp(1))?;
                    },
                    KeyCode::Down => {
                        self.stdout.execute(cursor::MoveDown(1))?;
                    },
                    KeyCode::Backspace => {
                        self.stdout.execute(cursor::MoveLeft(1))?;
                        self.stdout.execute(Print(' '))?;
                        self.stdout.execute(cursor::MoveLeft(1))?;
                    },
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
}