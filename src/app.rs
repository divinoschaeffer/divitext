use crate::editor::Editor;
use crossterm::{event, execute};
use crossterm::event::{DisableMouseCapture, Event, KeyCode, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use std::cell::Cell;
use std::io;
use std::io::stdout;
use std::rc::Rc;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

#[derive(Debug)]
pub struct App<'a> {
    pub current_screen: CurrentScreen,
    pub editor: Editor<'a>,
    pub exit: Rc<Cell<bool>>,
}

impl Default for App<'_> {
    fn default() -> Self {
        let exit = Rc::new(Cell::new(false));
        Self {
            current_screen: CurrentScreen::Editor,
            editor: Editor::new(exit.clone()),
            exit,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum CurrentScreen {
    Home,
    #[default]
    Editor,
}

impl App<'_> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, file: Option<String>) -> io::Result<()> {
        self.editor.init(file)?;

        while !self.exit.get() {
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
        frame.render_widget(&self.editor, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::CONTROL {
                self.exit.set(true)
            }
            match self.current_screen {
                CurrentScreen::Home => (),
                CurrentScreen::Editor => {
                    self.editor.handle_input(key)?;
                }
            }
        }
        Ok(())
    }
}