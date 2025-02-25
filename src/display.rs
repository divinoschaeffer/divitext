use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute, ExecutableCommand};
use std::io::{stdout, Stdout};
use crossterm::cursor::MoveTo;
use crossterm::style::Color::{Black, White};

#[derive(Debug)]
pub struct Display {
    pub stdout: Stdout,
    pub width: u16,
    pub height: u16,
    pub first_line_visible: u16,
}

impl Default for Display {
    fn default() -> Self {
        let (width, height): (u16, u16) = crossterm::terminal::size().unwrap();
        Self {
            stdout: stdout(),
            width,
            height,
            first_line_visible: 0,
        }
    }
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

    pub fn clear_and_print(&mut self, chars: String) -> Result<(), std::io::Error>{
        self.clear_all_display()?;
        self.print_string(&chars)?;
        Ok(())
    }

    pub fn clear_all_display(&mut self) -> Result<(), std::io::Error> {
        self.stdout.execute(Clear(ClearType::All))?;
        self.stdout.execute(MoveTo(0, 0))?;
        Ok(())
    }

    pub fn clear_display_before_cursor(&mut self) {
        self.stdout.execute(Clear(ClearType::FromCursorUp)).unwrap();
    }

    pub fn clear_display_after_cursor(&mut self) {
        self.stdout.execute(Clear(ClearType::FromCursorDown)).unwrap();
    }

    pub fn get_displayable_lines(& self) -> Result<(u16, u16), std::io::Error> {
        Ok((self.first_line_visible, self.first_line_visible + self.height))
    }

    pub fn print_save_validation(&mut self) -> Result<(), std::io::Error> {
        execute!(
            self.stdout,
            MoveTo(0, self.height - 1),
            SetForegroundColor(Black),
            SetBackgroundColor(White),
            Print("Save file ? Y/N"),
            ResetColor,
        )?;
        Ok(())
    }

    pub fn print_filename_input(&mut self) -> Result<(), std::io::Error>{
        execute!(
            self.stdout,
            MoveTo(0, 0),
            Print("Enter a filename"),
        )?;
        Ok(())
    }
}