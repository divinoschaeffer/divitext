use crate::editor::Editor;
use crate::home::Home;
use crate::state::State;
use crossterm::event::{DisableMouseCapture, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, execute};
use ratatui::{DefaultTerminal, Frame};
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

#[derive(Debug)]
pub struct App<'a> {
    pub home: Home<'a>,
    pub editor: Editor<'a>,
    pub state: Rc<RefCell<State<'a>>>,
}

impl Default for App<'_> {
    fn default() -> Self {
        let state = Rc::new(RefCell::new(State::new(CurrentScreen::default())));
        Self {
            home: Home::new(state.clone()),
            editor: Editor::new(state.clone()),
            state,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum CurrentScreen {
    #[default]
    Home,
    Editor,
}

impl App<'_> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, file: Option<String>) -> io::Result<()> {
        self.editor.init(file)?;

        while !self.state.borrow().exit.get() {
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
        match &self.state.borrow().current_screen.borrow().clone() {
            CurrentScreen::Editor => frame.render_widget(&self.editor, frame.area()),
            CurrentScreen::Home => frame.render_widget(&self.home, frame.area())
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::CONTROL {
                self.state.borrow_mut().exit.set(true);
            }
            match self.state.borrow().current_screen.borrow().clone() {
                CurrentScreen::Home => self.home.handle_input(key)?,
                CurrentScreen::Editor => self.editor.handle_input(key)?,
            }
        }
        Ok(())
    }
}