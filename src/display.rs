pub struct Display {
    pub width: u16,
    pub height: u16,
    pub first_line_visible: u16,
}

impl Display {
    pub fn new(width: u16, height: u16, first_line_visible: u16) -> Display {
        Display {
            width,
            height,
            first_line_visible,
        }
    }

    pub fn default() -> Display {
        let (width, height): (u16, u16) = crossterm::terminal::size().unwrap();
        Display {
            width,
            height,
            first_line_visible: 0,
        }
    }
}