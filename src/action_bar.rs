use std::cell::{Cell, RefCell};
use std::io;
use std::rc::Rc;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Widget;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use crate::new_file_widget::NewFileWidget;
use crate::state::State;

const ACTION: &str = "n: Create File | o: Open File\n\nEsc: Close";

#[derive(Debug, PartialEq, Default)]
pub enum ActionWidget {
    NewFile,
    OpenFile,
    #[default]
    None
}
#[derive(Debug)]
pub struct ActionBar<'a> {
    pub show: Rc<Cell<bool>>,
    pub state: Rc<RefCell<State<'a>>>,
    pub action_widget: ActionWidget,
    pub new_file_widget: NewFileWidget<'a>,
}

impl ActionBar<'_> {
    pub fn new(show: Rc<Cell<bool>>, state: Rc<RefCell<State>>) -> ActionBar<'_> {
        ActionBar {
            show,
            state: state.clone(),
            action_widget: ActionWidget::None,
            new_file_widget: NewFileWidget::new(state),
        }
    }
    pub fn handle_input(&mut self, key: KeyEvent) -> Result<(), io::Error> {
        if self.action_widget != ActionWidget::None {
            self.handle_when_widget_is_active(key)?;
        }
        match key {
            KeyEvent { code: KeyCode::Esc, modifiers: _, .. } => {
                self.action_widget = ActionWidget::None;
                self.show.set(false);
            }
            KeyEvent { code: KeyCode::Char('n'), .. } => {
                self.action_widget = ActionWidget::NewFile;
            },
            KeyEvent { code: KeyCode::Char('o'), .. } => {
                self.action_widget = ActionWidget::OpenFile;
            },
            _ => ()
        }
        Ok(())
    }

    pub fn handle_when_widget_is_active(&mut self, key: KeyEvent) -> Result<(), io::Error> {
        match key.code {
            KeyCode::Enter => {
                match self.action_widget {
                    ActionWidget::NewFile => {
                        self.new_file_widget.create_new_file()?;
                        self.action_widget = ActionWidget::None;
                    },
                    ActionWidget::OpenFile => {}
                    ActionWidget::None => {}
                }
            }
            _ => {
                self.new_file_widget.input.input(key);
            }
        }
        Ok(())
    }
}

impl Widget for &ActionBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
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

        match self.action_widget {
            ActionWidget::NewFile => {
                self.new_file_widget.render(area, buf);
            },
            ActionWidget::OpenFile => {},
            ActionWidget::None => {}
        }
    }
}
