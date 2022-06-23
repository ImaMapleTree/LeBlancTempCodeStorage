use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::process;
use crate::leblanc::rustblanc::lib::leblanc_colored::{Color, colorize_str};

pub struct LeblancBaseException {
    rust_error: Box<dyn Error>,
    message: String,
    critical: bool,
    error_code: i32
}

impl LeblancBaseException {
    pub fn from(rust_error: Box<dyn Error>, message: &String, critical: bool, error_code: i32) -> LeblancBaseException {
        return LeblancBaseException {
            rust_error,
            message: message.to_string(),
            critical,
            error_code
        }
    }

    pub fn new(message: &String, critical: bool, error_code: i32) -> LeblancBaseException {
        return LeblancBaseException {
            rust_error: Box::new(EmptyError::new()),
            message: message.to_string(),
            critical,
            error_code

        }
    }

    pub fn handle(&self) -> Result<String, String> {
        match self.critical {
            true => {
                eprintln!("{} {}", colorize_str("[Critical]", Color::Red), self.message);
                process::exit(self.error_code);
            }
            false => eprintln!("{} {}", "[Warning]", self.message)
        }
        Ok(String::from(""))
    }

    pub fn throw(&self) {
        eprintln!("{}", self.message);
        process::exit(self.error_code);
    }

    pub fn output(&self) {
        eprintln!("{}", self.message);
    }

    pub fn code(&self) -> u32 { self.error_code as u32 }
}

#[derive(Debug)]
struct EmptyError {

}

impl Display for EmptyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Error for EmptyError {}

impl EmptyError {
    fn new() -> EmptyError {
        EmptyError {}
    }
}