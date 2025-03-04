use tui_textarea::TextArea;

#[derive(Debug)]
pub struct Buffer<'a>{
    pub input: TextArea<'a>,
    pub filename: Option<String>,
}

impl<'a> Buffer<'a> {
    pub fn new(input: TextArea<'a>, filename: Option<String>) -> Buffer<'a> {
        Buffer { input, filename }
    }
}