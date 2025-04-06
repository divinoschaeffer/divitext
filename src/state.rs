use std::cell::{Cell, RefCell};
use std::rc::Rc;
use crate::app::CurrentScreen;
use crate::buffer::Buffer;

#[derive(Debug)]
pub struct State<'a> {
    pub current_screen: CurrentScreen,
    pub current_buffer: usize,
    pub buffer_list: Vec<Buffer<'a>>,
    pub exit: bool,
}

impl<'a> Default for State<'a> {
    fn default() -> State<'a> {
        Self {
            current_screen: CurrentScreen::Home,
            current_buffer: 0,
            buffer_list: Vec::new(),
            exit: false,
        }
    }
}

impl<'a> State<'a> {
    pub fn new(current_screen: CurrentScreen) -> State<'a> {
        Self {
            current_screen,
            current_buffer: 0,
            buffer_list: Vec::new(),
            exit: false,
        }
    }

    pub fn push_buffer(&mut self, buffer: Buffer<'a>) {
        self.buffer_list.push(buffer);
        self.current_buffer = self.buffer_list.len() - 1;
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
        assert_eq!(*state.current_screen, CurrentScreen::Home);
        assert_eq!(*state.current_buffer, 0);
        assert!(state.buffer_list.borrow().is_empty());
        assert_eq!(state.exit, false);
    }

    #[test]
    fn test_state_new() {
        let state = State::new(CurrentScreen::Editor);
        assert_eq!(*state.current_screen, CurrentScreen::Editor);
        assert_eq!(*state.current_buffer, 0);
        assert!(state.buffer_list.borrow().is_empty());
        assert_eq!(state.exit, false);
    }

    #[test]
    fn test_push_buffer() {
        let mut state = State::default();
        let buffer = Buffer::new(TextArea::default(), Some("test.txt".to_string()));

        state.push_buffer(buffer.clone());

        let buffer_list = state.buffer_list.borrow();
        assert_eq!(buffer_list.len(), 1);
        assert_eq!(*state.current_buffer, 0);
        assert_eq!(buffer_list[0].filename, Some("test.txt".to_string()));
    }

    #[test]
    fn test_push_multiple_buffers() {
        let mut state = State::default();

        let buffer1 = Buffer::new(TextArea::default(), Some("file1.txt".to_string()));
        let buffer2 = Buffer::new(TextArea::default(), Some("file2.txt".to_string()));

        state.push_buffer(buffer1);
        assert_eq!(*state.current_buffer, 0);

        state.push_buffer(buffer2);
        assert_eq!(*state.current_buffer, 1);
        assert_eq!(state.buffer_list.borrow().len(), 2);
    }

    #[test]
    fn test_update_screen() {
        let state = State::default();
        *state.current_screen = CurrentScreen::Editor;
        assert_eq!(*state.current_screen, CurrentScreen::Editor);
    }

    #[test]
    fn test_exit_flag() {
        let mut state = State::default();
        assert_eq!(state.exit, false);

        state.exit = true;
        assert_eq!(state.exit, true);
    }
}
