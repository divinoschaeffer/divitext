use ratatui::prelude::Style;
use ratatui::widgets::{Block, Borders};
use tui_textarea::TextArea;

pub fn text_area_popup(title: &str) -> TextArea {
    let mut text_area = TextArea::default();
    text_area.set_cursor_line_style(Style::default());
    text_area.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
    );
    text_area
}