use ratatui::buffer::Buffer;
use ratatui::layout::Flex;
use ratatui::prelude::*;

const name: &str = "       ██    ██  ███████  ██       ██       ███████
       ██    ██  ██       ██       ██      ██     ██
       ████████  ██████   ██       ██      ██     ██
       ██    ██  ██       ██       ██      ██     ██
       ██    ██  ███████  ███████  ███████  ███████

██     ███    ██   ███████   ███████    ██       ████████
██     ███    ██  ██     ██  ██    ██   ██       ██     ██
██     ███    ██  ██     ██  ███████    ██       ██      ██
 ██   ██ ██  ██   ██     ██  ██    ██   ██       ██     ██
   ████   ████     ███████   ██    ██   ███████  ████████";

#[derive(Debug, Default)]
pub struct Home {

}

impl Home {

}

impl Widget for &Home {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ])
            .flex(Flex::Center)
            .split(area);

        let center_area_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .flex(Flex::Center)
            .split(layout[1]);

        let inner_center_area= Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3000),
                Constraint::Min(7000),
            ])
            .split(center_area_layout[1]);

        let title_area = inner_center_area[0];
        let main_area = inner_center_area[1];

        let title = Text::raw(name.clone());
        title.render(title_area, buf);
    }
}