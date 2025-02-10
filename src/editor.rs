use std::io::{stdout, Stdout};
use crossterm::event::Event::Key;
use crossterm::event::{read, KeyCode, KeyEvent, KeyModifiers};
use crossterm::ExecutableCommand;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

pub struct Editor {
    stdout: Stdout,
    pub exit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self {
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
}