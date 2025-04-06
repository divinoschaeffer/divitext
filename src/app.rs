use crate::editor::Editor;
use crate::home::Home;
use crate::state::State;
use crossterm::event::{DisableMouseCapture, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, execute};
use ratatui::{DefaultTerminal, Frame};
use std::cell::{Cell, RefCell};
use std::io;
use std::rc::Rc;
use crate::action_bar::{ActionBar, ActionWidget};

#[derive(Debug)]
pub struct App<'a> {
    pub home: Home<'a>,
    pub editor: Editor<'a>,
    pub action_bar: ActionBar<'a>,
    pub state: Rc<RefCell<State<'a>>>,
    pub show_action_bar: Rc<Cell<bool>>,
}

impl Default for App<'_> {
    fn default() -> Self {
        let state = Rc::new(RefCell::new(State::new(CurrentScreen::default())));
        let show_action_bar = Rc::new(Cell::new(false));
        Self {
            home: Home::new(state.clone()),
            editor: Editor::new(state.clone()),
            action_bar: ActionBar::new(show_action_bar.clone(), state.clone()),
            state,
            show_action_bar: show_action_bar.clone(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum CurrentScreen {
    #[default]
    Home,
    Editor,
}

impl App<'_> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, file: Option<&String>) -> io::Result<()> {
        if file.is_some() {
            self.state.borrow_mut().current_screen = CurrentScreen::Editor;
        }

        self.editor.init(file)?;

        while !self.state.borrow().exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    pub fn init(terminal: &mut DefaultTerminal) -> Result<(), io::Error> {
        enable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            EnterAlternateScreen
        )?;
        Ok(())
    }

    pub fn drop(terminal: &mut DefaultTerminal) -> Result<(), io::Error> {
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        disable_raw_mode()?;
        terminal.show_cursor()?;
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        match &self.state.borrow().current_screen {
            CurrentScreen::Editor => frame.render_widget(&self.editor, frame.area()),
            CurrentScreen::Home => frame.render_widget(&self.home, frame.area())
        }

        if self.show_action_bar.get() {
            frame.render_widget(&self.action_bar, frame.area());
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {

            if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::CONTROL {
                self.state.borrow_mut().exit = true;
            }

            if key.code == KeyCode::Char(' ') && key.modifiers == KeyModifiers::CONTROL {
                self.action_bar.action_widget = ActionWidget::None;
                self.show_action_bar.set(!self.show_action_bar.get());
            }

            if self.show_action_bar.get() {
                if key.code == KeyCode::Esc {
                    self.show_action_bar.set(false);
                    self.action_bar.action_widget = ActionWidget::None;
                } else {
                    self.action_bar.handle_input(key)?;
                }
            } else {
                let current_screen = self.state.borrow().current_screen.clone();

                match current_screen {
                    CurrentScreen::Home => self.home.handle_input(key)?,
                    CurrentScreen::Editor => self.editor.handle_input(key)?,
                }
            }
        }
        Ok(())
    }
}
