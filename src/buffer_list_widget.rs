use crate::action_bar::ActionWidget;
use crate::popup::popup_area;
use crate::state::State;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Widget;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders, Clear, HighlightSpacing, List, ListItem, ListState};
use std::cell::RefCell;
use std::fmt::Debug;
use std::io::Error;
use std::option::Option;
use std::rc::Rc;

const SELECT_STYLE: Style = Style::new().bg(Color::White).fg(Color::Black);
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug)]
pub struct BufferListWidget<'a> {
    pub state: Rc<RefCell<State<'a>>>,
    pub items: Vec<BufferItem>,
    pub list_state: ListState,
    pub current: Option<usize>,
}

#[derive(Debug)]
pub struct BufferItem {
    file_path: String,
    char: char
}

impl BufferItem {
    pub fn new(file_path: String, char: char) -> BufferItem {
        BufferItem {file_path, char}
    }
}

impl<'a> BufferListWidget<'a> {
    pub fn new(state: Rc<RefCell<State<'a>>>) -> BufferListWidget<'a> {
        let items = Vec::new();
        BufferListWidget {
            state,
            items,
            list_state: ListState::default(),
            current: None,
        }
    }

    pub fn refresh_list(&mut self) {
        let state = self.state.borrow();
        self.items.clear();

        let mut chars = ALPHABET.chars();

        for buffer in state.buffer_list.iter() {
            if let Some(path) = buffer.path.clone() {
                let buffer_char = chars.next().unwrap_or('-');
                self.items.push(BufferItem::new(path, buffer_char));
            }
        }
    }

    pub fn render_content(&self, area: Rect, buf: &mut Buffer) {
        let area = popup_area(area, 70, 20);

        let block = Block::default()
            .bold()
            .title("Buffer List")
            .borders(Borders::ALL);

        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                if self.current.is_some() && self.current.unwrap() == i {
                    ListItem::from(item).black().on_white()
                } else {
                    ListItem::from(item)
                }
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECT_STYLE)
            .highlight_symbol(">> ")
            .highlight_spacing(HighlightSpacing::Always);

        Clear.render(area, buf);
        list.render(area, buf);
    }

    fn select_none(&mut self) {
        self.current = None;
        self.list_state.select(None);
    }

    fn select_next(&mut self) {
        self.current = if let Some(i) = self.current {
            if i + 1 < self.items.len() {
                Some(i + 1)
            } else {
                Some(i)
            }
        } else {
            Some(0)
        };
        self.list_state.select(self.current);
    }

    fn select_previous(&mut self) {
        self.current = if let Some(i) = self.current {
            if i > 0 {
                Some(i - 1)
            } else {
                Some(0)
            }
        } else { Some(0) };
        self.list_state.select(self.current);
    }

    fn select_first(&mut self) {
        self.current = Some(0);
        self.list_state.select(self.current);
    }

    fn select_last(&mut self) {
        if !self.items.is_empty() {
            self.current = Some(self.items.len() - 1);
            self.list_state.select(self.current);
        }
    }

    fn select_by_letter(&mut self, letter: char) {
        if let Some(index) = self.items.iter().position(|item| item.char == letter) {
            self.current = Some(index);
            self.list_state.select(self.current);
        }
    }

    pub fn handle_event(&mut self, key: KeyEvent) -> Result<(), Error> {
        match key.code {
            KeyCode::Left => self.select_previous(),
            KeyCode::Right => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Down => self.select_next(),
            KeyCode::Home => self.select_first(),
            KeyCode::End => self.select_last(),
            KeyCode::Char(c) if c.is_ascii_alphabetic() => self.select_by_letter(c.to_ascii_lowercase()),
            _ => ()
        }
        Ok(())
    }
}

impl From<&BufferItem> for ListItem<'_> {
    fn from(value: &BufferItem) -> Self {
        let line = format!("{} {}", value.char, value.file_path);
        ListItem::new(line)
    }
}

impl ActionWidget for BufferListWidget<'_> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        self.render_content(area, buf);
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<(), Error> {
        self.handle_event(key)
    }

    fn has_error(&self) -> bool {
        false
    }

    fn process_action(&mut self) -> Result<bool, Error> {
        if self.current.is_some() {
            let mut state = self.state.borrow_mut();
            state.current_buffer = state.find_buffer_index(
                &*self.items[self.current.unwrap()].file_path
            ).unwrap();
        }
        Ok(true)
    }

    fn init_action(&mut self) {
        self.refresh_list();
    }

    fn reset(&mut self) {
        self.select_none();
    }
}

