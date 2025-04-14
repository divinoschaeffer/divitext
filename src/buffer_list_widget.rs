use crate::action_bar::ActionWidget;
use crate::popup::popup_area;
use crate::state::State;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Widget};
use std::cell::RefCell;
use std::fmt;
use std::fmt::Debug;
use std::io::Error;
use std::option::Option;
use std::rc::Rc;

const SELECT_STYLE: Style = Style::new().bg(Color::White).fg(Color::Black);
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub type ProcessActionFn<'a> = Box<dyn Fn(&mut State<'a>, usize) -> Result<bool, Error> + 'a>;

pub struct BufferListWidget<'a> {
    pub state: Rc<RefCell<State<'a>>>,
    pub items: Vec<BufferItem>,
    pub list_state: ListState,
    pub current: Option<usize>,
    pub title: String,
    pub process_fn: ProcessActionFn<'a>,
}

impl<'a> fmt::Debug for BufferListWidget<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BufferListWidget")
            .field("state", &self.state)
            .field("items", &self.items)
            .field("list_state", &self.list_state)
            .field("current", &self.current)
            .field("title", &self.title)
            .field("process_fn", &"<function>")
            .finish()
    }
}

#[derive(Debug)]
pub struct BufferItem {
    pub file_path: String,
    pub buffer_index: usize,
    pub char: char
}

impl BufferItem {
    pub fn new(file_path: String, buffer_index: usize, char: char) -> BufferItem {
        BufferItem {file_path, buffer_index, char}
    }
}

