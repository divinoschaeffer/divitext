use std::fmt;

#[derive(PartialEq, Debug)]
pub enum ErrorType {
    NONE,
    FileNotFound,
    FileExists,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ErrorType::FileNotFound => "File not found",
            ErrorType::FileExists => "File already exists",
            _ => ""
        };
        write!(f, "{}", message)
    }
}