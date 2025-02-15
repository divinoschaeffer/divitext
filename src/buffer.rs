use log::error;

#[derive(Debug)]
pub struct Buffer {
    pub content: Vec<u8>,
    pub point: Mark,
    pub mark_list: Vec<Mark>,
    pub file_name: Option<String>,
}

#[derive(Debug)]
pub struct Mark {
    pub name: String,
    pub buffer_position: u32,
}

impl Buffer {
    pub fn default() -> Buffer {
        Buffer {
            content: vec![],
            point: Mark::new(String::from("Point"), 0),
            mark_list: vec![],
            file_name: None,
        }
    }

    pub fn get_position_from_line_col(&self, line_index: usize, col: usize) -> usize {
        let mut position = 0;
        for (i, line) in self.content.split(|&c| c == b'\n').enumerate() {
            if i == line_index {
                return position + col.min(line.len());
            }
            position += line.len() + 1;
        }
        position
    }

    pub fn get_closest_column(&self, line_index: usize, col: usize) -> usize {
        if let Some(line) = self.content.split(|&c| c == b'\n').nth(line_index) {
            col.min(line.len())
        } else {
            0
        }
    }

    pub fn line_count(&self) -> usize {
        self.content.iter().filter(|&&c| c == b'\n').count() + 1
    }

    pub fn move_point_to(&mut self, line_offset: isize, col_offset: isize) -> bool {
        if let Some((line_index, col)) = self.get_point_line_and_column() {
            let new_line_index = (line_index as isize + line_offset).max(0) as usize;

            if new_line_index >= self.line_count() {
                return false
            }

            let new_col = (col as isize + col_offset).max(0) as usize;
            let new_col = self.get_closest_column(new_line_index, new_col);

            if new_col == col && new_col != 0 {
                return false
            }

            let new_position = self.get_position_from_line_col(new_line_index, new_col);
            self.point.buffer_position = new_position as u32;
            return true
        }
        false
    }

    pub fn get_point_line_and_column(&self) -> Option<(usize, usize)> {
        let mut current_pos = 0;
        for (line_index, line) in self.content.split(|&c| c == b'\n').enumerate() {
            if self.point.buffer_position <= (current_pos + line.len()) as u32 {
                return Some((line_index, (self.point.buffer_position as usize) - current_pos));
            }
            current_pos += line.len() + 1;
        }
        None
    }

    pub fn write_char(&mut self, c: char) -> Result<(), std::io::Error> {
        let position = self.point.buffer_position;
        let u8_char = u8::try_from(c).unwrap();

        if position >= self.content.len() as u32 {
            self.content.push(u8_char);
        } else {
            if c == ' ' || c == '\n' {
                self.content.insert(position as usize, u8_char)
            }
            self.content[position as usize] = u8_char;
        }
        Ok(())
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
