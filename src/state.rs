use std::cell::{Cell, RefCell};
use std::rc::Rc;
use crate::app::CurrentScreen;

#[derive(Debug)]
pub struct State {
    pub current_screen: Rc<RefCell<CurrentScreen>>,
    pub exit: Rc<Cell<bool>>,
}

impl State {
    pub fn new(current_screen: CurrentScreen) -> Self {
        Self {
            current_screen: Rc::new(RefCell::new(current_screen)),
            exit: Rc::new(Cell::new(false)),
        }
    }
}
