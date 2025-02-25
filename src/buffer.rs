use log::error;

#[derive(Debug)]
pub struct Buffer {
    pub content: String,
    pub point: Mark,
    pub mark_list: Vec<Mark>,
    pub file_name: Option<String>,
    pub buffer_type: BufferType,
}

#[derive(Debug)]
pub enum BufferType {
    FILE,
    OPTION
}

#[derive(Debug)]
pub struct Mark {
    pub name: String,
    pub buffer_position: u16,
}

impl Buffer {
    pub fn default() -> Buffer {
        Buffer {
            content: String::new(),
            point: Mark::new(String::from("Point"), 0),
            mark_list: vec![],
            file_name: None,
            buffer_type: BufferType::FILE,
        }
    }

    pub fn get_position_from_line_col(&self, line_index: u16, col: u16) -> u16 {
        let mut position = 0;

        for (i, line) in self.content.lines().enumerate() {
            let line_length = line.chars().count() as u16;

            if i == line_index as usize {
                return position + col.min(line_length);
            }

            position += line_length + 1;
        }

        position
    }


    pub fn get_closest_column(&self, line_index: u16, col: u16) -> u16 {
        if let Some(line) = self.content.split('\n').nth(line_index as usize) {
            col.min(line.chars().count() as u16 - 1)
        } else {
            0
        }
    }

    pub fn get_last_column(&self, line_index: u16) -> u16 {
        let lines: Vec<&str> = self.content.split('\n').collect();

        if let Some(line) = lines.get(line_index as usize) {
            let base_length = line.chars().count() as u16;
            if line_index < (lines.len() as u16 - 1) {
                base_length + 1
            } else {
                base_length
            }
        } else {
            0
        }
    }

    pub fn line_count(&self) -> u16 {
        self.content.split('\n').count() as u16
    }

    pub fn move_point_to(&mut self, line_offset: u16, col_offset: u16) {
        let new_position = self.get_position_from_line_col(line_offset, col_offset);
        self.point.buffer_position = new_position;
    }

    pub fn get_point_line_and_column(&self) -> Option<(u16, u16)> {
        let mut current_pos = 0;

        for (line_index, line) in self.content.split('\n').enumerate() {
            let char_count = line.chars().count() as u16;

            if self.point.buffer_position <= current_pos + char_count {
                return Some((line_index as u16, self.point.buffer_position - current_pos));
            }

            current_pos += char_count + 1;
        }

        None
    }

    pub fn write_char(&mut self, c: char) -> Result<(), std::io::Error> {
        let position = self.point.buffer_position as usize;
        if position >= self.content.chars().count() {
            self.content.push(c);
        } else {
            let mut new_content = String::with_capacity(self.content.len() + c.len_utf8());
            let mut chars = self.content.chars();

            for (i, ch) in chars.by_ref().enumerate() {
                if i == position {
                    new_content.push(c);
                }
                new_content.push(ch);
            }

            new_content.extend(chars);

            self.content = new_content;
        }
        Ok(())
    }

    pub fn get_last_visible_char_position(&self) -> Vec<Option<u16>> {
        self.content
            .split('\n')
            .map(|line| {
                let char_count = line.chars().count() as u16;
                if char_count == 0 { None } else { Some(char_count - 1) }
            })
            .collect()
    }

    pub fn remove_char(&mut self) -> Result<(), std::io::Error> {
        let position = self.point.buffer_position;
        self.content.remove(position as usize);
        Ok(())
    }

    pub fn get_buffer_part(&self, line1: u16, line2: u16) -> Result<String, std::io::Error> {
        let lines: Vec<&str> = self.content.split('\n').collect();

        if line1 as usize >= lines.len() {
            return Ok(String::new());
        }

        let end = (line2 as usize).min(lines.len());

        let buffer_part = lines[line1 as usize..end].join("\n");

        Ok(buffer_part)
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
