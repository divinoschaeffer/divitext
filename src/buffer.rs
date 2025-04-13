use ratatui::prelude::{Color, Style};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::io;
use std::path::Path;
use tui_textarea::TextArea;

#[derive(Debug, Clone, Default)]
pub struct Buffer<'a>{
    pub input: TextArea<'a>,
    pub filename: Option<String>,
    pub path: Option<String>
}

impl<'a> Buffer<'a> {
    pub fn new(input: TextArea<'a>, path: Option<String>) -> Buffer<'a> {
        Buffer { input, filename: None, path }
    }

    pub fn init(&mut self, path: &str) -> Result<(), io::Error> {
        let filename = Path::new(path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(path)?;

        let result = BufReader::new(file).lines().collect::<io::Result<_>>()?;
        self.input = self.custom_text_area(result);
        self.path = Some(String::from(path));
        self.filename = Some(filename);
        Ok(())
    }

    pub fn custom_text_area(&self, lines: Vec<String>) -> TextArea<'a>{
        let mut text_area = TextArea::new(lines);
        text_area.set_cursor_line_style(Style::default());
        text_area.set_line_number_style(Style::default().fg(Color::DarkGray));
        text_area
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_buffer_default() {
        let buffer = Buffer::default();
        assert!(buffer.filename.is_none());
    }

    #[test]
    fn test_buffer_new() {
        let textarea = TextArea::new(vec!["Hello, world!".to_string()]);
        let buffer = Buffer::new(textarea.clone(), Some("test.txt".to_string()));
        assert_eq!(buffer.path, Some("test.txt".to_string()));
        assert_eq!(buffer.input.lines(), textarea.lines());
    }

    #[test]
    fn test_buffer_init() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        writeln!(temp_file, "Hello, world!").expect("Failed to write to temp file");
        let path = temp_file.path().to_str().unwrap().to_string();

        let mut buffer = Buffer::default();
        assert!(buffer.init(&path).is_ok());
        assert_eq!(buffer.path, Some(path));
        assert_eq!(buffer.input.lines(), vec!["Hello, world!".to_string()]);
    }

    #[test]
    fn test_custom_text_area() {
        let buffer = Buffer::default();
        let lines = vec!["Line 1".to_string(), "Line 2".to_string()];
        let text_area = buffer.custom_text_area(lines.clone());
        assert_eq!(text_area.lines(), lines);
    }

}