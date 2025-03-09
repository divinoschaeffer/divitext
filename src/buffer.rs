use std::fs::OpenOptions;
use std::{io, result};
use std::io::{BufRead, BufReader};
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders};
use tui_textarea::TextArea;

#[derive(Debug, Clone)]
pub struct Buffer<'a>{
    pub input: TextArea<'a>,
    pub filename: Option<String>,
}

impl<'a> Default for Buffer<'a>{
    fn default() -> Buffer<'a>{
        Buffer {
            input: TextArea::default(),
            filename: None,
        }
    }
}

impl<'a> Buffer<'a> {
    pub fn new(input: TextArea<'a>, filename: Option<String>) -> Buffer<'a> {
        Buffer { input, filename }
    }

    pub fn init(&mut self, filename: &str) -> Result<(), io::Error> {
        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(&filename)?;

        let result = BufReader::new(file).lines().collect::<io::Result<_>>()?;
        self.input = self.custom_text_area(result);
        self.filename = Some(String::from(filename));
        Ok(())
    }

    pub fn custom_text_area(&self, lines: Vec<String>) -> TextArea<'a>{
        let mut text_area = TextArea::new(lines);
        text_area.set_cursor_line_style(Style::default());
        text_area.set_line_number_style(Style::default().fg(Color::DarkGray));
        text_area
    }
}