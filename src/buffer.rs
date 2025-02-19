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
    pub buffer_position: u16,
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

    pub fn get_position_from_line_col(&self, line_index: u16, col: u16) -> u16 {
        let mut position = 0;
        for (i, line) in self.content.split(|&c| c == b'\n').enumerate() {
            if i == line_index as usize {
                return position + col.min(line.len() as u16);
            }
            position += (line.len() + 1) as u16;
        }
        position
    }

    pub fn get_closest_column(&self, line_index: u16, col: u16) -> u16 {
        if let Some(line) = self.content.split(|&c| c == b'\n').nth(line_index as usize) {
            col.min(line.len() as u16)
        } else {
            0
        }
    }

    pub fn get_last_column(&self, line_index: u16) -> u16 {
        if let Some(line) = self.content.split(|&c| c == b'\n').nth(line_index as usize) {
            line.len() as u16
        } else {
            0
        }
    }

    pub fn line_count(&self) -> u16 {
        (self.content.iter().filter(|&&c| c == b'\n').count() + 1) as u16
    }

    pub fn move_point_to(&mut self, line_offset: u16, col_offset: u16) {
        let new_position = self.get_position_from_line_col(line_offset, col_offset);
        self.point.buffer_position = new_position;
    }

    pub fn get_point_line_and_column(&self) -> Option<(u16, u16)> {
        let mut current_pos = 0;
        for (line_index, line) in self.content.split(|&c| c == b'\n').enumerate() {
            if self.point.buffer_position <= (current_pos + line.len()) as u16 {
                return Some((line_index as u16, self.point.buffer_position - current_pos as u16))
            }
            current_pos += line.len() + 1;
        }
        None
    }

    pub fn write_char(&mut self, c: char) -> Result<(), std::io::Error> {
        let position = self.point.buffer_position;
        let u8_char = u8::try_from(c).unwrap();

        if position >= self.content.len() as u16 {
            self.content.push(u8_char);
        } else {
            self.content.insert(position as usize, u8_char)
        }
        Ok(())
    }

    pub fn get_last_visible_char_position(&self) -> Vec<Option<u16>> {
        self.content
            .split(|&c| c == b'\n')
            .map(|line| if line.is_empty() { None } else { Some((line.len() - 1) as u16) })
            .collect()
    }

    pub fn remove_char(&mut self) -> Result<(), std::io::Error> {
        let position = self.point.buffer_position;
        self.content.remove(position as usize);
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
        buffer_position: u16,
    ) -> Mark {
        Mark {
            name,
            buffer_position,
        }
    }
}
