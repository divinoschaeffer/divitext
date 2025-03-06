use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use std::cell::Cell;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead};
use std::ops::Deref;
use std::rc::Rc;
use crossterm::event::KeyEvent;
use tui_textarea::TextArea;
use crate::buffer::Buffer;

#[derive(Debug)]
pub struct Editor<'a> {
    pub exit: Rc<Cell<bool>>,
    pub current_buffer: usize,
    pub buffer_list: Vec<Buffer<'a>>,
}

impl<'a> Editor<'a> {
    pub fn new(exit: Rc<Cell<bool>>) -> Editor<'a> {
        Self {
            exit,
            current_buffer: 0,
            buffer_list: vec![],
        }
    }

    pub fn init(&mut self, file_path: Option<String>) ->Result<(), io::Error> {
        if let Some(filename) = file_path.as_ref() {
            let file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .read(true)
                .write(true)
                .open(filename)?;

            let input: TextArea = io::BufReader::new(file).lines().collect::<io::Result<_>>()?;
            let buffer = Buffer::new(input, Option::from(filename.deref().to_owned()));

            self.buffer_list.push(buffer);
        } else {
            let buffer = Buffer::new(TextArea::default(), None);
            self.buffer_list.push(buffer);
        }
        Ok(())
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> Result<(), io::Error> {
        self.buffer_list[self.current_buffer].input.input(key);
        Ok(())
    }
}

impl<'a> Widget for &Editor<'a>{
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer)
    {
        if !self.buffer_list.is_empty() {
            self.buffer_list[self.current_buffer].input.render(area, buf);
        }
    }
}
