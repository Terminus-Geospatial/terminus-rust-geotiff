
use std::fmt;

#[derive(Debug)]
pub enum ErrorCode {
    NotFound,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCode::NotFound   => write!(f, "NotFound"),
       }
    }
}