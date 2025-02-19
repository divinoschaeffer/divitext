use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, ExecutableCommand};
use std::io::{stdout, Stdout};

pub struct Display {
    pub stdout: Stdout,
    pub width: u16,
    pub height: u16,
    pub first_line_visible: u16,
}

impl Display {
    pub fn new(stdout: Stdout, width: u16, height: u16, first_line_visible: u16) -> Display {
        Display {
            stdout,
            width,
            height,
            first_line_visible,
        }
    }

    pub fn default() -> Display {
        let (width, height): (u16, u16) = crossterm::terminal::size().unwrap();
        Display {
            stdout: stdout(),
            width,
            height,
            first_line_visible: 0,
        }
    }

    pub fn print_char(&mut self, c: char) {
        self.stdout.execute(Print(c)).unwrap();
    }

    pub fn print_string(&mut self, s: &str) -> Result<(), std::io::Error> {
        for c in s.chars() {
            if c == '\n' {
                self.stdout.execute(cursor::MoveToNextLine(1))?;
            } else {
                self.print_char(c);
            }
        }
        Ok(())
    }

    pub fn clear_and_print(&mut self, chars: Vec<u8>) -> Result<(), std::io::Error>{
        self.clear_all_display()?;
        let updated_content = String::from_utf8_lossy(chars.as_slice());
        self.print_string(&updated_content)?;
        Ok(())
    }

    pub fn clear_all_display(&mut self) -> Result<(), std::io::Error> {
        self.stdout.execute(Clear(ClearType::All))?;
        self.stdout.execute(cursor::MoveTo(0, 0))?;
        Ok(())
    }

    pub fn clear_display_before_cursor(&mut self) {
        self.stdout.execute(Clear(ClearType::FromCursorUp)).unwrap();
    }

    pub fn clear_display_after_cursor(&mut self) {
        self.stdout.execute(Clear(ClearType::FromCursorDown)).unwrap();
    }
}