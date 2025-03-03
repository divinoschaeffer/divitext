use std::cell::Cell;
use std::io;
use std::rc::Rc;
use crossterm::event;
use crossterm::event::Event;
use ratatui::{DefaultTerminal, Frame};
use crate::editor::Editor;

#[derive(Debug)]
pub struct App {
    pub current_screen: CurrentScreen,
    pub editor: Editor,
    pub exit: Rc<Cell<bool>>,
}

impl Default for App {
    fn default() -> Self {
        let exit = Rc::new(Cell::new(false));
        Self {
            current_screen: CurrentScreen::Editor,
            editor: Editor::new(exit.clone()),
            exit,
        }
    }
}

#[derive(Debug, Default)]
pub enum CurrentScreen {
    Home,
    #[default]
    Editor,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, file: Option<String>) -> io::Result<()> {
        self.editor.init(file)?;

        while !self.exit.get() {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget("hello world", frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match self.current_screen {
                CurrentScreen::Home => (),
                CurrentScreen::Editor => {
                    self.editor.handle_key_events(key)?;
                }
            }
        }
        Ok(())
    }
}