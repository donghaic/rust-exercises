use std::fmt;

#[derive(Debug)]
pub enum BizError {
    NotFound,
    ParseError,
}

impl std::error::Error for BizError {}

impl fmt::Display for BizError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BizError::NotFound => write!(f, "NotFound Error"),
            BizError::ParseError => write!(f, "Parse Error"),
        }
    }
}