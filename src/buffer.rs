use std::fs::OpenOptions;
use std::{io, result};
use std::io::{BufRead, BufReader};
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
        self.input = TextArea::new(result);
        self.filename = Some(String::from(filename));
        Ok(())
    }
}