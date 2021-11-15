use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum CustomError {
    NotFound(String),
    NotUnique(String)
}
impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let msg = match &self {
            Self::NotFound(param) => format!("Not found {}", param),
            Self::NotUnique(param) => format!("Not unique {}", param)
        };
        writeln!(f, "{}", msg)
    }
}

impl Error for CustomError {}
