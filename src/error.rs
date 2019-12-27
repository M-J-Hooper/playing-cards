use std::error::Error;
use std::fmt;
use std::str;

#[derive(Debug, Clone)]
pub struct ParseError {
    input: String,
    message: String,
}

impl ParseError {
    pub fn new(input: &str, message: &str) -> ParseError {
        ParseError { 
            input: input.to_owned(), 
            message: message.to_owned()
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse '{}': {}", self.input, self.message)
    }
}

impl Error for ParseError {}