impl<'a> BufferListWidget<'a> {
    pub fn new(
        state: Rc<RefCell<State<'a>>>,
        title: &str,
        process_fn: ProcessActionFn<'a>
    ) -> BufferListWidget<'a> {
        let items = Vec::new();
        BufferListWidget {
            state,
            items,
            list_state: ListState::default(),
            current: None,
            title: title.to_string(),
            process_fn,
        }
    }

    pub fn for_navigation(state: Rc<RefCell<State<'a>>>) -> BufferListWidget<'a> {
        let navigate_fn: ProcessActionFn<'a> = Box::new(|state, buffer_index| {
            state.current_buffer = buffer_index;
            Ok(true)
        });

        BufferListWidget::new(state, "Buffer List", navigate_fn)
    }

    pub fn for_deletion(state: Rc<RefCell<State<'a>>>) -> BufferListWidget<'a> {
        let delete_fn: ProcessActionFn<'a> = Box::new(|state, buffer_index| {
            if state.buffer_list.len() > 1 {
                if state.current_buffer == buffer_index {
                    if buffer_index > 0 {
                        state.current_buffer = buffer_index - 1;
                    } else if state.buffer_list.len() > 1 {
                        state.current_buffer = 0;
                    }
                } else if state.current_buffer > buffer_index {
                    state.current_buffer -= 1;
                }

                state.buffer_list.remove(buffer_index);

                return Ok(true);
            }
            Ok(false)
        });

        BufferListWidget::new(state, "Delete Buffer", delete_fn)
    }

    pub fn refresh_list(&mut self) {
        let state = self.state.borrow();
        self.items.clear();

        let mut chars = ALPHABET.chars();

        for (index, buffer) in state.buffer_list.iter().enumerate() {
            if let Some(path) = buffer.path.clone() {
                let buffer_char = chars.next().unwrap_or('-');
                self.items.push(BufferItem::new(path, index, buffer_char));
            }
        }
    }

    pub fn render_content(&self, area: Rect, buf: &mut Buffer) {
        let area = popup_area(area, 70, 20);

        let block = Block::default()
            .bold()
            .title(self.title.clone())
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

impl<'a> ActionWidget for BufferListWidget<'a> {
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
        if let Some(current_idx) = self.current {
            if current_idx < self.items.len() {
                let buffer_index = self.items[current_idx].buffer_index;
                let mut state = self.state.borrow_mut();

                return (self.process_fn)(&mut *state, buffer_index);
            }
        }
        Ok(false)
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
        let process_fn: ProcessActionFn = Box::new(|_, _| Ok(true));
        let widget = BufferListWidget::new(state, "Test Title", process_fn);

        assert!(widget.items.is_empty());
        assert_eq!(widget.current, None);
        assert_eq!(widget.title, "Test Title");
    }

    #[test]
    fn test_factory_methods() {
        let state = create_test_state();

        let navigation_widget = BufferListWidget::for_navigation(state.clone());
        assert_eq!(navigation_widget.title, "Buffer List");

        let deletion_widget = BufferListWidget::for_deletion(state.clone());
        assert_eq!(deletion_widget.title, "Delete Buffer");
    }

    #[test]
    fn test_refresh_list() {
        let state = create_test_state();
        let process_fn: ProcessActionFn = Box::new(|_, _| Ok(true));
        let mut widget = BufferListWidget::new(state, "Test", process_fn);

        widget.refresh_list();

        assert_eq!(widget.items.len(), 3);
        assert_eq!(widget.items[0].file_path, "file1.txt");
        assert_eq!(widget.items[1].file_path, "file2.txt");
        assert_eq!(widget.items[2].file_path, "file3.txt");

        assert_eq!(widget.items[0].buffer_index, 0);
        assert_eq!(widget.items[1].buffer_index, 1);
        assert_eq!(widget.items[2].buffer_index, 2);
    }

    #[test]
    fn test_select_navigation() {
        let state = create_test_state();
        let process_fn: ProcessActionFn = Box::new(|_, _| Ok(true));
        let mut widget = BufferListWidget::new(state, "Test", process_fn);
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
    fn test_process_action_navigation() {
        let state = create_test_state();
        let state_clone = state.clone();
        let mut widget = BufferListWidget::for_navigation(state);
        widget.refresh_list();

        widget.select_by_letter('b');
        assert_eq!(widget.current, Some(1));

        let result = widget.process_action().unwrap();
        assert!(result);

        {
            let state_ref = state_clone.borrow();
            assert_eq!(state_ref.current_buffer, 1);
        }
    }

    #[test]
    fn test_process_action_deletion() {
        let state = create_test_state();
        let state_clone = state.clone();
        let mut widget = BufferListWidget::for_deletion(state);
        widget.refresh_list();

        widget.select_by_letter('b');
        assert_eq!(widget.current, Some(1));

        // Verify there is 3 buffer
        {
            let state_ref = state_clone.borrow();
            assert_eq!(state_ref.buffer_list.len(), 3);
        }

        // remove buffer
        let result = widget.process_action().unwrap();
        assert!(result);

        // Verify there is 2 buffer after delete
        {
            let state_ref = state_clone.borrow();
            assert_eq!(state_ref.buffer_list.len(), 2);
            assert_eq!(state_ref.buffer_list[0].path, Some("file1.txt".to_string()));
            assert_eq!(state_ref.buffer_list[1].path, Some("file3.txt".to_string()));
        }
    }

    #[test]
    fn test_delete_current_buffer() {
        let state = create_test_state();
        {
            let mut state_ref = state.borrow_mut();
            state_ref.current_buffer = 1;
        }

        let mut widget = BufferListWidget::for_deletion(state.clone());
        widget.refresh_list();

        widget.select_by_letter('b');
        let result = widget.process_action().unwrap();
        assert!(result);

        {
            let state_ref = state.borrow();
            assert_eq!(state_ref.current_buffer, 0);
            assert_eq!(state_ref.buffer_list.len(), 2);
        }
    }

    #[test]
    fn test_prevent_delete_last_buffer() {
        let state = Rc::new(RefCell::new(State::default()));
        {
            let mut state_ref = state.borrow_mut();
            let buffer = Buffer::new(TextArea::default(), Some("only_file.txt".to_string()));
            state_ref.push_buffer(buffer);
        }

        let mut widget = BufferListWidget::for_deletion(state.clone());
        widget.refresh_list();

        widget.select_first();
        let result = widget.process_action().unwrap();

        assert!(!result);

        {
            let state_ref = state.borrow();
            assert_eq!(state_ref.buffer_list.len(), 1);
        }
    }

    #[test]
    fn test_custom_process_function() {
        let state = create_test_state();
        let state_clone = state.clone();

        let mark_fn: ProcessActionFn = Box::new(|state, buffer_index| {
            state.current_buffer = buffer_index;
            Ok(true)
        });

        let mut widget = BufferListWidget::new(state, "Mark Buffer", mark_fn);
        widget.refresh_list();

        widget.select_by_letter('c');
        let result = widget.process_action().unwrap();

        assert!(result);
        {
            let state_ref = state_clone.borrow();
            assert_eq!(state_ref.current_buffer, 2);
        }
    }
}