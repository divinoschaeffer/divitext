use std::cell::{Cell, RefCell};
use std::io;
use std::rc::Rc;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Widget;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use crate::buffer_list_widget::BufferListWidget;
use crate::new_file_widget::NewFileWidget;
use crate::open_file_widget::OpenFileWidget;
use crate::state::State;

pub trait ActionWidget: std::fmt::Debug {
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn handle_input(&mut self, key: KeyEvent) -> Result<(), io::Error>;
    fn has_error(&self) -> bool;
    fn process_action(&mut self) -> Result<bool, io::Error>;
    fn init_action(&mut self);
    fn reset(&mut self);
}

#[derive(Debug, PartialEq, Default)]
pub enum ActionType {
    NewFile,
    OpenFile,
    ChangeBuffer,
    DeleteBuffer,
    #[default]
    None
}

const ACTION: &str = "n: Create File | o: Open File\n b: Change Buffer | d: Close Buffer\n\nEsc: Close";

#[derive(Debug)]
pub struct ActionBar<'a> {
    pub show: Rc<Cell<bool>>,
    pub state: Rc<RefCell<State<'a>>>,
    pub current_action: ActionType,
    pub widgets: Vec<Box<dyn ActionWidget + 'a>>,
}

impl<'a> ActionBar<'a> {
    pub fn new(show: Rc<Cell<bool>>, state: Rc<RefCell<State<'a>>>) -> ActionBar<'a> {
        let new_file_widget = Box::new(NewFileWidget::new(state.clone()));
        let open_file_widget = Box::new(OpenFileWidget::new(state.clone()));
        let buffer_navigation_widget = Box::new(BufferListWidget::for_navigation(state.clone()));
        let buffer_deletion_widget = Box::new(BufferListWidget::for_deletion(state.clone()));

        ActionBar {
            show,
            state,
            current_action: ActionType::None,
            widgets: vec![
                new_file_widget,
                open_file_widget,
                buffer_navigation_widget,
                buffer_deletion_widget,
            ],
        }
    }

    pub fn handle_input(&mut self, key: KeyEvent) -> Result<(), io::Error> {
        if self.current_action != ActionType::None {
            self.handle_active_widget_input(key)?;
            return Ok(());
        } else {
            for widget in self.widgets.iter_mut() {
                widget.init_action();
            }
        }

        match key.code {
            KeyCode::Esc => {
                self.current_action = ActionType::None;
                self.reset();
                self.show.set(false);
            }
            KeyCode::Char('n') => {
                self.current_action = ActionType::NewFile;
            },
            KeyCode::Char('o') => {
                self.current_action = ActionType::OpenFile;
            },
            KeyCode::Char('b') => {
                self.current_action = ActionType::ChangeBuffer;
            },
            KeyCode::Char('d') => {
                self.current_action = ActionType::DeleteBuffer;
            }
            _ => ()
        }
        Ok(())
    }

    fn handle_active_widget_input(&mut self, key: KeyEvent) -> Result<(), io::Error> {
        let widget_index = match self.current_action {
            ActionType::NewFile => 0,
            ActionType::OpenFile => 1,
            ActionType::ChangeBuffer => 2,
            ActionType::DeleteBuffer => 3,
            ActionType::None => return Ok(()),
        };

        let has_error = self.widgets[widget_index].has_error();

        if has_error && (key.code == KeyCode::Enter || key.code == KeyCode::Char(' ')) {
            self.widgets[widget_index].handle_input(key)?;
            return Ok(());
        }

        match key.code {
            KeyCode::Enter => {
                let action_completed = self.widgets[widget_index].process_action()?;
                if action_completed {
                    self.show.set(false);
                    self.current_action = ActionType::None;
                }
            }
            _ => {
                self.widgets[widget_index].handle_input(key)?;
            }
        }
        Ok(())
    }

    fn get_active_widget(&self) -> Option<&Box<dyn ActionWidget + 'a>> {
        match self.current_action {
            ActionType::NewFile => Some(&self.widgets[0]),
            ActionType::OpenFile => Some(&self.widgets[1]),
            ActionType::ChangeBuffer => Some(&self.widgets[2]),
            ActionType::DeleteBuffer => Some(&self.widgets[3]),
            ActionType::None => None,
        }
    }
    
    pub fn reset(&mut self) {
        for i in 0..self.widgets.len() {
            self.widgets[i].reset();
        }
    }
}

impl Widget for &ActionBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Max(6),
            ])
            .split(area);

        let block = Block::default().borders(Borders::ALL);

        let action_message = Paragraph::new(ACTION)
            .block(block)
            .centered()
            .bold();

        Clear.render(layout[1], buf);
        action_message.render(layout[1], buf);

        if let Some(widget) = self.get_active_widget() {
            widget.render(area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;
    use std::cell::{RefCell, Cell};
    use std::rc::Rc;
    use crossterm::event::{KeyCode, KeyModifiers};

    #[test]
    fn test_action_bar_new_defaults() {
        let state = Rc::new(RefCell::new(State::default()));
        let show = Rc::new(Cell::new(false));
        let action_bar = ActionBar::new(Rc::clone(&show), Rc::clone(&state));

        assert_eq!(action_bar.current_action, ActionType::None);
        assert_eq!(show.get(), false);
    }

    #[test]
    fn test_handle_input_shortcuts() {
        let state = Rc::new(RefCell::new(State::default()));
        let show = Rc::new(Cell::new(false));
        let mut action_bar = ActionBar::new(Rc::clone(&show), Rc::clone(&state));

        // Test New File
        let key = KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE);
        action_bar.handle_input(key).unwrap();
        assert_eq!(action_bar.current_action, ActionType::NewFile);

        action_bar.current_action = ActionType::None;

        // Test Open File
        let key = KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE);
        action_bar.handle_input(key).unwrap();
        assert_eq!(action_bar.current_action, ActionType::OpenFile);

        action_bar.current_action = ActionType::None;

        // Test Change Buffer
        let key = KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE);
        action_bar.handle_input(key).unwrap();
        assert_eq!(action_bar.current_action, ActionType::ChangeBuffer);

        action_bar.current_action = ActionType::None;

        // Test Escape
        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        action_bar.handle_input(key).unwrap();
        assert_eq!(action_bar.current_action, ActionType::None);
        assert_eq!(show.get(), false);
    }
}