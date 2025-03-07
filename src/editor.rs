use crate::buffer::Buffer;
use crate::state::State;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::ops::Deref;
use std::rc::Rc;
use tui_textarea::TextArea;

#[derive(Debug)]
pub struct Editor<'a> {
    pub state: Rc<RefCell<State<'a>>>,
}

impl<'a> Editor<'a> {
    pub fn new(state: Rc<RefCell<State<'a>>>) -> Editor<'a> {
        Self {
            state,
        }
    }

    pub fn init(&mut self, file_path: Option<String>) ->Result<(), io::Error> {
        let state = self.state.borrow_mut();
        let mut buffer_list = state.buffer_list.borrow_mut();
        let mut current_buffer = state.current_buffer.borrow_mut();

        if let Some(filename) = file_path.as_ref() {
            let file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .read(true)
                .write(true)
                .open(filename)?;

            let input: TextArea = io::BufReader::new(file).lines().collect::<io::Result<_>>()?;
            let buffer = Buffer::new(input, Option::from(filename.deref().to_owned()));

            buffer_list.push(buffer);
            *current_buffer = buffer_list.len() - 1;
        } else {
            let buffer = Buffer::new(TextArea::default(), None);
            buffer_list.push(buffer);
        }
        Ok(())
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> Result<(), io::Error> {
        let state = self.state.borrow_mut();
        let mut buffer_list = state.buffer_list.borrow_mut();
        let mut current_buffer = state.current_buffer.borrow_mut();
        buffer_list[*current_buffer].input.input(key);
        Ok(())
    }
}

impl<'a> Widget for &Editor<'a>{
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer)
    {
        let state = self.state.borrow_mut();
        let mut buffer_list = state.buffer_list.borrow_mut();
        let mut current_buffer = state.current_buffer.borrow_mut();
        if !buffer_list.is_empty() {
            buffer_list[*current_buffer].input.render(area, buf);
        }
    }
}
