use std::cell::{Cell, RefCell};
use std::rc::Rc;
use crate::app::CurrentScreen;
use crate::buffer::Buffer;

#[derive(Debug)]
pub struct State<'a> {
    pub current_screen: Rc<RefCell<CurrentScreen>>,
    pub current_buffer: Rc<RefCell<usize>>,
    pub buffer_list: Rc<RefCell<Vec<Buffer<'a>>>>,
    pub exit: Rc<Cell<bool>>,
}

impl<'a> State<'a> {
    pub fn new(current_screen: CurrentScreen) -> State<'a> {
        Self {
            current_screen: Rc::new(RefCell::new(current_screen)),
            current_buffer: Rc::new(RefCell::new(0)),
            buffer_list: Rc::new(RefCell::new(Vec::new())),
            exit: Rc::new(Cell::new(false)),
        }
    }

    pub fn push_buffer(&self, buffer: Buffer<'a>) {
        self.buffer_list.borrow_mut().push(buffer);
        *self.current_buffer.borrow_mut() = self.buffer_list.borrow_mut().len() - 1;
    }
}