impl Widget for &BufferListWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        self.render_content(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Buffer;
    use crate::state::State;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
    use std::cell::RefCell;
    use std::rc::Rc;
    use tui_textarea::TextArea;

    fn create_test_state<'a>() -> Rc<RefCell<State<'a>>> {
        let mut state = State::default();

        let buffer1 = Buffer::new(TextArea::default(), Some("file1.txt".to_string()));
        let buffer2 = Buffer::new(TextArea::default(), Some("file2.txt".to_string()));
        let buffer3 = Buffer::new(TextArea::default(), Some("file3.txt".to_string()));

        state.push_buffer(buffer1);
        state.push_buffer(buffer2);
        state.push_buffer(buffer3);

        Rc::new(RefCell::new(state))
    }

    fn create_key_event(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn test_buffer_list_widget_new() {
        let state = create_test_state();
        let widget = BufferListWidget::new(state);

        assert!(widget.items.is_empty());
        assert_eq!(widget.current, None);
    }

    #[test]
    fn test_buffer_item_new() {
        let item = BufferItem::new("test.txt".to_string(), 'a');

        assert_eq!(item.file_path, "test.txt");
        assert_eq!(item.char, 'a');
    }

    #[test]
    fn test_refresh_list() {
        let state = create_test_state();
        let mut widget = BufferListWidget::new(state);

        widget.refresh_list();

        assert_eq!(widget.items.len(), 3);
        assert_eq!(widget.items[0].file_path, "file1.txt");
        assert_eq!(widget.items[1].file_path, "file2.txt");
        assert_eq!(widget.items[2].file_path, "file3.txt");

        assert_eq!(widget.items[0].char, 'a');
        assert_eq!(widget.items[1].char, 'b');
        assert_eq!(widget.items[2].char, 'c');
    }

    #[test]
    fn test_select_navigation() {
        let state = create_test_state();
        let mut widget = BufferListWidget::new(state);
        widget.refresh_list();

        // Initially no selection
        assert_eq!(widget.current, None);

        // Select first
        widget.select_first();
        assert_eq!(widget.current, Some(0));

        // Select next
        widget.select_next();
        assert_eq!(widget.current, Some(1));

        // Select last
        widget.select_last();
        assert_eq!(widget.current, Some(2));

        // Try to go beyond last (should stay at last)
        widget.select_next();
        assert_eq!(widget.current, Some(2));

        // Select previous
        widget.select_previous();
        assert_eq!(widget.current, Some(1));

        // Reset selection
        widget.select_none();
        assert_eq!(widget.current, None);
    }

    #[test]
    fn test_select_by_letter() {
        let state = create_test_state();
        let mut widget = BufferListWidget::new(state);
        widget.refresh_list();

        widget.select_by_letter('b');
        assert_eq!(widget.current, Some(1));

        widget.select_by_letter('c');
        assert_eq!(widget.current, Some(2));

        widget.select_by_letter('a');
        assert_eq!(widget.current, Some(0));

        // Non-existing letter should not change selection
        widget.select_by_letter('z');
        assert_eq!(widget.current, Some(0));
    }

    #[test]
    fn test_handle_event() {
        let state = create_test_state();
        let mut widget = BufferListWidget::new(state);
        widget.refresh_list();

        // Test arrow keys
        widget.handle_event(create_key_event(KeyCode::Down)).unwrap();
        assert_eq!(widget.current, Some(0));

        widget.handle_event(create_key_event(KeyCode::Down)).unwrap();
        assert_eq!(widget.current, Some(1));

        widget.handle_event(create_key_event(KeyCode::Up)).unwrap();
        assert_eq!(widget.current, Some(0));

        // Test Home/End
        widget.handle_event(create_key_event(KeyCode::End)).unwrap();
        assert_eq!(widget.current, Some(2));

        widget.handle_event(create_key_event(KeyCode::Home)).unwrap();
        assert_eq!(widget.current, Some(0));

        // Test letter selection
        widget.handle_event(create_key_event(KeyCode::Char('b'))).unwrap();
        assert_eq!(widget.current, Some(1));
    }

    #[test]
    fn test_process_action() {
        let state = create_test_state();
        let state_clone = state.clone();
        let mut widget = BufferListWidget::new(state);
        widget.refresh_list();

        widget.select_by_letter('b');
        let result = widget.process_action().unwrap();

        assert!(result);

        let state_ref = state_clone.borrow();
        assert_eq!(state_ref.current_buffer, 1);
    }

    #[test]
    fn test_init_action_and_reset() {
        let state = create_test_state();
        let mut widget = BufferListWidget::new(state);

        assert!(widget.items.is_empty());

        widget.init_action();
        assert_eq!(widget.items.len(), 3);

        widget.select_first();
        assert_eq!(widget.current, Some(0));

        widget.reset();
        assert_eq!(widget.current, None);
    }

    #[test]
    fn test_has_error() {
        let state = create_test_state();
        let widget = BufferListWidget::new(state);

        assert_eq!(widget.has_error(), false);
    }

    #[test]
    fn test_list_item_conversion() {
        let item = BufferItem::new("test.txt".to_string(), 'x');
        let list_item = ListItem::from(&item);

        // This test is limited as we can't easily inspect the content of the ListItem
        // But it ensures the conversion works without panicking
    }
}
