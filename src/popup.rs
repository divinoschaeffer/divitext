use ratatui::layout::{Constraint, Flex, Layout, Rect};

pub fn popup_area(area: Rect, max_x: u16, max_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Max(max_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Max(max_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}