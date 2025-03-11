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

impl<'a> Default for State<'a> {
    fn default() -> State<'a> {
        Self {
            current_screen: Rc::new(RefCell::new(CurrentScreen::Home)),
            current_buffer: Rc::new(RefCell::new(0)),
            buffer_list: Rc::new(RefCell::new(Vec::new())),
            exit: Rc::new(Cell::new(false)),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::CurrentScreen;
    use crate::buffer::Buffer;
    use tui_textarea::TextArea;

    #[test]
    fn test_state_default() {
        let state = State::default();
        assert_eq!(*state.current_screen.borrow(), CurrentScreen::Home);
        assert_eq!(*state.current_buffer.borrow(), 0);
        assert!(state.buffer_list.borrow().is_empty());
        assert_eq!(state.exit.get(), false);
    }

    #[test]
    fn test_state_new() {
        let state = State::new(CurrentScreen::Editor);
        assert_eq!(*state.current_screen.borrow(), CurrentScreen::Editor);
        assert_eq!(*state.current_buffer.borrow(), 0);
        assert!(state.buffer_list.borrow().is_empty());
        assert_eq!(state.exit.get(), false);
    }

    #[test]
    fn test_push_buffer() {
        let state = State::default();
        let buffer = Buffer::new(TextArea::default(), Some("test.txt".to_string()));

        state.push_buffer(buffer.clone());

        let buffer_list = state.buffer_list.borrow();
        assert_eq!(buffer_list.len(), 1);
        assert_eq!(*state.current_buffer.borrow(), 0);
        assert_eq!(buffer_list[0].filename, Some("test.txt".to_string()));
    }

    #[test]
    fn test_push_multiple_buffers() {
        let state = State::default();

        let buffer1 = Buffer::new(TextArea::default(), Some("file1.txt".to_string()));
        let buffer2 = Buffer::new(TextArea::default(), Some("file2.txt".to_string()));

        state.push_buffer(buffer1);
        assert_eq!(*state.current_buffer.borrow(), 0);

        state.push_buffer(buffer2);
        assert_eq!(*state.current_buffer.borrow(), 1);
        assert_eq!(state.buffer_list.borrow().len(), 2);
    }

    #[test]
    fn test_update_screen() {
        let state = State::default();
        *state.current_screen.borrow_mut() = CurrentScreen::Editor;
        assert_eq!(*state.current_screen.borrow(), CurrentScreen::Editor);
    }

    #[test]
    fn test_exit_flag() {
        let state = State::default();
        assert_eq!(state.exit.get(), false);

        state.exit.set(true);
        assert_eq!(state.exit.get(), true);
    }
}
