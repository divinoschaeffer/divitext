use std::error::Error;

pub struct Buffer {
    pub content: Vec<u8>,
    pub point: Mark,
    pub num_lines: u32,
    pub mark_list: Vec<Mark>,
    pub file_name: Option<String>,
}

pub struct Mark {
    pub name: String,
    pub buffer_position: u32,
}

impl Buffer {
    pub fn default() -> Buffer {
        Buffer {
            content: vec![],
            point: Mark::new(String::from("Point"), 0),
            num_lines: 0,
            mark_list: vec![],
            file_name: None,
        }
    }

    pub fn handle_point_movement(&mut self, movement: MarkerMovement, number: u16) {
        match movement {
            MarkerMovement::Left => {
                if self.point.buffer_position > (number as u32) {
                    self.point.buffer_position -= (number as u32);
                }
                // TODO: error generation
            },
            MarkerMovement::Right => {
                if self.point.buffer_position < (number as u32) {
                    self.point.buffer_position += (number as u32);
                }
                // TODO: error generation
            },
        };
    }
}

pub enum MarkerMovement {
    Left,
    Right,
}

impl Mark {
    pub fn new(
        name: String,
        buffer_position: u32,
    ) -> Mark {
        Mark {
            name,
            buffer_position,
        }
    }
}
